use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::link::link,
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    column![
        header("Links"),
        link(theme, "Visit the iced website", Message::LinkClicked),
        link(theme, "Read the gpui-component docs", Message::LinkClicked),
        link(theme, "View source on GitHub", Message::LinkClicked),
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Inline"),
        row![
            text("Click ").size(14.0).color(theme.foreground),
            link(theme, "here", Message::LinkClicked),
            text(" to continue.").size(14.0).color(theme.foreground),
        ]
        .align_y(iced::alignment::Vertical::Center),
    ]
    .spacing(10)
    .into()
}
