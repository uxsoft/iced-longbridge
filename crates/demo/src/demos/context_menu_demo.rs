use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{column, container, text},
};

use crate::{
    Message, State,
    components::{
        context_menu::ContextMenu,
        menu::Item,
    },
    demos::common::{section_caption, section_title, vspace},
    lucide,
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let t = *theme;

    let items = vec![
        Item::new("Cut", Message::MenuAction("Cut".into())).shortcut("⌘X"),
        Item::new("Copy", Message::MenuAction("Copy".into())).shortcut("⌘C"),
        Item::new("Paste", Message::MenuAction("Paste".into())).shortcut("⌘V"),
        Item::Separator,
        Item::new("Delete", Message::MenuAction("Delete".into()))
            .icon(lucide::trash_2())
            .danger(),
    ];

    let target = container(
        text("Right-click here").size(14.0).color(theme.muted_foreground),
    )
    .width(Length::Fixed(320.0))
    .height(Length::Fixed(160.0))
    .padding(Padding::from(12.0))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .style(move |_| container::Style {
        background: Some(Background::Color(t.muted)),
        text_color: Some(t.foreground),
        border: Border {
            color: t.border,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    });

    let ctx = ContextMenu::new(target, items).view(theme);

    let last = if state.last_action.starts_with("Menu: ") {
        format!("Last action: {}", state.last_action)
    } else {
        String::from("Last action: (right-click to open the menu)")
    };

    column![
        section_title(theme, "Context menu"),
        section_caption(theme, "Right-click the area below to open a menu at the cursor."),
        vspace(6.0),
        ctx,
        vspace(6.0),
        text(last).size(12.0).color(theme.muted_foreground),
    ]
    .spacing(8)
    .into()
}
