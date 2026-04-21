//! Menu bar — horizontal strip of labeled menus used as application chrome.
//!
//! Each entry is a label whose trigger toggles an anchored [`menu`] panel.
//! Caller owns the "which menu is open" state, passed in as `open` and
//! updated via the per-entry `on_toggle` message. Close the bar when any
//! action fires by resetting that state to `None`.

use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow,
    alignment::Vertical,
    widget::{button, container, mouse_area, row, text},
};

use crate::{
    components::{
        floating_panel::FloatingPanel,
        menu::{menu, Item},
    },
    styles,
    theme::AppTheme,
};

pub struct MenuBarMenu<Message> {
    pub label: String,
    pub on_toggle: Message,
    pub items: Vec<Item<Message>>,
}

impl<Message> MenuBarMenu<Message> {
    pub fn new(
        label: impl Into<String>,
        on_toggle: Message,
        items: Vec<Item<Message>>,
    ) -> Self {
        Self {
            label: label.into(),
            on_toggle,
            items,
        }
    }
}

pub fn menu_bar<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    menus: Vec<MenuBarMenu<Message>>,
    open: Option<usize>,
) -> Element<'a, Message> {
    let t = *theme;
    let any_open = open.is_some();
    let mut bar = row![].spacing(2).align_y(Vertical::Top);
    for (i, m) in menus.into_iter().enumerate() {
        let is_open = open == Some(i);
        let toggle = m.on_toggle.clone();
        let mut trigger = menu_bar_trigger(t, &m.label, is_open, toggle);

        // When any menu is open, hovering a different trigger switches to it.
        if any_open && !is_open {
            trigger = mouse_area(trigger).on_enter(m.on_toggle).into();
        }

        let panel = is_open.then(|| {
            container(menu(theme, m.items))
                .padding(Padding::from([6.0, 8.0]))
                .style(move |_| styles::popover_container(&t, 8.0))
                .into()
        });
        bar = bar.push(FloatingPanel::new(trigger, panel).flip(false));
    }

    container(bar)
        .width(Length::Fill)
        .padding(Padding::from([4.0, 6.0]))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.background)),
            text_color: Some(t.foreground),
            border: Border {
                color: t.border,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn menu_bar_trigger<'a, Message: Clone + 'a>(
    t: AppTheme,
    label: &str,
    open: bool,
    on_press: Message,
) -> Element<'a, Message> {
    button(text(label.to_string()).size(13.0).color(t.foreground))
        .padding(Padding::from([4.0, 10.0]))
        .on_press(on_press)
        .style(move |_, status| {
            use button::Status::*;
            let bg = if open {
                t.accent
            } else {
                match status {
                    Hovered => t.accent,
                    Pressed => t.muted,
                    _ => Color::TRANSPARENT,
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: t.foreground,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}
