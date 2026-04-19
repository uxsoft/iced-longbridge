use iced::{
    Element,
    widget::{column, text, Space},
};

use crate::{
    Message, State,
    components::radio::radio_styled,
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    column![
        header("Single choice"),
        column![
            radio_styled(theme, "Free plan", 0u8, Some(state.radio_value), Message::RadioSelected),
            radio_styled(theme, "Pro plan", 1u8, Some(state.radio_value), Message::RadioSelected),
            radio_styled(theme, "Enterprise plan", 2u8, Some(state.radio_value), Message::RadioSelected),
        ].spacing(10),
        Space::new().height(iced::Length::Fixed(8.0)),
        text(format!("Selected: {}", state.radio_value)).size(13.0).color(theme.muted_foreground),
    ]
    .spacing(12)
    .into()
}
