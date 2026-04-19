use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::tag::{tag, TagVariant},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let variants = row![
        tag(theme, "default", TagVariant::Default),
        tag(theme, "secondary", TagVariant::Secondary),
        tag(theme, "outline", TagVariant::Outline),
        tag(theme, "danger", TagVariant::Danger),
        tag(theme, "success", TagVariant::Success),
        tag(theme, "warning", TagVariant::Warning),
        tag(theme, "info", TagVariant::Info),
    ]
    .spacing(6)
    .align_y(iced::alignment::Vertical::Center);

    let tech = row![
        tag(theme, "rust", TagVariant::Danger),
        tag(theme, "iced", TagVariant::Info),
        tag(theme, "gpui", TagVariant::Warning),
        tag(theme, "ui", TagVariant::Success),
        tag(theme, "components", TagVariant::Default),
    ]
    .spacing(6);

    column![
        header("Variants"),
        variants,
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Labels"),
        tech,
    ]
    .spacing(12)
    .into()
}
