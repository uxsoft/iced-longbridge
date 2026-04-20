//! Area chart — filled region below a line series.

use iced::{
    Color, Element, Length, Point, Rectangle, Renderer, Theme,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke},
};

use crate::theme::{AppTheme, with_alpha};

use super::{
    axis::{draw_x_labels, draw_y_axis, fill_background, PlotArea, PlotPadding},
    line::format_tick,
    scale::{padded_domain, Linear},
};

type AreaSeries = (String, Vec<(f32, f32)>, Option<Color>);

pub struct AreaChart {
    pub theme: AppTheme,
    pub series: Vec<AreaSeries>,
    pub x_labels: Option<Vec<String>>,
}

impl AreaChart {
    pub fn new(theme: &AppTheme, series: Vec<(String, Vec<(f32, f32)>)>) -> Self {
        Self {
            theme: *theme,
            series: series.into_iter().map(|(n, p)| (n, p, None)).collect(),
            x_labels: None,
        }
    }

    pub fn x_labels(mut self, labels: Vec<String>) -> Self {
        self.x_labels = Some(labels);
        self
    }
}

#[allow(dead_code)]
pub fn area_chart<'a, Message: 'a>(
    theme: &AppTheme,
    series: Vec<(String, Vec<(f32, f32)>)>,
) -> Element<'a, Message> {
    Canvas::new(AreaChart::new(theme, series))
        .width(Length::Fill)
        .height(Length::Fixed(260.0))
        .into()
}

impl<Message> canvas::Program<Message> for AreaChart {
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

        let area = PlotArea::from_bounds(bounds, PlotPadding::DEFAULT);
        if area.width() <= 0.0 || area.height() <= 0.0 {
            return vec![frame.into_geometry()];
        }

        let (x_min, x_max) = padded_domain(
            self.series
                .iter()
                .flat_map(|(_, p, _)| p.iter().map(|(x, _)| *x)),
        );
        let mut all_y: Vec<f32> = self
            .series
            .iter()
            .flat_map(|(_, p, _)| p.iter().map(|(_, y)| *y))
            .collect();
        all_y.push(0.0);
        let (y_min_raw, y_max) = padded_domain(all_y.into_iter());
        let y_min = y_min_raw.min(0.0);

        let x_scale = Linear::new((x_min, x_max), area.x_range());
        let y_scale = Linear::new((y_min, y_max), area.y_range());

        draw_y_axis(&mut frame, &self.theme, area, y_scale, 5, format_tick);

        if let Some(labels) = &self.x_labels
            && !labels.is_empty()
        {
            let positions: Vec<(f32, String)> = labels
                .iter()
                .enumerate()
                .map(|(i, l)| {
                    let t = if labels.len() > 1 {
                        i as f32 / (labels.len() - 1) as f32
                    } else {
                        0.5
                    };
                    (area.left + t * area.width(), l.clone())
                })
                .collect();
            draw_x_labels(&mut frame, &self.theme, area, &positions);
        }

        let baseline_y = y_scale.to_pixel(0.0);

        for (i, (_, points, override_color)) in self.series.iter().enumerate() {
            if points.len() < 2 {
                continue;
            }
            let color =
                override_color.unwrap_or(self.theme.chart[i % self.theme.chart.len()]);

            let fill_path = Path::new(|b| {
                let first = points[0];
                b.move_to(Point::new(x_scale.to_pixel(first.0), baseline_y));
                for (x, y) in points.iter() {
                    b.line_to(Point::new(x_scale.to_pixel(*x), y_scale.to_pixel(*y)));
                }
                let last = points[points.len() - 1];
                b.line_to(Point::new(x_scale.to_pixel(last.0), baseline_y));
                b.close();
            });
            frame.fill(&fill_path, canvas::Fill::from(with_alpha(color, 0.25)));

            let line_path = Path::new(|b| {
                for (j, (x, y)) in points.iter().enumerate() {
                    let p = Point::new(x_scale.to_pixel(*x), y_scale.to_pixel(*y));
                    if j == 0 {
                        b.move_to(p);
                    } else {
                        b.line_to(p);
                    }
                }
            });
            frame.stroke(
                &line_path,
                Stroke::default().with_color(color).with_width(2.0),
            );
        }

        vec![frame.into_geometry()]
    }
}
