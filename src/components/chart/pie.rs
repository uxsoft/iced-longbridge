//! Pie / donut chart.

use iced::{
    Color, Element, Length, Pixels, Point, Radians, Rectangle, Renderer, Theme,
    advanced::text::Alignment as TextAlign,
    alignment::Vertical,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text},
};

use crate::theme::AppTheme;

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

#[allow(dead_code)]
pub fn pie_chart<'a, Message: 'a>(
    theme: &AppTheme,
    slices: Vec<PieSlice>,
) -> Element<'a, Message> {
    Canvas::new(PieChart::new(theme, slices))
        .width(Length::Fill)
        .height(Length::Fixed(260.0))
        .into()
}

#[allow(dead_code)]
pub fn donut_chart<'a, Message: 'a>(
    theme: &AppTheme,
    slices: Vec<PieSlice>,
) -> Element<'a, Message> {
    Canvas::new(PieChart::new(theme, slices).donut(0.55))
        .width(Length::Fill)
        .height(Length::Fixed(260.0))
        .into()
}

impl<Message> canvas::Program<Message> for PieChart {
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
        frame.fill_rectangle(
            Point::new(0.0, 0.0),
            bounds.size(),
            canvas::Fill::from(self.theme.card),
        );

        let total: f32 = self.slices.iter().map(|s| s.value.max(0.0)).sum();
        if total <= 0.0 || self.slices.is_empty() {
            return vec![frame.into_geometry()];
        }

        // Reserve ~160px on the right for a legend, leaving a square pie region.
        let legend_w = 160.0_f32.min(bounds.width * 0.4);
        let pie_w = (bounds.width - legend_w).max(60.0);
        let pie_h = bounds.height;
        let radius = (pie_w.min(pie_h) / 2.0) - 12.0;
        let center = Point::new(pie_w / 2.0, pie_h / 2.0);
        let inner_radius = radius * self.inner_ratio;

        let mut start = -std::f32::consts::FRAC_PI_2;
        for (i, slice) in self.slices.iter().enumerate() {
            let frac = slice.value.max(0.0) / total;
            if frac <= 0.0 {
                continue;
            }
            let end = start + frac * std::f32::consts::TAU;
            let color = slice
                .color
                .unwrap_or(self.theme.chart[i % self.theme.chart.len()]);

            let path = Path::new(|b| {
                if inner_radius > 0.0 {
                    let start_inner = polar(center, inner_radius, start);
                    b.move_to(start_inner);
                    b.line_to(polar(center, radius, start));
                    b.arc(canvas::path::Arc {
                        center,
                        radius,
                        start_angle: Radians(start),
                        end_angle: Radians(end),
                    });
                    b.line_to(polar(center, inner_radius, end));
                    b.arc(canvas::path::Arc {
                        center,
                        radius: inner_radius,
                        start_angle: Radians(end),
                        end_angle: Radians(start),
                    });
                    b.close();
                } else {
                    b.move_to(center);
                    b.line_to(polar(center, radius, start));
                    b.arc(canvas::path::Arc {
                        center,
                        radius,
                        start_angle: Radians(start),
                        end_angle: Radians(end),
                    });
                    b.close();
                }
            });

            frame.fill(&path, canvas::Fill::from(color));
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(self.theme.card)
                    .with_width(1.5),
            );

            start = end;
        }

        // Legend
        let legend_x = pie_w + 8.0;
        let mut y = (pie_h - self.slices.len() as f32 * 18.0) / 2.0;
        if y < 4.0 {
            y = 4.0;
        }
        for (i, slice) in self.slices.iter().enumerate() {
            let color = slice
                .color
                .unwrap_or(self.theme.chart[i % self.theme.chart.len()]);
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
