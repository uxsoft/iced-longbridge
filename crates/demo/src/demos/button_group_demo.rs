use iced::{Element, widget::column};

use crate::{
    Message, State,
    components::button_group::{button_group, button_group_sized, GroupButton},
    demos::common::{section_caption, section_title, vspace},
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let bg_alignment = button_group(
        theme,
        vec![
            GroupButton::new("Left", Message::ButtonGroupSelected(0)).active(state.button_group == 0),
            GroupButton::new("Center", Message::ButtonGroupSelected(1)).active(state.button_group == 1),
            GroupButton::new("Right", Message::ButtonGroupSelected(2)).active(state.button_group == 2),
            GroupButton::new("Justify", Message::ButtonGroupSelected(3)).active(state.button_group == 3),
        ],
    );

    let bg_sizes = column![
        button_group_sized(theme, Size::Xs, sample_buttons(state)),
        button_group_sized(theme, Size::Sm, sample_buttons(state)),
        button_group_sized(theme, Size::Md, sample_buttons(state)),
        button_group_sized(theme, Size::Lg, sample_buttons(state)),
    ]
    .spacing(8);

    column![
        section_title(theme, "Segmented"),
        section_caption(theme, "Choose a single option from a small set."),
        bg_alignment,
        vspace(14.0),
        section_title(theme, "Sizes"),
        bg_sizes,
    ]
    .spacing(10)
    .into()
}

fn sample_buttons(state: &State) -> Vec<GroupButton<Message>> {
    vec![
        GroupButton::new("Day", Message::ButtonGroupSelected(0)).active(state.button_group == 0),
        GroupButton::new("Week", Message::ButtonGroupSelected(1)).active(state.button_group == 1),
        GroupButton::new("Month", Message::ButtonGroupSelected(2)).active(state.button_group == 2),
    ]
}
