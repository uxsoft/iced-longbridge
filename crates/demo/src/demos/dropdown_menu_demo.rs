use iced::{Element, widget::{column, row}};

use crate::{
    Message, State,
    components::{
        dropdown_button::{dropdown_button, dropdown_button_sized},
        menu::Item,
    },
    demos::common::{section_caption, section_title, vspace},
    lucide,
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let open = state.dropdown_menu_open;

    let basic_items = vec![
        Item::new("New file", Message::MenuAction("New file".into())).icon(lucide::file()),
        Item::new("New folder", Message::MenuAction("New folder".into())).icon(lucide::folder()),
        Item::Separator,
        Item::new("Open", Message::MenuAction("Open".into())).shortcut("⌘O"),
        Item::new("Save", Message::MenuAction("Save".into())).shortcut("⌘S"),
        Item::Separator,
        Item::Header("Danger zone".into()),
        Item::new("Delete", Message::MenuAction("Delete".into())).icon(lucide::trash_2()).danger(),
    ];

    let size_items = vec![
        Item::new("Item A", Message::NoOp),
        Item::new("Item B", Message::NoOp),
        Item::new("Item C", Message::NoOp).disabled(),
    ];

    let sizes = row![
        dropdown_button_sized(theme, Size::Xs, "XS", open == Some(0), Message::DropdownMenuToggle(0), sample_items()),
        dropdown_button_sized(theme, Size::Sm, "SM", open == Some(1), Message::DropdownMenuToggle(1), sample_items()),
        dropdown_button_sized(theme, Size::Md, "MD", open == Some(2), Message::DropdownMenuToggle(2), sample_items()),
        dropdown_button_sized(theme, Size::Lg, "LG", open == Some(3), Message::DropdownMenuToggle(3), sample_items()),
    ]
    .spacing(8)
    .align_y(iced::alignment::Vertical::Top);

    column![
        section_title(theme, "Dropdown button"),
        section_caption(theme, "A button that reveals a menu of actions."),
        dropdown_button(theme, "File", open == Some(10), Message::DropdownMenuToggle(10), basic_items),
        vspace(14.0),
        section_title(theme, "Disabled items / simple"),
        dropdown_button(theme, "Options", open == Some(11), Message::DropdownMenuToggle(11), size_items),
        vspace(14.0),
        section_title(theme, "Sizes"),
        sizes,
    ]
    .spacing(10)
    .into()
}

fn sample_items() -> Vec<Item<Message>> {
    vec![
        Item::new("Cut", Message::NoOp).shortcut("⌘X"),
        Item::new("Copy", Message::NoOp).shortcut("⌘C"),
        Item::new("Paste", Message::NoOp).shortcut("⌘V"),
    ]
}
