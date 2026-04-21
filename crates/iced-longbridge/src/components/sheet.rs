//! Sheet — slide-in side panel over a backdrop.

use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow, Vector,
    alignment::{Horizontal, Vertical},
    widget::{container, mouse_area, stack, Space},
};

use crate::{styles, theme::AppTheme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn sheet<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    base: Element<'a, Message>,
    open: bool,
    side: Side,
    on_dismiss: Message,
    panel: Element<'a, Message>,
) -> Element<'a, Message> {
    if !open {
        return base;
    }
    let t = *theme;
    let backdrop = mouse_area(
        container(Space::new().width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| styles::simple_container(Some(t.overlay), t.foreground, Color::TRANSPARENT, 0.0, 0.0)),
    )
    .on_press(on_dismiss);

    let panel_body = container(panel)
        .padding(Padding::from([20.0, 24.0]))
        .width(match side {
            Side::Left | Side::Right => Length::Fixed(360.0),
            Side::Top | Side::Bottom => Length::Fill,
        })
        .height(match side {
            Side::Top | Side::Bottom => Length::Fixed(280.0),
            Side::Left | Side::Right => Length::Fill,
        })
        .style(move |_| container::Style {
            background: Some(Background::Color(t.popover)),
            text_color: Some(t.popover_foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 32.0,
            },
            snap: true,
        });

    let (h_align, v_align) = match side {
        Side::Left => (Horizontal::Left, Vertical::Top),
        Side::Right => (Horizontal::Right, Vertical::Top),
        Side::Top => (Horizontal::Left, Vertical::Top),
        Side::Bottom => (Horizontal::Left, Vertical::Bottom),
    };

    let panel_layer = container(panel_body)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(h_align)
        .align_y(v_align);

    stack![base, backdrop, panel_layer].into()
}
