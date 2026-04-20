//! Hover card — rich, larger-than-tooltip hover content.
//!
//! Backed by iced's `tooltip` widget with a themed content panel.

use iced::{
    Background, Border, Color, Element, Padding, Shadow, Vector,
    widget::{container, tooltip},
};

use crate::theme::AppTheme;

pub fn hover_card<'a, Message: 'a>(
    theme: &AppTheme,
    trigger: Element<'a, Message>,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    let panel = container(content)
        .padding(Padding::from([12.0, 14.0]))
        .max_width(320)
        .style(move |_| container::Style {
            background: Some(Background::Color(t.popover)),
            text_color: Some(t.popover_foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            snap: true,
        });
    tooltip(trigger, panel, tooltip::Position::Top)
        .gap(6.0)
        .into()
}
