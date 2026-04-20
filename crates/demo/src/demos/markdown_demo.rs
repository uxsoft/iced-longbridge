use iced::{
    Element,
    widget::{column, text, Space},
};

use crate::{
    Message, State,
    components::markdown,
    theme::AppTheme,
};

pub const SOURCE: &str = r#"# Markdown

The `markdown` component renders **CommonMark** and *GitHub-Flavored Markdown*
through iced's built-in parser.

## Inline styles

Regular text mixes with **bold**, *italic*, ~~strikethrough~~, `inline code`,
and [external links](https://iced.rs).

## Lists

Unordered:

- Headings up to level 6
- Paragraphs, line breaks
- Emphasis, strong, strikethrough
- Inline and fenced code
- Links and images

Ordered:

1. Parse the source with `markdown::parse`
2. Store the resulting `Vec<Item>` in your state
3. Pass `&items` and a link-click closure to `markdown::markdown`

Task list:

- [x] Parse
- [x] Render
- [ ] Ship

## Blockquote

> Longbridge's design language emphasises calm palettes and tight spacing.
> Inline code like `fn main()` remains distinct from body text.

## Code block

```rust
fn fibonacci(n: u32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let t = a + b;
        a = b;
        b = t;
    }
    a
}
```

## Table

| Symbol | Last   | Change  |
| ------ | -----: | ------: |
| AAPL   | 192.45 | +1.23%  |
| MSFT   | 421.03 | +0.41%  |
| NVDA   | 126.51 | +3.77%  |

---

Horizontal rules separate sections cleanly.
"#;

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    column![
        markdown::markdown(theme, &state.markdown_items, Message::MarkdownLinkClicked),
        Space::new().height(iced::Length::Fixed(8.0)),
        text(format!("Last link: {}", state.markdown_last_link))
            .size(12.0)
            .color(theme.muted_foreground),
    ]
    .spacing(12)
    .into()
}
