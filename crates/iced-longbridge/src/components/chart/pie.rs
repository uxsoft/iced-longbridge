//! Pie / donut chart.

use iced::{
    Color, Element, Length, Pixels, Point, Rectangle, Renderer, Theme,
    advanced::text::Alignment as TextAlign,
    alignment::Vertical,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text},
};

use crate::theme::{AppTheme, with_alpha};

use super::interactive::{ChartHover, ChartHoverEntry, ChartState, ChartWidget};

#[derive(Debug, Clone)]
pub struct PieSlice {
    pub label: String,
    pub value: f32,
    pub color: Option<Color>,
}

impl PieSlice {
    pub fn new(label: impl Into<String>, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    #[allow(dead_code)]
    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }
}

pub struct PieChart {
    pub theme: AppTheme,
    pub slices: Vec<PieSlice>,
    /// Inner-to-outer radius ratio. `0.0` = full pie, `0.6` = donut.
    pub inner_ratio: f32,
}

impl PieChart {
    pub fn new(theme: &AppTheme, slices: Vec<PieSlice>) -> Self {
        Self {
            theme: *theme,
            slices,
            inner_ratio: 0.0,
        }
    }

    pub fn donut(mut self, ratio: f32) -> Self {
        self.inner_ratio = ratio.clamp(0.0, 0.95);
        self
    }
}

struct PieLayout {
    center: Point,
    radius: f32,
    inner_radius: f32,
}

impl PieChart {
    fn layout(&self, bounds: Rectangle) -> Option<PieLayout> {
        let legend_w = 160.0_f32.min(bounds.width * 0.4);
        let pie_w = (bounds.width - legend_w).max(60.0);
        let pie_h = bounds.height;
        let radius = (pie_w.min(pie_h) / 2.0) - 12.0;
        if radius <= 0.0 {
            return None;
        }
        Some(PieLayout {
            center: Point::new(pie_w / 2.0, pie_h / 2.0),
            radius,
            inner_radius: radius * self.inner_ratio,
        })
    }

    fn slice_color(&self, index: usize) -> Color {
        self.slices[index]
            .color
            .unwrap_or(self.theme.chart[index % self.theme.chart.len()])
    }

    fn total(&self) -> f32 {
        self.slices.iter().map(|s| s.value.max(0.0)).sum()
    }

    fn hit_test(&self, bounds: Rectangle, cursor: Point) -> Option<(usize, Point)> {
        let layout = self.layout(bounds)?;
        let total = self.total();
        if total <= 0.0 {
            return None;
        }

        // Cursor relative to canvas origin.
        let local = Point::new(cursor.x - bounds.x, cursor.y - bounds.y);
        let dx = local.x - layout.center.x;
        let dy = local.y - layout.center.y;
        let r = (dx * dx + dy * dy).sqrt();
        if r < layout.inner_radius || r > layout.radius {
            return None;
        }

        // Normalize angle to match draw's convention (-PI/2 at top, clockwise).
        let mut angle = dy.atan2(dx);
        while angle < -std::f32::consts::FRAC_PI_2 {
            angle += std::f32::consts::TAU;
        }

        let mut start = -std::f32::consts::FRAC_PI_2;
        for (i, slice) in self.slices.iter().enumerate() {
            let frac = slice.value.max(0.0) / total;
            if frac <= 0.0 {
                continue;
            }
            let end = start + frac * std::f32::consts::TAU;
            if angle >= start && angle <= end {
                // Anchor at the midpoint of the slice outer arc.
                let mid_angle = (start + end) / 2.0;
                let mid_r = (layout.radius + layout.inner_radius.max(layout.radius * 0.35))
                    / 2.0;
                let anchor = polar(layout.center, mid_r, mid_angle);
                return Some((i, anchor));
            }
            start = end;
        }
        None
    }
}

fn build_hover(chart: &PieChart, index: usize, anchor: Point) -> ChartHover {
    let total = chart.total();
    let slice = &chart.slices[index];
    let pct = if total > 0.0 {
        slice.value / total * 100.0
    } else {
        0.0
    };
    ChartHover {
        title: slice.label.clone(),
        entries: vec![
            ChartHoverEntry::new("Value", format!("{:.1}", slice.value), chart.slice_color(index)),
            ChartHoverEntry::new("Share", format!("{pct:.1}%"), chart.slice_color(index)),
        ],
        anchor,
    }
}

#[allow(dead_code)]
pub fn pie_chart<'a, Message: 'a>(
    theme: &AppTheme,
    slices: Vec<PieSlice>,
) -> Element<'a, Message> {
    let canvas = Canvas::new(PieChart::new(theme, slices))
        .width(Length::Fill)
        .height(Length::Fixed(260.0));
    ChartWidget::new(canvas, theme).into()
}

#[allow(dead_code)]
pub fn donut_chart<'a, Message: 'a>(
    theme: &AppTheme,
    slices: Vec<PieSlice>,
) -> Element<'a, Message> {
    let canvas = Canvas::new(PieChart::new(theme, slices).donut(0.55))
        .width(Length::Fill)
        .height(Length::Fixed(260.0));
    ChartWidget::new(canvas, theme).into()
}

impl<Message> canvas::Program<Message> for PieChart {
    type State = ChartState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let is_mouse = matches!(
            event,
            canvas::Event::Mouse(_)
        );
        if !is_mouse {
            return None;
        }

        let new_hover = cursor
            .position()
            .and_then(|pos| {
                if bounds.contains(pos) {
                    self.hit_test(bounds, pos)
                        .map(|(i, anchor)| build_hover(self, i, anchor))
                } else {
                    None
                }
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
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill_rectangle(
            Point::new(0.0, 0.0),
            bounds.size(),
            canvas::Fill::from(self.theme.card),
        );

        let total = self.total();
        if total <= 0.0 || self.slices.is_empty() {
            return vec![frame.into_geometry()];
        }

        let Some(layout) = self.layout(bounds) else {
            return vec![frame.into_geometry()];
        };

        // Labels uniquely identify a slice in the demo data — use them to
        // recover the hovered index without widening ChartHover.
        let hovered = state
            .hover
            .as_ref()
            .and_then(|h| self.slices.iter().position(|s| s.label == h.title));

        let mut start = -std::f32::consts::FRAC_PI_2;
        for (i, slice) in self.slices.iter().enumerate() {
            let frac = slice.value.max(0.0) / total;
            if frac <= 0.0 {
                continue;
            }
            let end = start + frac * std::f32::consts::TAU;
            let color = self.slice_color(i);
            let is_hovered = hovered == Some(i);

            // When one slice is hovered, dim the others slightly.
            let fill = if hovered.is_some() && !is_hovered {
                with_alpha(color, 0.55)
            } else {
                color
            };

            // Push hovered slice outward a little.
            let (r_out, r_in) = if is_hovered {
                (layout.radius + 6.0, layout.inner_radius)
            } else {
                (layout.radius, layout.inner_radius)
            };

            let steps = ((frac * 96.0).ceil() as usize).max(2);
            let step = (end - start) / steps as f32;
            let path = Path::new(|b| {
                if r_in > 0.0 {
                    b.move_to(polar(layout.center, r_out, start));
                    for k in 1..=steps {
                        b.line_to(polar(layout.center, r_out, start + step * k as f32));
                    }
                    b.line_to(polar(layout.center, r_in, end));
                    for k in 1..=steps {
                        b.line_to(polar(layout.center, r_in, end - step * k as f32));
                    }
                    b.close();
                } else {
                    b.move_to(layout.center);
                    b.line_to(polar(layout.center, r_out, start));
                    for k in 1..=steps {
                        b.line_to(polar(layout.center, r_out, start + step * k as f32));
                    }
                    b.close();
                }
            });

            frame.fill(&path, canvas::Fill::from(fill));
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(self.theme.card)
                    .with_width(if is_hovered { 2.0 } else { 1.5 }),
            );

            start = end;
        }

        // Legend
        let pie_w = layout.center.x * 2.0;
        let legend_x = pie_w + 8.0;
        let mut y = (bounds.height - self.slices.len() as f32 * 18.0) / 2.0;
        if y < 4.0 {
            y = 4.0;
        }
        for (i, slice) in self.slices.iter().enumerate() {
            let color = self.slice_color(i);
            let swatch = Path::rectangle(
                Point::new(legend_x, y),
                iced::Size::new(12.0, 12.0),
            );
            frame.fill(&swatch, canvas::Fill::from(color));

            let pct = slice.value.max(0.0) / total * 100.0;
            frame.fill_text(Text {
                content: format!("{} — {:.1}%", slice.label, pct),
                position: Point::new(legend_x + 18.0, y + 6.0),
                color: self.theme.foreground,
                size: Pixels(11.0),
                align_x: TextAlign::Left,
                align_y: Vertical::Center,
                ..Default::default()
            });
            y += 18.0;
        }

        vec![frame.into_geometry()]
    }
}

fn polar(center: Point, r: f32, angle: f32) -> Point {
    Point::new(
        center.x + r * angle.cos(),
        center.y + r * angle.sin(),
    )
}
