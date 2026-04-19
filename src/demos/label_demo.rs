use iced::{
    Element, Length,
    widget::{column, text, Space},
};

use crate::{
    Message, State,
    components::{input, label::label},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    column![
        header("Basic"),
        label(theme, "Username", false),
        input::input(theme, "Enter username", &state.input_value)
            .on_input(Message::InputChanged)
            .width(Length::Fixed(320.0)),
        Space::new().height(Length::Fixed(8.0)),
        header("Required"),
        label(theme, "Email", true),
        input::input(theme, "you@example.com", &state.input_value)
            .on_input(Message::InputChanged)
            .width(Length::Fixed(320.0)),
    ]
    .spacing(8)
    .into()
}
