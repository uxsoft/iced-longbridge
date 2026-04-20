use iced::{Element, widget::column};

use crate::{
    Message,
    components::description_list::{description_list, description_list_sized, DescItem},
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let profile = vec![
        DescItem::new("Name", "Ada Lovelace"),
        DescItem::new("Role", "Principal Engineer, Analytical Engines"),
        DescItem::new("Joined", "10 December 1815"),
        DescItem::new("Email", "ada@lovelace.io"),
        DescItem::new("Location", "London, UK"),
    ];

    let settings = vec![
        DescItem::new("Build profile", "dev (opt-level = 1, deps opt-level = 3)"),
        DescItem::new("Edition", "2024"),
        DescItem::new("MSRV", "1.80"),
        DescItem::new("Target", "x86_64-unknown-linux-gnu"),
    ];

    column![
        section_title(theme, "Profile"),
        section_caption(theme, "Aligned term/definition pairs."),
        description_list::<Message>(theme, profile),
        vspace(18.0),
        section_title(theme, "Wider term column"),
        description_list_sized::<Message>(theme, 220.0, settings),
    ]
    .spacing(10)
    .into()
}
