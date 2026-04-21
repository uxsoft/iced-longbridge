use iced::{Element, widget::{column, row}};

use crate::{
    Message, State,
    components::time_picker::{time_picker, time_picker_popover},
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let inline = time_picker(
        theme,
        state.time_value,
        true,
        Message::TimePickerHourChanged,
        Message::TimePickerMinuteChanged,
        Message::TimePickerSecondChanged,
    );

    let trigger = time_picker_popover(
        theme,
        state.time_value,
        false,
        state.time_picker_open,
        Message::TimePickerToggle,
        Message::TimePickerHourChanged,
        Message::TimePickerMinuteChanged,
        Message::TimePickerSecondChanged,
    );

    column![
        section_title(theme, "Inline picker"),
        section_caption(theme, "Three columns — hour, minute, second — with wrap-around stepper buttons."),
        inline,
        vspace(20.0),
        section_title(theme, "Popover trigger"),
        section_caption(theme, "Button trigger that reveals the picker in an anchored panel."),
        row![trigger].spacing(8),
    ]
    .spacing(10)
    .into()
}
