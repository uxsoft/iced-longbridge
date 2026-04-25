use iced::{Element, widget::{column, row}};

use crate::{
    Message, State,
    components::{
        icon::IconName,
        toggle_button::{toggle_button, toggle_button_ex},
    },
    demos::common::{section_caption, section_title, vspace},
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let toggles = row![
        toggle_button(theme, "Bold", state.toggles[0], Message::ToggleButtonPressed(0)),
        toggle_button(theme, "Italic", state.toggles[1], Message::ToggleButtonPressed(1)),
        toggle_button(theme, "Underline", state.toggles[2], Message::ToggleButtonPressed(2)),
    ]
    .spacing(8);

    let with_icons = row![
        toggle_button_ex(theme, "Favorite", Some(IconName::Star.into()), state.toggles[3], Size::Md, Message::ToggleButtonPressed(3)),
        toggle_button_ex(theme, "Pinned", Some(IconName::ArrowUp.into()), state.toggles[4], Size::Md, Message::ToggleButtonPressed(4)),
        toggle_button_ex(theme, "Muted", Some(IconName::EyeOff.into()), state.toggles[5], Size::Md, Message::ToggleButtonPressed(5)),
    ]
    .spacing(8);

    let sizes = row![
        toggle_button_ex(theme, "XS", None, state.toggles[0], Size::Xs, Message::ToggleButtonPressed(0)),
        toggle_button_ex(theme, "SM", None, state.toggles[0], Size::Sm, Message::ToggleButtonPressed(0)),
        toggle_button_ex(theme, "MD", None, state.toggles[0], Size::Md, Message::ToggleButtonPressed(0)),
        toggle_button_ex(theme, "LG", None, state.toggles[0], Size::Lg, Message::ToggleButtonPressed(0)),
    ]
    .spacing(8)
    .align_y(iced::alignment::Vertical::Center);

    column![
        section_title(theme, "Toggle buttons"),
        section_caption(theme, "Persist a pressed-look when active. Click to toggle."),
        toggles,
        vspace(10.0),
        section_title(theme, "With leading icon"),
        with_icons,
        vspace(10.0),
        section_title(theme, "Sizes"),
        sizes,
    ]
    .spacing(10)
    .into()
}
