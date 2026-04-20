//! Line chart — one or many series of `(x, y)` points.

use iced::{
    Color, Element, Length, Pixels, Point, Rectangle, Renderer, Theme,
    advanced::text::Alignment as TextAlign,
    alignment::Vertical,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text},
};

use crate::theme::AppTheme;

use super::{
    axis::{draw_x_labels, draw_y_axis, fill_background, PlotArea, PlotPadding},
    scale::{padded_domain, Linear},
};

#[derive(Debug, Clone)]
pub struct Series {
    pub name: String,
    pub points: Vec<(f32, f32)>,
    pub color: Option<Color>,
}

impl Series {
    pub fn new(name: impl Into<String>, points: Vec<(f32, f32)>) -> Self {
        Self {
            name: name.into(),
            points,
            color: None,
        }
    }

    #[allow(dead_code)]
    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }
}

pub struct LineChart {
    pub theme: AppTheme,
    pub series: Vec<Series>,
    pub x_labels: Option<Vec<String>>,
    pub show_points: bool,
}

impl LineChart {
    pub fn new(theme: &AppTheme, series: Vec<Series>) -> Self {
        Self {
            theme: *theme,
            series,
            x_labels: None,
            show_points: true,
        }
    }

    pub fn x_labels(mut self, labels: Vec<String>) -> Self {
        self.x_labels = Some(labels);
        self
    }

    #[allow(dead_code)]
    pub fn show_points(mut self, show: bool) -> Self {
        self.show_points = show;
        self
    }
}

#[allow(dead_code)]
pub fn line_chart<'a, Message: 'a>(
    theme: &AppTheme,
    series: Vec<Series>,
) -> Element<'a, Message> {
    Canvas::new(LineChart::new(theme, series))
        .width(Length::Fill)
        .height(Length::Fixed(260.0))
        .into()
}

impl<Message> canvas::Program<Message> for LineChart {
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
                .flat_map(|s| s.points.iter().map(|(x, _)| *x)),
        );
        let (y_min, y_max) = padded_domain(
            self.series
                .iter()
                .flat_map(|s| s.points.iter().map(|(_, y)| *y)),
        );

        let x_scale = Linear::new((x_min, x_max), area.x_range());
        let y_scale = Linear::new((y_min, y_max), area.y_range());

        draw_y_axis(&mut frame, &self.theme, area, y_scale, 5, |v| {
            format_tick(v)
        });

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
                    let x = area.left + t * area.width();
                    (x, l.clone())
                })
                .collect();
            draw_x_labels(&mut frame, &self.theme, area, &positions);
        }

        for (i, series) in self.series.iter().enumerate() {
            let color = series
                .color
                .unwrap_or(self.theme.chart[i % self.theme.chart.len()]);

            if series.points.len() >= 2 {
                let path = Path::new(|b| {
                    for (j, (x, y)) in series.points.iter().enumerate() {
                        let px = x_scale.to_pixel(*x);
                        let py = y_scale.to_pixel(*y);
                        if j == 0 {
                            b.move_to(Point::new(px, py));
                        } else {
                            b.line_to(Point::new(px, py));
                        }
                    }
                });
                frame.stroke(
                    &path,
                    Stroke::default()
                        .with_color(color)
                        .with_width(2.0)
                        .with_line_cap(canvas::LineCap::Round),
                );
            }

            if self.show_points {
                for (x, y) in &series.points {
                    let px = x_scale.to_pixel(*x);
                    let py = y_scale.to_pixel(*y);
                    let dot = Path::circle(Point::new(px, py), 3.0);
                    frame.fill(&dot, canvas::Fill::from(color));
                }
            }
        }

        // Legend (top-right).
        if self.series.len() > 1 {
            let mut y = area.top + 4.0;
            for (i, s) in self.series.iter().enumerate() {
                let color = s
                    .color
                    .unwrap_or(self.theme.chart[i % self.theme.chart.len()]);
                let swatch = Path::rectangle(
                    Point::new(area.right - 110.0, y),
                    iced::Size::new(10.0, 10.0),
                );
                frame.fill(&swatch, canvas::Fill::from(color));
                frame.fill_text(Text {
                    content: s.name.clone(),
                    position: Point::new(area.right - 96.0, y + 5.0),
                    color: self.theme.foreground,
                    size: Pixels(10.0),
                    align_x: TextAlign::Left,
                    align_y: Vertical::Center,
                    ..Default::default()
                });
                y += 14.0;
            }
        }

        vec![frame.into_geometry()]
    }
}

pub fn format_tick(v: f32) -> String {
    let abs = v.abs();
    if abs >= 100.0 {
        format!("{v:.0}")
    } else if abs >= 10.0 {
        format!("{v:.1}")
    } else {
        format!("{v:.2}")
    }
}
