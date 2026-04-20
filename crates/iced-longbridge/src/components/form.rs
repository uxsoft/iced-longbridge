//! Form — label / input / help-text layout helper.

use iced::{
    Element, Length,
    widget::{column, text},
};

use crate::theme::AppTheme;

pub struct Field<'a, Message> {
    pub label: String,
    pub required: bool,
    pub input: Element<'a, Message>,
    pub help: Option<String>,
    pub error: Option<String>,
}

impl<'a, Message> Field<'a, Message> {
    pub fn new(label: impl Into<String>, input: Element<'a, Message>) -> Self {
        Self {
            label: label.into(),
            required: false,
            input,
            help: None,
            error: None,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn help(mut self, h: impl Into<String>) -> Self {
        self.help = Some(h.into());
        self
    }

    pub fn error(mut self, e: impl Into<String>) -> Self {
        self.error = Some(e.into());
        self
    }
}

pub fn field<'a, Message: 'a>(
    theme: &AppTheme,
    field: Field<'a, Message>,
) -> Element<'a, Message> {
    let label_text = if field.required {
        format!("{} *", field.label)
    } else {
        field.label
    };
    let label_color = if field.required { theme.danger } else { theme.foreground };
    let mut c = column![
        text(label_text).size(13.0).color(label_color),
        field.input,
    ]
    .spacing(6)
    .width(Length::Fill);
    if let Some(e) = field.error {
        c = c.push(text(e).size(12.0).color(theme.danger));
    } else if let Some(h) = field.help {
        c = c.push(text(h).size(12.0).color(theme.muted_foreground));
    }
    c.into()
}

pub fn form<'a, Message: 'a>(
    theme: &AppTheme,
    fields: Vec<Field<'a, Message>>,
) -> Element<'a, Message> {
    let mut c = column![].spacing(14).width(Length::Fill);
    for f in fields {
        c = c.push(field(theme, f));
    }
    c.into()
}
