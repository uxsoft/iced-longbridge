use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::avatar::initials,
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let sizes = row![
        initials(theme, "AB", 24.0),
        initials(theme, "CD", 32.0),
        initials(theme, "EF", 40.0),
        initials(theme, "GH", 56.0),
        initials(theme, "IJ", 72.0),
    ]
    .spacing(12)
    .align_y(iced::alignment::Vertical::Center);

    let names = row![
        initials(theme, "Jane Doe", 40.0),
        initials(theme, "John Smith", 40.0),
        initials(theme, "Longbridge UI", 40.0),
        initials(theme, "Rust Iced", 40.0),
    ]
    .spacing(8);

    column![
        header("Sizes"),
        sizes,
        Space::new().height(iced::Length::Fixed(8.0)),
        header("From names"),
        names,
    ]
    .spacing(12)
    .into()
}
