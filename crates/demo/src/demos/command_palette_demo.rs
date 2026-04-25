use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{column, row, text, Space},
};

use crate::{
    Message, State,
    components::{
        button::{button_ex, Variant},
        kbd::combo,
    },
    demos::common::{card, section_caption, section_title, vspace},
    theme::{AppTheme, Size},
};

pub fn view<'a>(_state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let t = theme;

    let shortcut: Element<'a, Message> = if cfg!(target_os = "macos") {
        combo(t, &["⌘", "K"])
    } else {
        combo(t, &["Ctrl", "K"])
    };

    let trigger_row = row![
        button_ex(
            t,
            "Open palette",
            Variant::Primary,
            Size::Md,
            Some(Message::CommandPaletteOpen),
            false,
            false,
        ),
        Space::new().width(Length::Fixed(16.0)),
        text("or press").size(13.0).color(t.muted_foreground),
        shortcut,
        text("from anywhere").size(13.0).color(t.muted_foreground),
    ]
    .spacing(8)
    .align_y(Vertical::Center);

    let usage = card(
        t,
        column![
            text("How to use").size(14.0).color(t.foreground),
            text("Type to filter component pages by name. Use ↑ ↓ to move the highlight, ⏎ to jump, Esc or click outside to dismiss. Click the chip in the top-right of the app to open it with the mouse.")
                .size(13.0)
                .color(t.muted_foreground),
        ]
        .spacing(8)
        .into(),
    );

    column![
        section_title(t, "Command palette"),
        section_caption(t, "Cmdk-style fuzzy launcher overlaid on top of the app."),
        vspace(8.0),
        trigger_row,
        vspace(16.0),
        usage,
    ]
    .spacing(10)
    .into()
}
