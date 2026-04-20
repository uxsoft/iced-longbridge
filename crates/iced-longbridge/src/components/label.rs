//! Label — form field label.

use iced::{
    Element,
    widget::{row, text},
};

use crate::theme::AppTheme;

pub fn label<'a, Message: 'a>(
    theme: &AppTheme,
    content: impl Into<String>,
    required: bool,
) -> Element<'a, Message> {
    let t = *theme;
    let mut r = row![text(content.into()).size(13.0).color(t.foreground)].spacing(2);
    if required {
        r = r.push(text("*").size(13.0).color(t.danger));
    }
    r.into()
}
