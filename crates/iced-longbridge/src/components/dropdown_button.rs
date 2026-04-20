//! Dropdown button — a button with a trailing chevron that toggles a menu.

use iced::{
    Background, Border, Element, Padding, Shadow,
    alignment::Vertical,
    widget::{button, row, text},
};

use crate::{
    components::{
        icon::{icon, IconName},
        menu::{menu, Item},
        popover::popover,
    },
    theme::{AppTheme, Size},
};

pub fn dropdown_button<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    label: impl Into<String>,
    open: bool,
    on_toggle: Message,
    items: Vec<Item<Message>>,
) -> Element<'a, Message> {
    dropdown_button_sized(theme, Size::Md, label, open, on_toggle, items)
}

pub fn dropdown_button_sized<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    size: Size,
    label: impl Into<String>,
    open: bool,
    on_toggle: Message,
    items: Vec<Item<Message>>,
) -> Element<'a, Message> {
    let t = *theme;
    let radius = size.radius();
    let text_size = size.font_size();
    let px = size.padding_x();
    let h = size.height();
    let label = label.into();

    let trigger = button(
        row![
            text(label).size(text_size).align_y(Vertical::Center),
            icon(theme, if open { IconName::ChevronUp } else { IconName::ChevronDown }, text_size),
        ]
        .spacing(8)
        .align_y(Vertical::Center),
    )
    .padding(Padding::from([0.0, px]))
    .height(iced::Length::Fixed(h))
    .on_press(on_toggle)
    .style(move |_, status| {
        use button::Status::*;
        let (bg, fg) = match status {
            Hovered => (t.accent, t.foreground),
            Pressed => (t.muted, t.foreground),
            _ => (t.background, t.foreground),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: fg,
            border: Border {
                color: t.border,
                width: 1.0,
                radius: radius.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        }
    });

    let menu_panel = open.then(|| menu(theme, items));
    popover(theme, trigger.into(), menu_panel)
}
