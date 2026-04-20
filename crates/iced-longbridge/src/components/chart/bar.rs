//! Bar chart — grouped bars on a band x-scale.

use iced::{
    Color, Element, Length, Point, Rectangle, Renderer, Theme,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke},
};

use crate::theme::{AppTheme, with_alpha};

use super::{
    axis::{PlotArea, PlotPadding, draw_x_labels, draw_y_axis, fill_background},
    interactive::{ChartHover, ChartHoverEntry, ChartState, ChartWidget},
    line::format_tick,
    scale::{Band, Linear, padded_domain},
};

pub struct BarChart {
    pub theme: AppTheme,
    pub categories: Vec<String>,
    pub groups: Vec<(String, Vec<f32>, Option<Color>)>,
}

impl BarChart {
    pub fn new(
        theme: &AppTheme,
        categories: Vec<String>,
        groups: Vec<(String, Vec<f32>)>,
    ) -> Self {
        Self {
            theme: *theme,
            categories,
            groups: groups
                .into_iter()
                .map(|(n, v)| (n, v, None))
                .collect(),
        }
    }

    fn group_color(&self, group_index: usize) -> Color {
        self.groups[group_index]
            .2
            .unwrap_or(self.theme.chart[group_index % self.theme.chart.len()])
    }

    fn hit_test(&self, bounds: Rectangle, cursor: Point) -> Option<(usize, usize, Rectangle)> {
        let area = PlotArea::from_bounds(bounds, PlotPadding::DEFAULT);
        if area.width() <= 0.0 || area.height() <= 0.0 || self.categories.is_empty() {
            return None;
        }

        let mut all_values: Vec<f32> = self
            .groups
            .iter()
            .flat_map(|(_, v, _)| v.iter().copied())
            .collect();
        all_values.push(0.0);
        let (y_min_raw, y_max) = padded_domain(all_values.into_iter());
        let y_min = y_min_raw.min(0.0);

        let x_band = Band::new(self.categories.len(), area.x_range()).padding(0.25);
        let y_scale = Linear::new((y_min, y_max), area.y_range());

        let group_count = self.groups.len().max(1);
        let band_w = x_band.bandwidth();
        let bar_w = band_w / group_count as f32;
        let zero_y = y_scale.to_pixel(0.0);

        // Convert cursor to canvas-local.
        let local = Point::new(cursor.x - bounds.x, cursor.y - bounds.y);

        for (gi, (_, values, _)) in self.groups.iter().enumerate() {
            for (ci, v) in values.iter().enumerate() {
                if ci >= self.categories.len() {
                    break;
                }
                let left = x_band.left(ci) + bar_w * gi as f32;
                let top = y_scale.to_pixel(v.max(0.0));
                let bottom = if *v < 0.0 {
                    y_scale.to_pixel(*v)
                } else {
                    zero_y
                };
                let y = top.min(bottom);
                let h = (top - bottom).abs().max(1.0);
                let rect = Rectangle {
                    x: left,
                    y,
                    width: (bar_w - 2.0).max(1.0),
                    height: h,
                };
                if local.x >= rect.x
                    && local.x <= rect.x + rect.width
                    && local.y >= rect.y
                    && local.y <= rect.y + rect.height
                {
                    return Some((gi, ci, rect));
                }
            }
        }
        None
    }
}

pub fn bar_chart<'a, Message: 'a>(
    theme: &AppTheme,
    categories: Vec<String>,
    groups: Vec<(String, Vec<f32>)>,
) -> Element<'a, Message> {
    let canvas = Canvas::new(BarChart::new(theme, categories, groups))
        .width(Length::Fill)
        .height(Length::Fixed(260.0));
    ChartWidget::new(canvas, theme).into()
}

impl<Message> canvas::Program<Message> for BarChart {
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
            self.hit_test(bounds, pos).map(|(gi, ci, rect)| {
                let group_name = self.groups[gi].0.clone();
                let value = self.groups[gi].1[ci];
                let category = self.categories[ci].clone();
                ChartHover {
                    title: category,
                    entries: vec![ChartHoverEntry::new(
                        group_name,
                        format!("{value:.1}"),
                        self.group_color(gi),
                    )],
                    anchor: Point::new(rect.x + rect.width / 2.0, rect.y),
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
        if area.width() <= 0.0 || area.height() <= 0.0 || self.categories.is_empty() {
            return vec![frame.into_geometry()];
        }

        let mut all_values: Vec<f32> =
            self.groups.iter().flat_map(|(_, v, _)| v.iter().copied()).collect();
        all_values.push(0.0);
        let (y_min_raw, y_max) = padded_domain(all_values.into_iter());
        let y_min = y_min_raw.min(0.0);

        let x_band = Band::new(self.categories.len(), area.x_range()).padding(0.25);
        let y_scale = Linear::new((y_min, y_max), area.y_range());

        draw_y_axis(&mut frame, &self.theme, area, y_scale, 5, format_tick);

        let labels: Vec<(f32, String)> = self
            .categories
            .iter()
            .enumerate()
            .map(|(i, l)| (x_band.center(i), l.clone()))
            .collect();
        draw_x_labels(&mut frame, &self.theme, area, &labels);

        // Re-run hit-test from current cursor so highlight stays in lock-step
        // with the tooltip without needing to repeat identity in `ChartHover`.
        let hovered = cursor
            .position()
            .and_then(|p| self.hit_test(bounds, p))
            .map(|(gi, ci, _)| (gi, ci))
            .or_else(|| {
                // Hover may persist for a moment without cursor in-bounds.
                state.hover.as_ref().and_then(|h| {
                    let ci = self.categories.iter().position(|c| c == &h.title)?;
                    let gi = h
                        .entries
                        .first()
                        .and_then(|e| self.groups.iter().position(|g| g.0 == e.label))?;
                    Some((gi, ci))
                })
            });

        let group_count = self.groups.len().max(1);
        let band_w = x_band.bandwidth();
        let bar_w = band_w / group_count as f32;
        let zero_y = y_scale.to_pixel(0.0);

        for (gi, (_, values, _)) in self.groups.iter().enumerate() {
            let base_color = self.group_color(gi);
            for (i, v) in values.iter().enumerate() {
                if i >= self.categories.len() {
                    break;
                }
                let left = x_band.left(i) + bar_w * gi as f32;
                let top = y_scale.to_pixel(v.max(0.0));
                let bottom = if *v < 0.0 {
                    y_scale.to_pixel(*v)
                } else {
                    zero_y
                };
                let y = top.min(bottom);
                let h = (top - bottom).abs().max(1.0);

                let is_hovered = hovered == Some((gi, i));
                let fill = if hovered.is_some() && !is_hovered {
                    with_alpha(base_color, 0.45)
                } else {
                    base_color
                };

                let path = Path::rectangle(
                    Point::new(left, y),
                    iced::Size::new(bar_w - 2.0, h),
                );
                frame.fill(&path, canvas::Fill::from(fill));

                if is_hovered {
                    frame.stroke(
                        &path,
                        Stroke::default()
                            .with_color(self.theme.foreground)
                            .with_width(1.5),
                    );
                }
            }
        }

        vec![frame.into_geometry()]
    }
}
