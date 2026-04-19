use iced::{
    Element, Length,
    widget::{column, row, text, Space},
};

use crate::{
    Message, State,
    components::{input, label::label},
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let basic = input::input(theme, "Enter your name…", &state.input_value)
        .on_input(Message::InputChanged)
        .width(Length::Fixed(320.0));

    let sizes = column![
        input::input_sized(theme, Size::Xs, "Extra small", &state.input_value).on_input(Message::InputChanged).width(Length::Fixed(280.0)),
        input::input_sized(theme, Size::Sm, "Small", &state.input_value).on_input(Message::InputChanged).width(Length::Fixed(280.0)),
        input::input_sized(theme, Size::Md, "Medium", &state.input_value).on_input(Message::InputChanged).width(Length::Fixed(280.0)),
        input::input_sized(theme, Size::Lg, "Large", &state.input_value).on_input(Message::InputChanged).width(Length::Fixed(280.0)),
    ]
    .spacing(8);

    let disabled = input::input(theme, "Disabled", &state.input_value)
        .width(Length::Fixed(280.0));

    let form_field = column![
        label(theme, "Email address", true),
        input::input(theme, "you@example.com", &state.input_value)
            .on_input(Message::InputChanged)
            .width(Length::Fixed(320.0)),
        text("We'll never share your email.").size(12.0).color(theme.muted_foreground),
    ]
    .spacing(6);

    let password = input::input(theme, "Password", &state.password_value)
        .on_input(Message::PasswordChanged)
        .secure(true)
        .width(Length::Fixed(320.0));

    column![
        header("Basic"),
        basic,
        Space::new().height(Length::Fixed(8.0)),
        header("Sizes"),
        sizes,
        Space::new().height(Length::Fixed(8.0)),
        header("Disabled"),
        disabled,
        Space::new().height(Length::Fixed(8.0)),
        header("With label"),
        form_field,
        Space::new().height(Length::Fixed(8.0)),
        header("Password"),
        password,
        Space::new().height(Length::Fixed(8.0)),
        row![
            text("Value:").size(13.0).color(theme.muted_foreground),
            text(state.input_value.clone()).size(13.0).color(theme.foreground),
        ].spacing(8),
    ]
    .spacing(12)
    .into()
}
