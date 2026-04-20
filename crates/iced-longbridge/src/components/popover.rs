//! Popover — trigger that reveals a floating panel.
//!
//! Positioning note: iced 0.14 doesn't expose per-widget bounds at the
//! composition layer, so the popover renders inline directly below its
//! trigger rather than floating freely. The effect is equivalent for
//! trigger-anchored dropdowns, date pickers, and menus.

use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow, Vector,
    alignment::Horizontal,
    widget::{column, container},
};

use crate::theme::AppTheme;

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
    let mut col = column![trigger].spacing(4);
    if let Some(p) = panel {
        let wrapped = container(p)
            .padding(Padding::from([6.0, 8.0]))
            .style(move |_| container::Style {
                background: Some(Background::Color(t.popover)),
                text_color: Some(t.popover_foreground),
                border: Border {
                    color: t.border,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 12.0,
                },
                snap: true,
            });
        col = col.push(wrapped);
    }
    container(col)
        .width(Length::Shrink)
        .align_x(align)
        .into()
}
