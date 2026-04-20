//! Candlestick chart — OHLC bars with optional volume strip below.
//!
//! Red when close < open, green when close >= open. A thin wick spans the
//! high/low, the body spans open/close.

use iced::{
    Element, Length, Point, Rectangle, Renderer, Theme,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke},
};

use crate::theme::{with_alpha, AppTheme};

use super::{
    axis::{draw_x_labels, draw_y_axis, fill_background, PlotArea, PlotPadding},
    line::format_tick,
    scale::{padded_domain, Band, Linear},
};

#[derive(Debug, Clone, Copy)]
pub struct Candle {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
}

impl Candle {
    pub fn new(open: f32, high: f32, low: f32, close: f32, volume: f32) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
        }
    }
}

pub struct CandlestickChart {
    pub theme: AppTheme,
    pub candles: Vec<Candle>,
    pub x_labels: Vec<String>,
    pub show_volume: bool,
}

impl CandlestickChart {
    pub fn new(theme: &AppTheme, candles: Vec<Candle>, x_labels: Vec<String>) -> Self {
        Self {
            theme: *theme,
            candles,
            x_labels,
            show_volume: true,
        }
    }

    #[allow(dead_code)]
    pub fn show_volume(mut self, show: bool) -> Self {
        self.show_volume = show;
        self
    }
}

pub fn candlestick_chart<'a, Message: 'a>(
    theme: &AppTheme,
    candles: Vec<Candle>,
    x_labels: Vec<String>,
) -> Element<'a, Message> {
    Canvas::new(CandlestickChart::new(theme, candles, x_labels))
        .width(Length::Fill)
        .height(Length::Fixed(320.0))
        .into()
}

impl<Message> canvas::Program<Message> for CandlestickChart {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _iced_theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        fill_background(&mut frame, bounds, &self.theme);

        if self.candles.is_empty() {
            return vec![frame.into_geometry()];
        }

        let padding = PlotPadding::DEFAULT;
        let volume_h = if self.show_volume {
            (bounds.height * 0.22).clamp(40.0, 80.0)
        } else {
            0.0
        };
        let volume_gap = if self.show_volume { 8.0 } else { 0.0 };

        let price_area = PlotArea {
            left: padding.left,
            top: padding.top,
            right: bounds.width - padding.right,
            bottom: bounds.height - padding.bottom - volume_h - volume_gap,
        };
        let volume_area = PlotArea {
            left: padding.left,
            top: price_area.bottom + volume_gap,
            right: bounds.width - padding.right,
            bottom: bounds.height - padding.bottom,
        };

        if price_area.width() <= 0.0 || price_area.height() <= 0.0 {
            return vec![frame.into_geometry()];
        }

        let (y_min, y_max) = padded_domain(
            self.candles
                .iter()
                .flat_map(|c| [c.low, c.high].into_iter()),
        );
        let price_scale = Linear::new((y_min, y_max), price_area.y_range());
        let band = Band::new(self.candles.len(), price_area.x_range()).padding(0.25);

        draw_y_axis(&mut frame, &self.theme, price_area, price_scale, 5, format_tick);

        // X labels appear below the volume strip (or below price if no volume).
        let label_area = if self.show_volume { volume_area } else { price_area };
        let positions: Vec<(f32, String)> = self
            .x_labels
            .iter()
            .enumerate()
            .map(|(i, l)| {
                let t = if self.x_labels.len() > 1 {
                    i as f32 / (self.x_labels.len() - 1) as f32
                } else {
                    0.5
                };
                (label_area.left + t * label_area.width(), l.clone())
            })
            .collect();
        draw_x_labels(&mut frame, &self.theme, label_area, &positions);

        let up = self.theme.success;
        let down = self.theme.danger;
        let body_w = band.bandwidth().max(1.0);

        for (i, c) in self.candles.iter().enumerate() {
            let center_x = band.center(i);
            let bullish = c.close >= c.open;
            let color = if bullish { up } else { down };

            // Wick.
            let wick = Path::line(
                Point::new(center_x, price_scale.to_pixel(c.high)),
                Point::new(center_x, price_scale.to_pixel(c.low)),
            );
            frame.stroke(
                &wick,
                Stroke::default().with_color(color).with_width(1.0),
            );

            // Body.
            let y_open = price_scale.to_pixel(c.open);
            let y_close = price_scale.to_pixel(c.close);
            let top = y_open.min(y_close);
            let h = (y_open - y_close).abs().max(1.0);
            let body = Path::rectangle(
                Point::new(center_x - body_w / 2.0, top),
                iced::Size::new(body_w, h),
            );
            frame.fill(&body, canvas::Fill::from(color));
        }

        // Volume strip.
        if self.show_volume && volume_area.height() > 0.0 {
            let v_max = self
                .candles
                .iter()
                .map(|c| c.volume)
                .fold(0.0_f32, f32::max);
            if v_max > 0.0 {
                let v_scale = Linear::new((0.0, v_max), volume_area.y_range());
                let baseline = v_scale.to_pixel(0.0);
                for (i, c) in self.candles.iter().enumerate() {
                    let bullish = c.close >= c.open;
                    let color = with_alpha(
                        if bullish { up } else { down },
                        0.55,
                    );
                    let top = v_scale.to_pixel(c.volume);
                    let h = (baseline - top).max(1.0);
                    let x = band.left(i);
                    let rect = Path::rectangle(
                        Point::new(x, top),
                        iced::Size::new(body_w, h),
                    );
                    frame.fill(&rect, canvas::Fill::from(color));
                }
            }
        }

        vec![frame.into_geometry()]
    }
}
