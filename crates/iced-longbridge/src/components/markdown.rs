//! Markdown — themed rich-text rendering built on `iced::widget::markdown`.
//!
//! Supports CommonMark + GFM via pulldown-cmark: headings, paragraphs,
//! bold / italic / strikethrough, inline code, links, images, blockquotes,
//! lists, tables, horizontal rules, and fenced code blocks.
//!
//! Parse once (with [`parse`] or [`Content`]) and keep the resulting items
//! in your app state. Then render them with [`view`] or the [`markdown`]
//! convenience:
//!
//! ```ignore
//! let items: Vec<markdown::Item> = markdown::parse("# hi\n\nsome **text**").collect();
//! let element = markdown::markdown(theme, &items, Message::MarkdownLinkClicked);
//! ```

use iced::{
    Background, Border, Element, Font, Padding,
    advanced::text::{Highlight, Renderer as TextRenderer},
    widget::markdown as iced_markdown,
};

use crate::theme::AppTheme;

pub use iced::widget::markdown::{Content, Item, Settings, Style, Uri, parse};

/// Render a list of parsed Markdown [`Item`]s using the app theme.
///
/// Link clicks are passed to `on_link_click` as a [`Uri`] (string).
pub fn markdown<'a, Message: 'a>(
    theme: &AppTheme,
    items: &'a [Item],
    on_link_click: impl Fn(Uri) -> Message + 'a,
) -> Element<'a, Message> {
    view(theme, items, on_link_click)
}

/// Same as [`markdown`], with an explicit base text size in pixels.
pub fn markdown_sized<'a, Message: 'a>(
    theme: &AppTheme,
    items: &'a [Item],
    text_size: f32,
    on_link_click: impl Fn(Uri) -> Message + 'a,
) -> Element<'a, Message> {
    let settings = Settings::with_text_size(text_size, style(theme));
    iced_markdown::view(items, settings).map(on_link_click)
}

/// Lower-level view helper mirroring `iced::widget::markdown::view`, but
/// themed and generic in the renderer.
pub fn view<'a, Message, Theme, Renderer>(
    theme: &AppTheme,
    items: &'a [Item],
    on_link_click: impl Fn(Uri) -> Message + 'a,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: iced_markdown::Catalog + 'a,
    Renderer: TextRenderer<Font = Font> + 'a,
{
    iced_markdown::view(items, settings(theme)).map(on_link_click)
}

/// Build [`Settings`] whose inline-code / link colors pick up the app theme.
///
/// Heading sizes follow iced's default ratios (h1 = 2×, h2 = 1.75×, …).
pub fn settings(theme: &AppTheme) -> Settings {
    Settings::with_text_size(16.0, style(theme))
}

/// Build a [`Style`] using colors from the app theme.
pub fn style(theme: &AppTheme) -> Style {
    Style {
        font: Font::default(),
        inline_code_padding: Padding::from([1.0, 4.0]),
        inline_code_highlight: Highlight {
            background: Background::Color(theme.muted),
            border: Border {
                color: theme.border,
                width: 0.0,
                radius: 4.0.into(),
            },
        },
        inline_code_color: theme.foreground,
        inline_code_font: Font::MONOSPACE,
        code_block_font: Font::MONOSPACE,
        link_color: theme.link,
    }
}
