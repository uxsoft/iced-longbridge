//! Slider component.

use iced::widget::{slider, Slider};

use crate::theme::AppTheme;

pub fn slider_styled<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> Slider<'a, f32, Message> {
    let t = *theme;
    slider(range, value, on_change)
        .step(0.01)
        .style(move |_, status| {
            use iced::widget::slider::Status;
            let (rail_bg, rail_fill, handle) = match status {
                Status::Hovered => (t.muted, t.primary_hover, t.primary_hover),
                Status::Dragged => (t.muted, t.primary_active, t.primary_active),
                Status::Active => (t.muted, t.primary, t.primary),
            };
            iced::widget::slider::Style {
                rail: iced::widget::slider::Rail {
                    backgrounds: (rail_fill.into(), rail_bg.into()),
                    width: 4.0,
                    border: iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: 999.0.into(),
                    },
                },
                handle: iced::widget::slider::Handle {
                    shape: iced::widget::slider::HandleShape::Circle { radius: 9.0 },
                    background: handle.into(),
                    border_color: t.background,
                    border_width: 2.0,
                },
            }
        })
}
