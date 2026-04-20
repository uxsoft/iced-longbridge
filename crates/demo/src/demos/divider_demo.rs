use iced::{
    Element, Length,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::divider,
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let horizontal = column![
        text("Above").size(14.0).color(theme.foreground),
        divider::horizontal(theme),
        text("Below").size(14.0).color(theme.foreground),
    ]
    .spacing(8)
    .max_width(360);

    let vertical = row![
        text("Left").size(14.0).color(theme.foreground),
        iced::widget::container(divider::vertical(theme)).height(Length::Fixed(24.0)),
        text("Middle").size(14.0).color(theme.foreground),
        iced::widget::container(divider::vertical(theme)).height(Length::Fixed(24.0)),
        text("Right").size(14.0).color(theme.foreground),
    ]
    .spacing(12)
    .align_y(iced::alignment::Vertical::Center);

    column![
        header("Horizontal"),
        horizontal,
        Space::new().height(Length::Fixed(8.0)),
        header("Vertical"),
        vertical,
    ]
    .spacing(12)
    .into()
}
