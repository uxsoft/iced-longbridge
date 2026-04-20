use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::kbd::{combo, kbd},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let singles = row![
        kbd(theme, "A"),
        kbd(theme, "Enter"),
        kbd(theme, "Esc"),
        kbd(theme, "Tab"),
        kbd(theme, "⇧"),
    ]
    .spacing(6)
    .align_y(iced::alignment::Vertical::Center);

    let shortcuts = column![
        row![text("Copy").size(14.0).color(theme.foreground), Space::new().width(iced::Length::Fixed(16.0)), combo(theme, &["Ctrl", "C"])].align_y(iced::alignment::Vertical::Center),
        row![text("Paste").size(14.0).color(theme.foreground), Space::new().width(iced::Length::Fixed(16.0)), combo(theme, &["Ctrl", "V"])].align_y(iced::alignment::Vertical::Center),
        row![text("Save").size(14.0).color(theme.foreground), Space::new().width(iced::Length::Fixed(16.0)), combo(theme, &["Ctrl", "S"])].align_y(iced::alignment::Vertical::Center),
        row![text("Quit").size(14.0).color(theme.foreground), Space::new().width(iced::Length::Fixed(16.0)), combo(theme, &["Ctrl", "Shift", "Q"])].align_y(iced::alignment::Vertical::Center),
    ]
    .spacing(8);

    column![
        header("Single keys"),
        singles,
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Shortcuts"),
        shortcuts,
    ]
    .spacing(12)
    .into()
}
