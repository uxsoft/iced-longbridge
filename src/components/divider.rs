//! Divider — horizontal or vertical separator.

use iced::{
    Background, Border, Element, Length, Shadow,
    widget::container,
};

use crate::theme::AppTheme;

pub fn horizontal<'a, Message: 'a>(theme: &AppTheme) -> Element<'a, Message> {
    let t = *theme;
    container(iced::widget::Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.border)),
            text_color: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        })
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .into()
}

pub fn vertical<'a, Message: 'a>(theme: &AppTheme) -> Element<'a, Message> {
    let t = *theme;
    container(iced::widget::Space::new().width(Length::Fixed(1.0)).height(Length::Fill))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.border)),
            text_color: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        })
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .into()
}
