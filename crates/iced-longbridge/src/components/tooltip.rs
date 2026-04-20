//! Tooltip — wraps another element and shows a popover on hover.

use iced::{
    Background, Border, Element, Padding, Shadow,
    widget::{container, text, tooltip, Tooltip},
};

use crate::theme::AppTheme;

pub fn wrap<'a, Message: 'a>(
    theme: &AppTheme,
    content: Element<'a, Message>,
    label: impl Into<String>,
) -> Tooltip<'a, Message> {
    let t = *theme;
    let label = label.into();
    let tip = container(text(label).size(12.0).color(t.popover_foreground))
        .padding(Padding::from([4.0, 8.0]))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.popover)),
            text_color: Some(t.popover_foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 6.0,
            },
            snap: true,
        });

    tooltip(content, tip, tooltip::Position::Top)
        .gap(6)
}
