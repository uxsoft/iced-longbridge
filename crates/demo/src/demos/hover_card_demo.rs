use iced::{
    Background, Border, Element, Padding, Shadow,
    alignment::Vertical,
    widget::{column, container, row, text},
};

use crate::{
    Message,
    components::{
        avatar::initials,
        hover_card::hover_card,
        link::link,
    },
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let t = *theme;

    let trigger_link = link(theme, "@uxsoft", Message::NoOp);
    let body: Element<'a, Message> = column![
        row![
            initials(theme, "JD", 40.0),
            column![
                text("Jan Dryk").size(14.0).color(theme.foreground),
                text("Software engineer. Iced + Rust.").size(12.0).color(theme.muted_foreground),
            ]
            .spacing(2),
        ]
        .spacing(10)
        .align_y(Vertical::Center),
        text("Joined March 2020 • 143 followers").size(11.0).color(theme.muted_foreground),
    ]
    .spacing(8)
    .into();

    let card1 = hover_card(theme, trigger_link, body);

    let trigger_pill = container(text("Hover me").size(13.0).color(theme.foreground))
        .padding(Padding::from([6.0, 14.0]))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.muted)),
            text_color: Some(t.foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 999.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into();

    let rich: Element<'a, Message> = column![
        text("Release notes").size(14.0).color(theme.foreground),
        text("v0.2.0 — new chart subsystem, calendar popover, toggle button.").size(12.0).color(theme.muted_foreground),
    ]
    .spacing(6)
    .into();
    let card2 = hover_card(theme, trigger_pill, rich);

    column![
        section_title(theme, "Hover cards"),
        section_caption(theme, "Hover the trigger to reveal richer content than a tooltip."),
        row![card1, card2].spacing(40).align_y(Vertical::Center),
        vspace(10.0),
    ]
    .spacing(10)
    .into()
}
