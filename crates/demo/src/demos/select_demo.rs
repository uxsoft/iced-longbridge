use iced::{Element, widget::{column, row, text}};

use crate::{
    Message, State,
    components::select::{combobox, select, select_sized},
    demos::common::{section_caption, section_title, vspace},
    theme::{AppTheme, Size},
};

pub const FRUITS: &[&str] = &[
    "Apple",
    "Banana",
    "Cherry",
    "Date",
    "Elderberry",
    "Fig",
    "Grape",
    "Honeydew",
];

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let basic = select(theme, FRUITS, state.select_fruit, Message::SelectFruit)
        .placeholder("Choose a fruit");

    let sizes = column![
        select_sized(theme, Size::Xs, FRUITS, state.select_fruit, Message::SelectFruit)
            .placeholder("XS"),
        select_sized(theme, Size::Sm, FRUITS, state.select_fruit, Message::SelectFruit)
            .placeholder("SM"),
        select_sized(theme, Size::Md, FRUITS, state.select_fruit, Message::SelectFruit)
            .placeholder("MD"),
        select_sized(theme, Size::Lg, FRUITS, state.select_fruit, Message::SelectFruit)
            .placeholder("LG"),
    ]
    .spacing(8);

    let combo = combobox(
        theme,
        &state.combobox_state,
        "Type to filter fruits…",
        state.combobox_value.as_ref(),
        Message::ComboboxSelected,
    );

    column![
        section_title(theme, "Pick list"),
        section_caption(theme, "Simple dropdown with options."),
        row![basic].spacing(10),
        text(format!("Selected: {}", state.select_fruit.unwrap_or("—"))).size(12.0).color(theme.muted_foreground),
        vspace(14.0),
        section_title(theme, "Sizes"),
        sizes,
        vspace(14.0),
        section_title(theme, "Combobox (filterable)"),
        combo,
    ]
    .spacing(10)
    .into()
}
