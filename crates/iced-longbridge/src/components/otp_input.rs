//! OTP input — N single-character boxes that share a value.
//!
//! Each box is a focusable `text_input` with a deterministic [`Id`] derived
//! from `id_prefix` and its index. The `on_change` callback reports the new
//! full value along with `Some(next_index)` when a character was accepted
//! and focus should advance, or `None` on a clear.
//!
//! The caller advances focus from `update` by returning the [`focus`] task.

use iced::{
    Background, Border, Element, Length, Padding, Task,
    advanced::widget::Id,
    alignment::Vertical,
    widget::{container, operation, row, text_input},
};

use crate::theme::AppTheme;

/// Stable widget [`Id`] for the box at `index` in the OTP group named
/// `id_prefix`. Pair with [`focus`] to move focus to a specific box.
pub fn field_id(id_prefix: &str, index: usize) -> Id {
    Id::from(format!("otp-{id_prefix}-{index}"))
}

/// Focus the OTP box at `index` in group `id_prefix`.
pub fn focus<Message: Send + 'static>(id_prefix: &str, index: usize) -> Task<Message> {
    operation::focus(field_id(id_prefix, index))
}

pub fn otp_input<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    id_prefix: &'a str,
    value: &str,
    length: usize,
    on_change: impl Fn(String, Option<usize>) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let t = *theme;
    let mut r = row![].spacing(8).align_y(Vertical::Center);
    let chars: Vec<char> = value.chars().collect();
    for i in 0..length {
        let ch = chars.get(i).copied().unwrap_or(' ');
        let current = if ch == ' ' { String::new() } else { ch.to_string() };
        let existing = value.to_string();
        let input = text_input("", &current)
            .id(field_id(id_prefix, i))
            .on_input(move |new_value| {
                let new_chars: Vec<char> = new_value.chars().collect();
                let mut updated: Vec<char> = existing.chars().collect();
                updated.resize(length, ' ');
                let advance = if new_chars.is_empty() {
                    updated[i] = ' ';
                    None
                } else {
                    // Typing over a filled field: the last character wins.
                    updated[i] = *new_chars.last().unwrap();
                    (i + 1 < length).then_some(i + 1)
                };
                let new_val = updated
                    .into_iter()
                    .collect::<String>()
                    .trim_end()
                    .to_string();
                on_change(new_val, advance)
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
