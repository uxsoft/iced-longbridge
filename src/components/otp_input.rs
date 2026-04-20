//! OTP input — N single-character boxes that share a value.

use iced::{
    Background, Border, Element, Length, Padding,
    alignment::Vertical,
    widget::{container, row, text_input},
};

use crate::theme::AppTheme;

pub fn otp_input<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    value: &str,
    length: usize,
    on_change: impl Fn(String) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let t = *theme;
    let mut r = row![].spacing(8).align_y(Vertical::Center);
    let chars: Vec<char> = value.chars().collect();
    for i in 0..length {
        let ch = chars.get(i).copied().unwrap_or(' ');
        let current = if ch == ' ' { String::new() } else { ch.to_string() };
        let existing = value.to_string();
        let input = text_input("", &current)
            .on_input(move |new_value| {
                // Take the last typed character (paste handles itself via length)
                let new_chars: Vec<char> = new_value.chars().collect();
                let mut updated: Vec<char> = existing.chars().collect();
                updated.resize(length, ' ');
                if new_chars.is_empty() {
                    updated[i] = ' ';
                } else {
                    let ch = *new_chars.last().unwrap();
                    updated[i] = ch;
                }
                on_change(updated.into_iter().collect::<String>().trim_end().to_string())
            })
            .size(20.0)
            .padding(Padding::from([6.0, 0.0]))
            .align_x(iced::alignment::Horizontal::Center)
            .style(move |_, status| {
                use text_input::Status::*;
                let border = match status {
                    Focused { .. } => t.ring,
                    _ => t.input_border,
                };
                text_input::Style {
                    background: Background::Color(t.background),
                    border: Border {
                        color: border,
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    icon: t.muted_foreground,
                    placeholder: t.muted_foreground,
                    value: t.foreground,
                    selection: crate::theme::with_alpha(t.primary, 0.3),
                }
            });
        r = r.push(
            container(input)
                .width(Length::Fixed(36.0))
                .height(Length::Fixed(40.0)),
        );
    }
    r.into()
}
