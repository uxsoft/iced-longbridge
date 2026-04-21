//! Overlay — stacks content on top of a base element with an optional dim backdrop.
//!
//! Used by dialog, popover, sheet, notification, and the dropdown/context menus.
//! Built on iced's `stack` widget so everything remains z-ordered correctly.

use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{container, stack, Space},
};

use crate::{styles, theme::AppTheme};

/// Wrap `base` with an overlay that only renders when `shown` is true.
///
/// The overlay panel is centred by default. The backdrop dims the whole surface
/// and emits `on_backdrop_press` when clicked (for dialog dismissal).
pub fn overlay<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    base: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
    on_backdrop_press: Option<Message>,
) -> Element<'a, Message> {
    let Some(panel) = panel else {
        return base;
    };
    let t = *theme;

    let backdrop = iced::widget::mouse_area(
        container(Space::new().width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| {
                styles::simple_container(Some(t.overlay), t.foreground, Color::TRANSPARENT, 0.0, 0.0)
            }),
    );
    let backdrop = if let Some(msg) = on_backdrop_press {
        backdrop.on_press(msg)
    } else {
        backdrop
    };

    let panel_layer = container(panel)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);

    stack![base, backdrop, panel_layer].into()
}

/// Render `panel` anchored to a corner of the surface with an offset.
/// Used for toast stacks (bottom-right) and sheet side panels.
pub fn anchored<'a, Message: 'a>(
    base: Element<'a, Message>,
    panel: Element<'a, Message>,
    horizontal: Horizontal,
    vertical: Vertical,
    padding: f32,
) -> Element<'a, Message> {
    let panel_layer = container(panel)
        .padding(Padding::from(padding))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(horizontal)
        .align_y(vertical);
    stack![base, panel_layer].into()
}

/// Standard panel container used by dialogs and popovers.
pub fn panel<'a, Message: 'a>(
    theme: &AppTheme,
    content: Element<'a, Message>,
    max_width: f32,
) -> Element<'a, Message> {
    let t = *theme;
    container(content)
        .padding(Padding::from([20.0, 24.0]))
        .max_width(max_width)
        .style(move |_| container::Style {
            background: Some(Background::Color(t.popover)),
            text_color: Some(t.popover_foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}
