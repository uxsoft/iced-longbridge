//! Popover — trigger that reveals a floating panel anchored to it.
//!
//! The panel is returned through [`FloatingPanel`]'s `Widget::overlay()` so it
//! floats above siblings instead of pushing them down. Used by the dropdown
//! button, time/date pickers, and the color picker.

use iced::{
    Background, Border, Element, Shadow,
    alignment::Horizontal,
    widget::container,
};

use crate::{components::floating_panel::FloatingPanel, theme::AppTheme};

/// Renders `content` inside the shared styled popover box (popover bg, 1px
/// border, 8px radius, shadow, no padding). Used by every menu-style popover
/// (menu bar, dropdown, context menu) and by the time/date/color pickers via
/// [`popover_aligned`].
pub fn popover_panel<'a, Message: 'a>(
    theme: &AppTheme,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    container(content)
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
}

pub fn popover<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    trigger: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    popover_aligned(theme, trigger, panel, Horizontal::Left, None)
}

/// Like [`popover`], but also closes the panel when the user clicks outside
/// it (the `on_dismiss` message fires on any mouse press beyond the panel's
/// bounds — typically the same message that toggled it open).
pub fn popover_dismissable<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    trigger: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
    on_dismiss: Message,
) -> Element<'a, Message> {
    popover_aligned(theme, trigger, panel, Horizontal::Left, Some(on_dismiss))
}

pub fn popover_aligned<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    trigger: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
    align: Horizontal,
    on_dismiss: Option<Message>,
) -> Element<'a, Message> {
    let wrapped = panel.map(|p| popover_panel(theme, p));

    let mut fp = FloatingPanel::new(trigger, wrapped).align(align);
    if let Some(msg) = on_dismiss {
        fp = fp.on_dismiss(msg);
    }
    fp.into()
}
