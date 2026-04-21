//! Hover card — rich, larger-than-tooltip hover content.
//!
//! Backed by iced's `tooltip` widget with a themed content panel.

use iced::{
    Element, Padding,
    widget::{container, tooltip},
};

use crate::{styles, theme::AppTheme};

pub fn hover_card<'a, Message: 'a>(
    theme: &AppTheme,
    trigger: Element<'a, Message>,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    let panel = container(content)
        .padding(Padding::from([12.0, 14.0]))
        .max_width(320)
        .style(move |_| styles::popover_container(&t, 8.0));
    tooltip(trigger, panel, tooltip::Position::Top)
        .gap(6.0)
        .into()
}
