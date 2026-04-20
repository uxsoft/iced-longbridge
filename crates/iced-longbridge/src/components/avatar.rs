//! Avatar — circular user identity.

use iced::{
    Background, Border, Element, Length, Padding, Shadow, alignment::{Horizontal, Vertical},
    widget::{container, text},
};

use crate::theme::AppTheme;

pub fn initials<'a, Message: 'a>(
    theme: &AppTheme,
    label: impl Into<String>,
    size: f32,
) -> Element<'a, Message> {
    let t = *theme;
    let s = label.into();
    let initials: String = s
        .split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase();

    container(
        text(initials)
            .size(size * 0.4)
            .color(t.primary_foreground)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
    )
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .padding(Padding::ZERO)
    .style(move |_| container::Style {
        background: Some(Background::Color(t.primary)),
        text_color: Some(t.primary_foreground),
        border: Border {
            color: t.border,
            width: 0.0,
            radius: 999.0.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    })
    .into()
}
