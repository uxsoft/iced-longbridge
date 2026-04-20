//! Line chart — one or many series of `(x, y)` points.

use iced::{
    Color, Element, Length, Pixels, Point, Rectangle, Renderer, Theme,
    advanced::text::Alignment as TextAlign,
    alignment::Vertical,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text},
};

use crate::theme::{AppTheme, with_alpha};

use super::{
    axis::{draw_x_labels, draw_y_axis, fill_background, PlotArea, PlotPadding},
    interactive::{ChartHover, ChartHoverEntry, ChartState, ChartWidget},
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

    fn series_color(&self, index: usize) -> Color {
        self.series[index]
            .color
            .unwrap_or(self.theme.chart[index % self.theme.chart.len()])
    }

    /// Find the nearest `(series, point)` to the cursor, within 24px.
    fn hit_test(
        &self,
        bounds: Rectangle,
        cursor: Point,
    ) -> Option<(usize, usize, Point)> {
        let area = PlotArea::from_bounds(bounds, PlotPadding::DEFAULT);
        if area.width() <= 0.0 || area.height() <= 0.0 {
            return None;
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

        let local = Point::new(cursor.x - bounds.x, cursor.y - bounds.y);
        let mut best: Option<(f32, usize, usize, Point)> = None;

        for (si, s) in self.series.iter().enumerate() {
            for (pi, (x, y)) in s.points.iter().enumerate() {
                let px = x_scale.to_pixel(*x);
                let py = y_scale.to_pixel(*y);
                let dx = px - local.x;
                let dy = py - local.y;
                let d2 = dx * dx + dy * dy;
                if best.is_none_or(|(b, _, _, _)| d2 < b) {
                    best = Some((d2, si, pi, Point::new(px, py)));
                }
            }
        }

        best.and_then(|(d2, si, pi, p)| {
            if d2.sqrt() <= 24.0 {
                Some((si, pi, p))
            } else {
                None
            }
        })
    }
}

#[allow(dead_code)]
pub fn line_chart<'a, Message: 'a>(
    theme: &AppTheme,
    series: Vec<Series>,
) -> Element<'a, Message> {
    let canvas = Canvas::new(LineChart::new(theme, series))
        .width(Length::Fill)
        .height(Length::Fixed(260.0));
    ChartWidget::new(canvas, theme).into()
}

impl<Message> canvas::Program<Message> for LineChart {
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
            self.hit_test(bounds, pos).map(|(si, pi, anchor)| {
                let s = &self.series[si];
                let (x, y) = s.points[pi];
                let title = self
                    .x_labels
                    .as_ref()
                    .and_then(|labels| {
                        if labels.len() == s.points.len() {
                            labels.get(pi).cloned()
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| format_tick(x));
                ChartHover {
                    title,
                    entries: vec![ChartHoverEntry::new(
                        s.name.clone(),
                        format_tick(y),
                        self.series_color(si),
                    )],
                    anchor,
                }
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
                    let x = area.left + t * area.width();
                    (x, l.clone())
                })
                .collect();
            draw_x_labels(&mut frame, &self.theme, area, &positions);
        }

        // Recover hovered (series, point) — prefer fresh cursor hit so the
        // dot highlight stays in lock-step with the tooltip.
        let hovered = cursor
            .position()
            .and_then(|p| self.hit_test(bounds, p))
            .map(|(si, pi, _)| (si, pi))
            .or_else(|| {
                state.hover.as_ref().and_then(|h| {
                    let entry = h.entries.first()?;
                    let si = self.series.iter().position(|s| s.name == entry.label)?;
                    let y: f32 = entry.value.parse().ok()?;
                    let pi = self.series[si]
                        .points
                        .iter()
                        .position(|(_, py)| (py - y).abs() < 1e-3)?;
                    Some((si, pi))
                })
            });

        for (i, series) in self.series.iter().enumerate() {
            let color = self.series_color(i);
            let is_dim = hovered.is_some_and(|(hi, _)| hi != i);
            let line_color = if is_dim { with_alpha(color, 0.35) } else { color };

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
                        .with_color(line_color)
                        .with_width(2.0)
                        .with_line_cap(canvas::LineCap::Round),
                );
            }

            if self.show_points {
                for (x, y) in &series.points {
                    let px = x_scale.to_pixel(*x);
                    let py = y_scale.to_pixel(*y);
                    let dot = Path::circle(Point::new(px, py), 3.0);
                    frame.fill(&dot, canvas::Fill::from(line_color));
                }
            }
        }

        // Highlight the hovered point with a crosshair + larger dot.
        if let Some((si, pi)) = hovered {
            let (x, y) = self.series[si].points[pi];
            let px = x_scale.to_pixel(x);
            let py = y_scale.to_pixel(y);
            let color = self.series_color(si);

            let crosshair = Path::line(
                Point::new(px, area.top),
                Point::new(px, area.bottom),
            );
            frame.stroke(
                &crosshair,
                Stroke::default()
                    .with_color(with_alpha(self.theme.foreground, 0.35))
                    .with_width(1.0),
            );

            let halo = Path::circle(Point::new(px, py), 6.0);
            frame.fill(
                &halo,
                canvas::Fill::from(with_alpha(color, 0.28)),
            );
            let dot = Path::circle(Point::new(px, py), 4.0);
            frame.fill(&dot, canvas::Fill::from(color));
            frame.stroke(
                &dot,
                Stroke::default()
                    .with_color(self.theme.foreground)
                    .with_width(1.2),
            );
        }

        // Legend (top-right).
        if self.series.len() > 1 {
            let mut y = area.top + 4.0;
            for (i, s) in self.series.iter().enumerate() {
                let color = self.series_color(i);
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
