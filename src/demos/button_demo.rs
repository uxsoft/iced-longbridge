use iced::{
    Element, Length,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::button::{button_ex, Variant},
    theme::{AppTheme, Size},
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let section_title = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let variants = row![
        button_ex(theme, "Primary", Variant::Primary, Size::Md, Some(Message::ButtonPressed("Primary".into())), false, false),
        button_ex(theme, "Secondary", Variant::Secondary, Size::Md, Some(Message::ButtonPressed("Secondary".into())), false, false),
        button_ex(theme, "Outline", Variant::Outline, Size::Md, Some(Message::ButtonPressed("Outline".into())), false, false),
        button_ex(theme, "Ghost", Variant::Ghost, Size::Md, Some(Message::ButtonPressed("Ghost".into())), false, false),
        button_ex(theme, "Link", Variant::Link, Size::Md, Some(Message::ButtonPressed("Link".into())), false, false),
    ]
    .spacing(8);

    let status_row = row![
        button_ex(theme, "Danger", Variant::Danger, Size::Md, Some(Message::ButtonPressed("Danger".into())), false, false),
        button_ex(theme, "Success", Variant::Success, Size::Md, Some(Message::ButtonPressed("Success".into())), false, false),
        button_ex(theme, "Warning", Variant::Warning, Size::Md, Some(Message::ButtonPressed("Warning".into())), false, false),
        button_ex(theme, "Info", Variant::Info, Size::Md, Some(Message::ButtonPressed("Info".into())), false, false),
    ]
    .spacing(8);

    let sizes = row![
        button_ex(theme, "Extra Small", Variant::Primary, Size::Xs, Some(Message::NoOp), false, false),
        button_ex(theme, "Small", Variant::Primary, Size::Sm, Some(Message::NoOp), false, false),
        button_ex(theme, "Medium", Variant::Primary, Size::Md, Some(Message::NoOp), false, false),
        button_ex(theme, "Large", Variant::Primary, Size::Lg, Some(Message::NoOp), false, false),
    ]
    .spacing(8)
    .align_y(iced::alignment::Vertical::Center);

    let states = row![
        button_ex(theme, "Normal", Variant::Primary, Size::Md, Some(Message::NoOp), false, false),
        button_ex(theme, "Disabled", Variant::Primary, Size::Md, Some(Message::NoOp), false, true),
        button_ex(theme, "Loading", Variant::Primary, Size::Md, Some(Message::NoOp), true, false),
    ]
    .spacing(8);

    column![
        section_title("Variants"),
        variants,
        Space::new().height(Length::Fixed(8.0)),
        section_title("Status variants"),
        status_row,
        Space::new().height(Length::Fixed(8.0)),
        section_title("Sizes"),
        sizes,
        Space::new().height(Length::Fixed(8.0)),
        section_title("States"),
        states,
    ]
    .spacing(12)
    .into()
}
