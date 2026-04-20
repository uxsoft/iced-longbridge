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
    interactive::{ChartHover, ChartHoverEntry, ChartState, ChartWidget},
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

struct CandleLayout {
    price_area: PlotArea,
    volume_area: PlotArea,
    band: Band,
    price_scale: Linear,
    body_w: f32,
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

    fn layout(&self, bounds: Rectangle) -> Option<CandleLayout> {
        if self.candles.is_empty() {
            return None;
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
            return None;
        }

        let (y_min, y_max) = padded_domain(
            self.candles
                .iter()
                .flat_map(|c| [c.low, c.high].into_iter()),
        );
        let price_scale = Linear::new((y_min, y_max), price_area.y_range());
        let band = Band::new(self.candles.len(), price_area.x_range()).padding(0.25);
        let body_w = band.bandwidth().max(1.0);

        Some(CandleLayout {
            price_area,
            volume_area,
            band,
            price_scale,
            body_w,
        })
    }

    fn hit_test(&self, bounds: Rectangle, cursor: Point) -> Option<usize> {
        let layout = self.layout(bounds)?;
        let local = Point::new(cursor.x - bounds.x, cursor.y - bounds.y);

        // Must be within the vertical span of either the price or (if shown)
        // volume area — anywhere inside the chart's active columns is fair
        // game.
        let within_y = (local.y >= layout.price_area.top
            && local.y <= layout.price_area.bottom)
            || (self.show_volume
                && local.y >= layout.volume_area.top
                && local.y <= layout.volume_area.bottom);
        if !within_y {
            return None;
        }

        let bw = layout.band.bandwidth();
        if bw <= 0.0 {
            return None;
        }

        // Find nearest band center to cursor.x.
        let mut best: Option<(f32, usize)> = None;
        for i in 0..self.candles.len() {
            let d = (layout.band.center(i) - local.x).abs();
            if best.is_none_or(|(b, _)| d < b) {
                best = Some((d, i));
            }
        }
        best.and_then(|(d, i)| {
            if d <= bw / 2.0 + 4.0 {
                Some(i)
            } else {
                None
            }
        })
    }
}

pub fn candlestick_chart<'a, Message: 'a>(
    theme: &AppTheme,
    candles: Vec<Candle>,
    x_labels: Vec<String>,
) -> Element<'a, Message> {
    let canvas = Canvas::new(CandlestickChart::new(theme, candles, x_labels))
        .width(Length::Fill)
        .height(Length::Fixed(320.0));
    ChartWidget::new(canvas, theme).into()
}

impl<Message> canvas::Program<Message> for CandlestickChart {
    type State = ChartState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        if !matches!(event, canvas::Event::Mouse(_)) {
            return None;
        }

        let new_hover = cursor.position().and_then(|pos| {
            if !bounds.contains(pos) {
                return None;
            }
            let idx = self.hit_test(bounds, pos)?;
            let layout = self.layout(bounds)?;
            let c = self.candles[idx];
            let bullish = c.close >= c.open;
            let color = if bullish { self.theme.success } else { self.theme.danger };

            let title = self
                .x_labels
                .get(idx)
                .cloned()
                .unwrap_or_else(|| format!("#{idx}"));

            let anchor_y = layout.price_scale.to_pixel(c.high).min(layout.price_area.top);
            let anchor = Point::new(layout.band.center(idx), anchor_y.max(layout.price_area.top));

            Some(ChartHover {
                title,
                entries: vec![
                    ChartHoverEntry::new("Open", format_tick(c.open), color),
                    ChartHoverEntry::new("High", format_tick(c.high), color),
                    ChartHoverEntry::new("Low", format_tick(c.low), color),
                    ChartHoverEntry::new("Close", format_tick(c.close), color),
                    ChartHoverEntry::new("Vol", format_tick(c.volume), color),
                ],
                anchor,
            })
        });

        if state.hover != new_hover {
            state.hover = new_hover;
            return Some(canvas::Action::request_redraw());
        }
        None
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _iced_theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        fill_background(&mut frame, bounds, &self.theme);

        if self.candles.is_empty() {
            return vec![frame.into_geometry()];
        }

        let Some(layout) = self.layout(bounds) else {
            return vec![frame.into_geometry()];
        };
        let CandleLayout {
            price_area,
            volume_area,
            band,
            price_scale,
            body_w,
        } = layout;

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

        // Re-run hit-test so candle highlight stays in lock-step with tooltip.
        let hovered = cursor
            .position()
            .and_then(|p| self.hit_test(bounds, p))
            .or_else(|| {
                state.hover.as_ref().and_then(|h| {
                    self.x_labels.iter().position(|l| l == &h.title)
                })
            });

        let up = self.theme.success;
        let down = self.theme.danger;

        for (i, c) in self.candles.iter().enumerate() {
            let center_x = band.center(i);
            let bullish = c.close >= c.open;
            let base_color = if bullish { up } else { down };
            let is_hovered = hovered == Some(i);
            let color = if hovered.is_some() && !is_hovered {
                with_alpha(base_color, 0.35)
            } else {
                base_color
            };

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

            if is_hovered {
                frame.stroke(
                    &body,
                    Stroke::default()
                        .with_color(self.theme.foreground)
                        .with_width(1.5),
                );
            }
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
                    let base = if bullish { up } else { down };
                    let is_hovered = hovered == Some(i);
                    let alpha = if hovered.is_some() && !is_hovered { 0.25 } else { 0.55 };
                    let color = with_alpha(base, alpha);
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

        // Vertical crosshair spanning both panes for the hovered candle.
        if let Some(i) = hovered {
            let px = band.center(i);
            let cross_bottom = if self.show_volume {
                volume_area.bottom
            } else {
                price_area.bottom
            };
            let crosshair = Path::line(
                Point::new(px, price_area.top),
                Point::new(px, cross_bottom),
            );
            frame.stroke(
                &crosshair,
                Stroke::default()
                    .with_color(with_alpha(self.theme.foreground, 0.35))
                    .with_width(1.0),
            );
        }

        vec![frame.into_geometry()]
    }
}

