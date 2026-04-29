use iced::{Element, Length, widget::{column, container, row}};

use crate::{
    Message, State,
    components::{
        badge::{badge, BadgeVariant},
        icon::Icon,
        keyboard_focus,
        list::{list, ListItem},
    },
    demos::common::{section_caption, section_title, vspace},
    lucide,
    theme::AppTheme,
};

type SampleRow = (&'static str, &'static str, &'static str, fn() -> Icon);

pub const SAMPLE: &[SampleRow] = &[
    ("Inbox", "12 new messages", "primary", lucide::mail),
    ("Starred", "Pinned items", "secondary", lucide::star),
    ("Drafts", "3 unsent", "warning", lucide::pencil),
    ("Sent", "Last: 2h ago", "muted", lucide::upload),
    ("Archive", "—", "muted", lucide::folder),
    ("Trash", "Empty", "muted", lucide::trash_2),
];

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let selected = state.list_selected;

    let items: Vec<ListItem<'a, Message>> = SAMPLE
        .iter()
        .enumerate()
        .map(|(i, (name, hint, variant, icon_factory))| {
            let v = match *variant {
                "primary" => BadgeVariant::Default,
                "warning" => BadgeVariant::Warning,
                _ => BadgeVariant::Secondary,
            };
            let trailing = match *name {
                "Inbox" => Some(badge::<Message>(theme, "12", v)),
                "Drafts" => Some(badge::<Message>(theme, "3", v)),
                _ => None,
            };
            let mut it = ListItem::new(name.to_string())
                .secondary(hint.to_string())
                .leading_icon(theme, icon_factory())
                .selected(selected == i)
                .on_press(Message::ListSelected(i));
            if let Some(tr) = trailing {
                it = it.trailing(tr);
            }
            it
        })
        .collect();

    let scroll_items: Vec<ListItem<'a, Message>> = (0..80)
        .map(|n| {
            ListItem::new(format!("Item #{n:03}"))
                .secondary("A row in a scrolling list".to_string())
                .selected(selected == 100 + n)
                .on_press(Message::ListSelected(100 + n))
        })
        .collect();

    let nav_list = keyboard_focus::wrap(list(theme, items), Message::ListNav);

    column![
        section_title(theme, "Selectable list"),
        section_caption(
            theme,
            "Rows emit on_press; the parent keeps the selection. Click the left list to focus it, then ↑/↓/Home/End/PgUp/PgDn move the highlight; Enter activates.",
        ),
        row![
            container(nav_list).width(Length::FillPortion(1)),
            container(
                container(list(theme, scroll_items))
                    .max_height(320)
            )
            .width(Length::FillPortion(1)),
        ]
        .spacing(16),
        vspace(8.0),
    ]
    .spacing(10)
    .into()
}
