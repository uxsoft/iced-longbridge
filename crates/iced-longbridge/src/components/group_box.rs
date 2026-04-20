//! Group box — a titled bordered container.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    widget::{column, container, text},
};

use crate::theme::AppTheme;

pub fn group_box<'a, Message: 'a>(
    theme: &AppTheme,
    title: impl Into<String>,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    let header = text(title.into()).size(13.0).color(t.muted_foreground);
    let body = container(content)
        .padding(Padding::from([14.0, 16.0]))
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(t.card)),
            text_color: Some(t.card_foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        });

    column![header, body].spacing(6).into()
}
