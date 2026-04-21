//! Text view — read-only selectable text built on `iced::widget::text_editor`.
//!
//! Unlike [`markdown`](super::markdown), the body is plain text with no
//! formatting; unlike [`input`](super::input), the content never changes. The
//! host is expected to drop `Action::Edit(_)` in its update loop — use
//! [`is_editing`] to detect those.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    widget::{container, text_editor},
};

use crate::theme::{AppTheme, Size};

pub use iced::widget::text_editor::Content;

/// Build a read-only `Content` from a source string.
pub fn content_from(source: &str) -> Content {
    Content::with_text(source)
}

/// Returns true when the action would mutate the buffer.
///
/// The host should ignore such actions to keep the view read-only:
/// `.on_action(|a| if text_view::is_editing(&a) { Message::NoOp } else { Message::Action(a) })`.
pub fn is_editing(action: &text_editor::Action) -> bool {
    matches!(action, text_editor::Action::Edit(_))
}

pub fn text_view<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    content: &'a Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> Element<'a, Message> {
    text_view_sized(theme, Size::Md, content, on_action)
}

pub fn text_view_sized<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    size: Size,
    content: &'a Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> Element<'a, Message> {
    let t = *theme;
    let radius = size.radius();
    let font_size = size.font_size();

    let editor = text_editor(content)
        .size(font_size)
        .padding(Padding::from([8.0, 10.0]))
        .on_action(on_action)
        .style(move |_, status| editor_style(&t, status, radius));

    container(editor)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(t.background)),
            text_color: Some(t.foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: radius.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn editor_style(t: &AppTheme, status: text_editor::Status, radius: f32) -> text_editor::Style {
    use text_editor::Status::*;
    let border = match status {
        Focused { .. } => t.ring,
        Hovered => t.muted_foreground,
        _ => t.input_border,
    };
    text_editor::Style {
        background: Background::Color(t.background),
        border: Border {
            color: border,
            width: 1.0,
            radius: radius.into(),
        },
        placeholder: t.muted_foreground,
        value: t.foreground,
        selection: crate::theme::with_alpha(t.primary, 0.3),
    }
}
