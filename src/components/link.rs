//! Link — text button styled like an anchor link.

use iced::{
    Background, Border, Element, Padding, Shadow,
    widget::{button, text},
};

use crate::theme::AppTheme;

pub fn link<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    label: impl Into<String>,
    on_press: Message,
) -> Element<'a, Message> {
    let t = *theme;
    button(text(label.into()).size(14.0))
        .padding(Padding::from([0.0, 0.0]))
        .on_press(on_press)
        .style(move |_, status| {
            use button::Status::*;
            let fg = match status {
                Hovered | Pressed => t.link_hover,
                _ => t.link,
            };
            button::Style {
                background: Some(Background::Color(iced::Color::TRANSPARENT)),
                text_color: fg,
                border: Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}
