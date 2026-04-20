//! Rating — clickable star rating.

use iced::{
    Background, Border, Element, Padding, Shadow,
    widget::{button, row, text},
};

use crate::theme::AppTheme;

pub fn rating<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    value: u8,
    max: u8,
    on_change: impl Fn(u8) -> Message + 'a,
) -> Element<'a, Message> {
    let t = *theme;
    let mut r = row![].spacing(2);
    for i in 1..=max {
        let filled = i <= value;
        let glyph = if filled { "★" } else { "☆" };
        let color = if filled { t.warning } else { t.muted_foreground };
        let on_press_msg = on_change(i);
        r = r.push(
            button(text(glyph).size(20.0).color(color))
                .padding(Padding::from([0.0, 2.0]))
                .on_press(on_press_msg)
                .style(move |_, status| {
                    use button::Status::*;
                    let bg = match status {
                        Hovered => crate::theme::with_alpha(t.muted, 0.5),
                        _ => iced::Color::TRANSPARENT,
                    };
                    button::Style {
                        background: Some(Background::Color(bg)),
                        text_color: color,
                        border: Border {
                            color: iced::Color::TRANSPARENT,
                            width: 0.0,
                            radius: 4.0.into(),
                        },
                        shadow: Shadow::default(),
                        snap: true,
                    }
                }),
        );
    }
    r.into()
}
