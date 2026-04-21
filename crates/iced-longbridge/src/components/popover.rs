//! Popover — trigger that reveals a floating panel anchored to it.
//!
//! The panel is returned through [`FloatingPanel`]'s `Widget::overlay()` so it
//! floats above siblings instead of pushing them down. Used by the dropdown
//! button, time/date pickers, and the color picker.

use iced::{
    Background, Border, Element, Padding, Shadow,
    alignment::Horizontal,
    widget::container,
};

use crate::{components::floating_panel::FloatingPanel, theme::AppTheme};

pub fn popover<'a, Message: 'a>(
    theme: &AppTheme,
    trigger: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    popover_aligned(theme, trigger, panel, Horizontal::Left)
}

pub fn popover_aligned<'a, Message: 'a>(
    theme: &AppTheme,
    trigger: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
    align: Horizontal,
) -> Element<'a, Message> {
    let t = *theme;
    let wrapped = panel.map(|p| {
        container(p)
            .padding(Padding::from([6.0, 8.0]))
            .style(move |_| container::Style {
                background: Some(Background::Color(t.popover)),
                text_color: Some(t.popover_foreground),
                border: Border {
                    color: t.border,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            })
            .into()
    });

    FloatingPanel::new(trigger, wrapped).align(align).into()
}
