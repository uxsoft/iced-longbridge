//! Spinner — animated loading indicator.
//!
//! The caller must drive animation via a `Tick` message and pass the angle (radians).

use iced::{
    Color, Element, Point, Size as ISize, Vector,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke},
    Rectangle, Renderer, Theme,
};

use crate::theme::AppTheme;

pub struct Spinner {
    color: Color,
    size: f32,
    rotation: f32,
}

impl Spinner {
    pub fn new(theme: &AppTheme, rotation: f32) -> Self {
        Self {
            color: theme.primary,
            size: 24.0,
            rotation,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn view<'a, Message: 'a>(self) -> Element<'a, Message> {
        let size = self.size;
        Canvas::new(self)
            .width(iced::Length::Fixed(size))
            .height(iced::Length::Fixed(size))
            .into()
    }
}

impl<Message> canvas::Program<Message> for Spinner {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let radius = (bounds.width.min(bounds.height) / 2.0) - 3.0;

        let track = Path::circle(center, radius);
        frame.stroke(
            &track,
            Stroke::default()
                .with_color(Color {
                    a: 0.15,
                    ..self.color
                })
                .with_width(3.0),
        );

        let arc = Path::new(|b| {
            b.arc(canvas::path::Arc {
                center,
                radius,
                start_angle: iced::Radians(self.rotation),
                end_angle: iced::Radians(self.rotation + std::f32::consts::PI * 0.75),
            });
        });
        frame.stroke(
            &arc,
            Stroke::default()
                .with_color(self.color)
                .with_width(3.0)
                .with_line_cap(canvas::LineCap::Round),
        );

        let _ = Vector::new(0.0, 0.0);
        let _ = ISize::new(0.0, 0.0);

        vec![frame.into_geometry()]
    }
}
