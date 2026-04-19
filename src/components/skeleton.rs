//! Skeleton — placeholder for loading content.

use iced::{
    Background, Border, Element, Length, Shadow,
    widget::{container, Space},
};

use crate::theme::AppTheme;

pub fn skeleton<'a, Message: 'a>(
    theme: &AppTheme,
    width: Length,
    height: Length,
) -> Element<'a, Message> {
    let t = *theme;
    container(Space::new().width(width).height(height))
        .width(width)
        .height(height)
        .style(move |_| container::Style {
            background: Some(Background::Color(t.skeleton)),
            text_color: None,
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

pub fn circle<'a, Message: 'a>(
    theme: &AppTheme,
    size: f32,
) -> Element<'a, Message> {
    let t = *theme;
    container(Space::new().width(Length::Fixed(size)).height(Length::Fixed(size)))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.skeleton)),
            text_color: None,
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 999.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}
