use iced::{Element, widget::column};

use crate::{
    Message, State,
    components::{
        menu::Item,
        menu_bar::{menu_bar, MenuBarMenu},
    },
    demos::common::{section_caption, section_title},
    lucide,
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let open = state
        .dropdown_menu_open
        .and_then(|id| id.checked_sub(20))
        .map(|i| i as usize);

    let menus = vec![
        MenuBarMenu::new(
            "File",
            Message::DropdownMenuToggle(20),
            vec![
                Item::new("New file", Message::MenuAction("New file".into()))
                    .icon(lucide::file())
                    .shortcut("⌘N"),
                Item::new("New folder", Message::MenuAction("New folder".into()))
                    .icon(lucide::folder()),
                Item::Separator,
                Item::new("Open…", Message::MenuAction("Open".into())).shortcut("⌘O"),
                Item::new("Save", Message::MenuAction("Save".into())).shortcut("⌘S"),
                Item::new("Save as…", Message::MenuAction("Save as".into())).shortcut("⇧⌘S"),
                Item::Separator,
                Item::new("Close window", Message::MenuAction("Close window".into()))
                    .shortcut("⌘W"),
                Item::new("Quit", Message::MenuAction("Quit".into())).shortcut("⌘Q"),
            ],
        ),
        MenuBarMenu::new(
            "Edit",
            Message::DropdownMenuToggle(21),
            vec![
                Item::new("Undo", Message::MenuAction("Undo".into())).shortcut("⌘Z"),
                Item::new("Redo", Message::MenuAction("Redo".into())).shortcut("⇧⌘Z"),
                Item::Separator,
                Item::new("Cut", Message::MenuAction("Cut".into())).shortcut("⌘X"),
                Item::new("Copy", Message::MenuAction("Copy".into())).shortcut("⌘C"),
                Item::new("Paste", Message::MenuAction("Paste".into())).shortcut("⌘V"),
                Item::Separator,
                Item::new("Find…", Message::MenuAction("Find".into())).shortcut("⌘F"),
                Item::new("Replace…", Message::MenuAction("Replace".into()))
                    .shortcut("⌥⌘F"),
            ],
        ),
        MenuBarMenu::new(
            "View",
            Message::DropdownMenuToggle(22),
            vec![
                Item::Header("Appearance".into()),
                Item::new("Toggle sidebar", Message::MenuAction("Toggle sidebar".into()))
                    .shortcut("⌘B"),
                Item::new("Toggle panel", Message::MenuAction("Toggle panel".into()))
                    .shortcut("⌘J"),
                Item::Separator,
                Item::new("Zoom in", Message::MenuAction("Zoom in".into())).shortcut("⌘+"),
                Item::new("Zoom out", Message::MenuAction("Zoom out".into())).shortcut("⌘-"),
                Item::new("Reset zoom", Message::MenuAction("Reset zoom".into()))
                    .shortcut("⌘0"),
            ],
        ),
        MenuBarMenu::new(
            "Help",
            Message::DropdownMenuToggle(23),
            vec![
                Item::new("Documentation", Message::MenuAction("Docs".into())),
                Item::new("Keyboard shortcuts", Message::MenuAction("Shortcuts".into()))
                    .shortcut("⌘/"),
                Item::Separator,
                Item::new("Report issue", Message::MenuAction("Report issue".into()))
                    .disabled(),
                Item::new("About", Message::MenuAction("About".into())),
            ],
        ),
    ];

    column![
        section_title(theme, "Application menu bar"),
        section_caption(
            theme,
            "A horizontal strip of labeled menus — typical desktop app chrome.",
        ),
        menu_bar(theme, menus, open),
    ]
    .spacing(10)
    .into()
}
