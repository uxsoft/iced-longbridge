//! Area chart — filled region below a line series.

use iced::{
    Color, Element, Length, Point, Rectangle, Renderer, Theme,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke},
};

use crate::theme::{AppTheme, with_alpha};

use super::{
    axis::{draw_x_labels, draw_y_axis, fill_background, PlotArea, PlotPadding},
    interactive::{ChartHover, ChartHoverEntry, ChartState, ChartWidget},
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

    fn series_color(&self, index: usize) -> Color {
        self.series[index]
            .2
            .unwrap_or(self.theme.chart[index % self.theme.chart.len()])
    }

    /// Snap to the nearest x-index (by pixel distance on x). Returns the
    /// sample index and its pixel x. All series are assumed to share the
    /// same x-values (the demo does). Requires the cursor to be within the
    /// plot area.
    fn hit_test(&self, bounds: Rectangle, cursor: Point) -> Option<(usize, f32)> {
        let area = PlotArea::from_bounds(bounds, PlotPadding::DEFAULT);
        if area.width() <= 0.0 || area.height() <= 0.0 || self.series.is_empty() {
            return None;
        }

        let (x_min, x_max) = padded_domain(
            self.series
                .iter()
                .flat_map(|(_, p, _)| p.iter().map(|(x, _)| *x)),
        );
        let x_scale = Linear::new((x_min, x_max), area.x_range());

        let local = Point::new(cursor.x - bounds.x, cursor.y - bounds.y);
        if local.x < area.left
            || local.x > area.right
            || local.y < area.top
            || local.y > area.bottom
        {
            return None;
        }

        let ref_series = self
            .series
            .iter()
            .max_by_key(|(_, p, _)| p.len())
            .map(|(_, p, _)| p)?;
        if ref_series.is_empty() {
            return None;
        }

        let (best_idx, best_px) = ref_series
            .iter()
            .enumerate()
            .map(|(i, (x, _))| (i, x_scale.to_pixel(*x)))
            .min_by(|a, b| {
                (a.1 - local.x)
                    .abs()
                    .partial_cmp(&(b.1 - local.x).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;

        Some((best_idx, best_px))
    }
}

#[allow(dead_code)]
pub fn area_chart<'a, Message: 'a>(
    theme: &AppTheme,
    series: Vec<(String, Vec<(f32, f32)>)>,
) -> Element<'a, Message> {
    let canvas = Canvas::new(AreaChart::new(theme, series))
        .width(Length::Fill)
        .height(Length::Fixed(260.0));
    ChartWidget::new(canvas, theme).into()
}

impl<Message> canvas::Program<Message> for AreaChart {
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
            let (idx, px) = self.hit_test(bounds, pos)?;

            let area = PlotArea::from_bounds(bounds, PlotPadding::DEFAULT);
            let mut all_y: Vec<f32> = self
                .series
                .iter()
                .flat_map(|(_, p, _)| p.iter().map(|(_, y)| *y))
                .collect();
            all_y.push(0.0);
            let (y_min_raw, y_max) = padded_domain(all_y.into_iter());
            let y_min = y_min_raw.min(0.0);
            let y_scale = Linear::new((y_min, y_max), area.y_range());

            let mut entries = Vec::with_capacity(self.series.len());
            let mut anchor_y = area.top;
            let mut min_py = f32::INFINITY;
            for (si, (name, points, _)) in self.series.iter().enumerate() {
                let Some((_, y)) = points.get(idx) else {
                    continue;
                };
                let py = y_scale.to_pixel(*y);
                if py < min_py {
                    min_py = py;
                    anchor_y = py;
                }
                entries.push(ChartHoverEntry::new(
                    name.clone(),
                    format_tick(*y),
                    self.series_color(si),
                ));
            }
            if entries.is_empty() {
                return None;
            }

            let title = self
                .x_labels
                .as_ref()
                .and_then(|labels| labels.get(idx).cloned())
                .unwrap_or_else(|| {
                    let x = self
                        .series
                        .iter()
                        .find_map(|(_, p, _)| p.get(idx).map(|(x, _)| *x))
                        .unwrap_or(0.0);
                    format_tick(x)
                });

            Some(ChartHover {
                title,
                entries,
                anchor: Point::new(px, anchor_y),
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

        for (i, (_, points, _)) in self.series.iter().enumerate() {
            if points.len() < 2 {
                continue;
            }
            let color = self.series_color(i);

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

        // Hovered index: re-run from cursor so it stays in lock-step with
        // tooltip; fall back to state.hover while the overlay is easing out.
        let hovered_idx = cursor
            .position()
            .and_then(|p| self.hit_test(bounds, p))
            .map(|(idx, _)| idx)
            .or_else(|| {
                state.hover.as_ref().and_then(|h| {
                    self.x_labels.as_ref().and_then(|labels| {
                        labels.iter().position(|l| l == &h.title)
                    })
                })
            });

        if let Some(idx) = hovered_idx {
            let px = self
                .series
                .iter()
                .find_map(|(_, p, _)| p.get(idx).map(|(x, _)| x_scale.to_pixel(*x)));
            if let Some(px) = px {
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

                for (si, (_, points, _)) in self.series.iter().enumerate() {
                    if let Some((_, y)) = points.get(idx) {
                        let py = y_scale.to_pixel(*y);
                        let color = self.series_color(si);
                        let halo = Path::circle(Point::new(px, py), 6.0);
                        frame.fill(&halo, canvas::Fill::from(with_alpha(color, 0.28)));
                        let dot = Path::circle(Point::new(px, py), 4.0);
                        frame.fill(&dot, canvas::Fill::from(color));
                        frame.stroke(
                            &dot,
                            Stroke::default()
                                .with_color(self.theme.foreground)
                                .with_width(1.2),
                        );
                    }
                }
            }
        }

        vec![frame.into_geometry()]
    }
}
