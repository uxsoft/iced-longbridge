//! Code editor — `text_editor` with syntect-powered syntax highlighting.
//!
//! Requires the `highlighter` feature of `iced`. The highlighter theme tracks
//! the app's light / dark appearance so the editor blends into the surrounding
//! UI.

use iced::{
    Background, Border, Element, Font, Length, Padding, Shadow,
    highlighter,
    widget::{container, text_editor},
};

use crate::theme::{AppTheme, Appearance, Size};

pub use iced::widget::text_editor::Content;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Json,
    Toml,
    Markdown,
    Python,
    JavaScript,
    Html,
    Css,
    Shell,
}

impl Language {
    pub fn extension(self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::Json => "json",
            Language::Toml => "toml",
            Language::Markdown => "md",
            Language::Python => "py",
            Language::JavaScript => "js",
            Language::Html => "html",
            Language::Css => "css",
            Language::Shell => "sh",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::Json => "JSON",
            Language::Toml => "TOML",
            Language::Markdown => "Markdown",
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::Html => "HTML",
            Language::Css => "CSS",
            Language::Shell => "Shell",
        }
    }

    pub const ALL: &'static [Language] = &[
        Language::Rust,
        Language::Json,
        Language::Toml,
        Language::Markdown,
        Language::Python,
        Language::JavaScript,
        Language::Html,
        Language::Css,
        Language::Shell,
    ];
}

pub fn code_editor<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    content: &'a Content,
    language: Language,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> Element<'a, Message> {
    code_editor_sized(theme, Size::Md, content, language, on_action)
}

pub fn code_editor_sized<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    size: Size,
    content: &'a Content,
    language: Language,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> Element<'a, Message> {
    let t = *theme;
    let radius = size.radius();
    let font_size = size.font_size();
    let highlighter_theme = match t.appearance {
        Appearance::Light => highlighter::Theme::InspiredGitHub,
        Appearance::Dark => highlighter::Theme::Base16Mocha,
    };

    let editor = text_editor(content)
        .size(font_size)
        .font(Font::MONOSPACE)
        .padding(Padding::from([10.0, 12.0]))
        .on_action(on_action)
        .highlight(language.extension(), highlighter_theme)
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
