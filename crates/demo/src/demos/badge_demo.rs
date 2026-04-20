use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::badge::{badge, BadgeVariant},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let variants = row![
        badge(theme, "Default", BadgeVariant::Default),
        badge(theme, "Secondary", BadgeVariant::Secondary),
        badge(theme, "Outline", BadgeVariant::Outline),
        badge(theme, "Danger", BadgeVariant::Danger),
        badge(theme, "Success", BadgeVariant::Success),
        badge(theme, "Warning", BadgeVariant::Warning),
        badge(theme, "Info", BadgeVariant::Info),
    ]
    .spacing(8)
    .align_y(iced::alignment::Vertical::Center);

    let numeric = row![
        badge(theme, "1", BadgeVariant::Danger),
        badge(theme, "12", BadgeVariant::Danger),
        badge(theme, "99+", BadgeVariant::Danger),
        badge(theme, "New", BadgeVariant::Info),
    ]
    .spacing(8);

    column![
        header("Variants"),
        variants,
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Numeric / labels"),
        numeric,
    ]
    .spacing(12)
    .into()
}
