//! Tooltip — wraps another element and shows a popover on hover.

use iced::{
    Element, Padding,
    widget::{container, text, tooltip, Tooltip},
};

use crate::{styles, theme::AppTheme};

pub fn wrap<'a, Message: 'a>(
    theme: &AppTheme,
    content: Element<'a, Message>,
    label: impl Into<String>,
) -> Tooltip<'a, Message> {
    let t = *theme;
    let label = label.into();
    let tip = container(text(label).size(12.0).color(t.popover_foreground))
        .padding(Padding::from([4.0, 8.0]))
        .style(move |_| styles::tooltip_container(&t, 6.0));

    tooltip(content, tip, tooltip::Position::Top)
        .gap(6)
}
