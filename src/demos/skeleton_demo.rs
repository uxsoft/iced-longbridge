use iced::{
    Element, Length,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::skeleton,
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let profile_card = row![
        skeleton::circle(theme, 48.0),
        column![
            skeleton::skeleton(theme, Length::Fixed(200.0), Length::Fixed(14.0)),
            skeleton::skeleton(theme, Length::Fixed(160.0), Length::Fixed(12.0)),
        ].spacing(8),
    ]
    .spacing(12)
    .align_y(iced::alignment::Vertical::Center);

    let text_lines = column![
        skeleton::skeleton(theme, Length::Fixed(400.0), Length::Fixed(14.0)),
        skeleton::skeleton(theme, Length::Fixed(380.0), Length::Fixed(14.0)),
        skeleton::skeleton(theme, Length::Fixed(360.0), Length::Fixed(14.0)),
        skeleton::skeleton(theme, Length::Fixed(280.0), Length::Fixed(14.0)),
    ]
    .spacing(8);

    column![
        header("Profile card"),
        profile_card,
        Space::new().height(Length::Fixed(16.0)),
        header("Text"),
        text_lines,
    ]
    .spacing(12)
    .into()
}
