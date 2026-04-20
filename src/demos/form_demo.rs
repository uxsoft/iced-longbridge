use iced::{Element, widget::{column, row}};

use crate::{
    Message, State,
    components::{
        button::{button_ex, Variant},
        checkbox::checkbox_styled,
        form::{form, Field},
        input::input,
        select::select,
    },
    demos::common::{section_caption, section_title, vspace},
    demos::select_demo::FRUITS,
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let email = input::<Message>(theme, "you@example.com", &state.input_value)
        .on_input(Message::InputChanged)
        .width(iced::Length::Fill);

    let password = input::<Message>(theme, "••••••••", &state.password_value)
        .on_input(Message::PasswordChanged)
        .secure(true)
        .width(iced::Length::Fill);

    let fruit_select = select(theme, FRUITS, state.select_fruit, Message::SelectFruit)
        .placeholder("Pick one")
        .width(iced::Length::Fill);

    let remember = checkbox_styled::<Message>(theme, "Remember me", state.checkboxes[0])
        .on_toggle(|v| Message::CheckboxToggled(0, v));

    let fields = vec![
        Field::new("Email", email.into()).required().help("We'll never share this."),
        Field::new("Password", password.into()).required(),
        Field::new("Favorite fruit", fruit_select.into()).help("Used for personalisation."),
        Field::new("Remember me", remember.into()),
    ];

    let errored = vec![
        Field::new("Username", input::<Message>(theme, "username", "").into()).required().error("Username is already taken."),
        Field::new(
            "Website",
            input::<Message>(theme, "https://…", "http://example").into(),
        )
        .help("Must start with https://")
        .error("Invalid URL scheme."),
    ];

    column![
        section_title(theme, "Composed form"),
        section_caption(theme, "label + input + help / error layout helper."),
        form(theme, fields),
        vspace(8.0),
        row![
            button_ex(theme, "Submit", Variant::Primary, Size::Md, Some(Message::NoOp), false, false),
            button_ex(theme, "Cancel", Variant::Ghost, Size::Md, Some(Message::NoOp), false, false),
        ]
        .spacing(8),
        vspace(18.0),
        section_title(theme, "Validation states"),
        form(theme, errored),
    ]
    .spacing(10)
    .into()
}
