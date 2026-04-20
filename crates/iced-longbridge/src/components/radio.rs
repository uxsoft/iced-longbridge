//! Radio button component.

use iced::{
    widget::{radio, Radio},
};

use crate::theme::AppTheme;

pub fn radio_styled<'a, Message: Clone + 'a, V: Copy + Eq>(
    theme: &AppTheme,
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_select: impl FnOnce(V) -> Message + 'a,
) -> Radio<'a, Message> {
    let t = *theme;
    radio(label.into(), value, selected, on_select)
        .size(16)
        .text_size(14)
        .spacing(8)
        .style(move |_, status| {
            use radio::Status::*;
            let (border, dot) = match status {
                Active { is_selected } => {
                    if is_selected {
                        (t.primary, t.primary)
                    } else {
                        (t.border, t.background)
                    }
                }
                Hovered { is_selected } => {
                    if is_selected {
                        (t.primary_hover, t.primary_hover)
                    } else {
                        (t.muted_foreground, t.background)
                    }
                }
            };
            radio::Style {
                background: t.background.into(),
                dot_color: dot,
                border_color: border,
                border_width: 1.5,
                text_color: Some(t.foreground),
            }
        })
}
