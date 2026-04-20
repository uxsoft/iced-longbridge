//! Shared helpers used by every demo page to keep them consistent.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    widget::{column, container, text, Space},
};

use iced_longbridge::theme::AppTheme;

pub fn section_title<'a, Message: 'a>(theme: &AppTheme, label: impl Into<String>) -> Element<'a, Message> {
    text(label.into()).size(16.0).color(theme.foreground).into()
}

pub fn section_caption<'a, Message: 'a>(theme: &AppTheme, label: impl Into<String>) -> Element<'a, Message> {
    text(label.into()).size(12.0).color(theme.muted_foreground).into()
}

#[allow(dead_code)]
pub fn card<'a, Message: 'a>(
    theme: &AppTheme,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    container(content)
        .padding(Padding::from([16.0, 18.0]))
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
        })
        .into()
}

pub fn vspace<'a, Message: 'a>(height: f32) -> Element<'a, Message> {
    Space::new().height(Length::Fixed(height)).into()
}

#[allow(dead_code)]
pub fn hspace<'a, Message: 'a>(width: f32) -> Element<'a, Message> {
    Space::new().width(Length::Fixed(width)).into()
}

#[allow(dead_code)]
pub fn labelled_section<'a, Message: 'a>(
    theme: &AppTheme,
    label: impl Into<String>,
    body: Element<'a, Message>,
) -> Element<'a, Message> {
    column![section_title(theme, label), body].spacing(8).into()
}
