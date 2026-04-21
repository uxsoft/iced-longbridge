//! HTML — lightweight renderer for a subset of HTML tags.
//!
//! Tokenizes common inline and block tags into a Markdown string and then
//! reuses the [`markdown`](super::markdown) pipeline. Supported tags:
//! `h1`–`h6`, `p`, `br`, `hr`, `strong`/`b`, `em`/`i`, `code`, `pre`, `a`,
//! `ul`, `ol`, `li`, `blockquote`. Unknown tags are stripped but their text
//! content is preserved.

use iced::Element;

use crate::{components::markdown, theme::AppTheme};

pub use crate::components::markdown::{Item, Uri};

/// Convert an HTML subset into a Markdown string.
pub fn to_markdown(html: &str) -> String {
    let mut out = String::new();
    let mut tokens = Tokenizer::new(html);
    let mut list_stack: Vec<(ListKind, u32)> = Vec::new();

    while let Some(tok) = tokens.next() {
        match tok {
            Token::Text(s) => out.push_str(&s),
            Token::Open(name, attrs) => match name.as_str() {
                "h1" => out.push_str("\n\n# "),
                "h2" => out.push_str("\n\n## "),
                "h3" => out.push_str("\n\n### "),
                "h4" => out.push_str("\n\n#### "),
                "h5" => out.push_str("\n\n##### "),
                "h6" => out.push_str("\n\n###### "),
                "p" => out.push_str("\n\n"),
                "strong" | "b" => out.push_str("**"),
                "em" | "i" => out.push('*'),
                "code" => out.push('`'),
                "pre" => out.push_str("\n\n```\n"),
                "blockquote" => out.push_str("\n\n> "),
                "ul" => {
                    list_stack.push((ListKind::Unordered, 0));
                    out.push('\n');
                }
                "ol" => {
                    list_stack.push((ListKind::Ordered, 0));
                    out.push('\n');
                }
                "li" => {
                    if let Some((kind, counter)) = list_stack.last_mut() {
                        *counter += 1;
                        out.push('\n');
                        match kind {
                            ListKind::Unordered => out.push_str("- "),
                            ListKind::Ordered => out.push_str(&format!("{}. ", counter)),
                        }
                    } else {
                        out.push_str("\n- ");
                    }
                }
                "a" => {
                    out.push('[');
                    // href resolved on close
                    tokens.pending_href = attrs
                        .iter()
                        .find(|(k, _)| k == "href")
                        .map(|(_, v)| v.clone());
                }
                _ => {}
            },
            Token::Close(name) => match name.as_str() {
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" => out.push('\n'),
                "strong" | "b" => out.push_str("**"),
                "em" | "i" => out.push('*'),
                "code" => out.push('`'),
                "pre" => out.push_str("\n```\n"),
                "blockquote" => out.push('\n'),
                "ul" | "ol" => {
                    list_stack.pop();
                    out.push('\n');
                }
                "a" => {
                    let href = tokens.pending_href.take().unwrap_or_default();
                    out.push_str(&format!("]({})", href));
                }
                _ => {}
            },
            Token::SelfClose(name) => match name.as_str() {
                "br" => out.push_str("  \n"),
                "hr" => out.push_str("\n\n---\n\n"),
                _ => {}
            },
        }
    }

    out
}

/// Parse an HTML subset into Markdown items ready for [`render`].
pub fn parse(html: &str) -> Vec<Item> {
    let md = to_markdown(html);
    markdown::parse(&md).collect()
}

/// Render parsed items with the app theme.
pub fn render<'a, Message: 'a>(
    theme: &AppTheme,
    items: &'a [Item],
    on_link_click: impl Fn(Uri) -> Message + 'a,
) -> Element<'a, Message> {
    markdown::markdown(theme, items, on_link_click)
}

#[derive(Debug, Clone, Copy)]
enum ListKind {
    Unordered,
    Ordered,
}

#[derive(Debug, Clone)]
enum Token {
    Text(String),
    Open(String, Vec<(String, String)>),
    Close(String),
    SelfClose(String),
}

struct Tokenizer<'a> {
    src: &'a str,
    pos: usize,
    pending_href: Option<String>,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &'a str) -> Self {
        Self { src, pos: 0, pending_href: None }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let bytes = self.src.as_bytes();
        if self.pos >= bytes.len() {
            return None;
        }

        if bytes[self.pos] == b'<' {
            // Try to parse a tag; if it doesn't parse, fall through to text.
            if let Some((tok, new_pos)) = parse_tag(self.src, self.pos) {
                self.pos = new_pos;
                return Some(tok);
            }
        }

        let start = self.pos;
        while self.pos < bytes.len() && bytes[self.pos] != b'<' {
            self.pos += 1;
        }
        let text = decode_entities(&self.src[start..self.pos]);
        if text.is_empty() {
            self.next()
        } else {
            Some(Token::Text(text))
        }
    }
}

fn parse_tag(src: &str, pos: usize) -> Option<(Token, usize)> {
    let end = src[pos..].find('>').map(|i| pos + i)?;
    let inner = &src[pos + 1..end];
    if inner.is_empty() {
        return None;
    }

    let (is_close, body) = if let Some(stripped) = inner.strip_prefix('/') {
        (true, stripped)
    } else {
        (false, inner)
    };
    let (body, self_close) = if let Some(stripped) = body.strip_suffix('/') {
        (stripped.trim_end(), true)
    } else {
        (body, false)
    };

    // Extract tag name.
    let name_end = body.find(|c: char| c.is_whitespace()).unwrap_or(body.len());
    let name = body[..name_end].trim().to_ascii_lowercase();
    if name.is_empty() || !name.chars().next().unwrap().is_ascii_alphabetic() {
        return None;
    }

    let attrs = parse_attrs(&body[name_end..]);
    let new_pos = end + 1;

    // <br>, <hr>, <img> are implicitly self-closing even without the slash.
    let implicitly_void = matches!(name.as_str(), "br" | "hr" | "img" | "input");

    let tok = if is_close {
        Token::Close(name)
    } else if self_close || implicitly_void {
        Token::SelfClose(name)
    } else {
        Token::Open(name, attrs)
    };
    Some((tok, new_pos))
}

fn parse_attrs(s: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut chars = s.chars().peekable();
    loop {
        // Skip whitespace.
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }
        let mut key = String::new();
        while let Some(&c) = chars.peek() {
            if c == '=' || c.is_whitespace() || c == '>' {
                break;
            }
            key.push(c.to_ascii_lowercase());
            chars.next();
        }
        if key.is_empty() {
            break;
        }

        // Value is optional.
        let mut value = String::new();
        if matches!(chars.peek(), Some(&'=')) {
            chars.next();
            let quote = chars.peek().copied();
            match quote {
                Some('"') | Some('\'') => {
                    let q = chars.next().unwrap();
                    for c in chars.by_ref() {
                        if c == q {
                            break;
                        }
                        value.push(c);
                    }
                }
                _ => {
                    while let Some(&c) = chars.peek() {
                        if c.is_whitespace() || c == '>' {
                            break;
                        }
                        value.push(c);
                        chars.next();
                    }
                }
            }
        }
        out.push((key, decode_entities(&value)));
    }
    out
}

fn decode_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}
