//! Pagination — numbered page controls with prev/next.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{button, container, row, text},
};

use crate::{
    components::icon::{icon_colored, IconName},
    theme::AppTheme,
};

pub fn pagination<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    current: usize,
    total: usize,
    on_change: impl Fn(usize) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let t = *theme;
    let mut r = row![].spacing(4).align_y(Vertical::Center);

    let prev_enabled = current > 1;
    r = r.push(arrow_btn(
        t,
        IconName::ChevronLeft,
        prev_enabled.then(|| on_change(current.saturating_sub(1).max(1))),
    ));

    let pages = visible_pages(current, total);
    for p in pages {
        match p {
            PageToken::Num(n) => {
                r = r.push(page_btn(t, n, n == current, on_change(n)));
            }
            PageToken::Ellipsis => {
                r = r.push(text("…").size(13.0).color(t.muted_foreground));
            }
        }
    }

    let next_enabled = current < total;
    r = r.push(arrow_btn(
        t,
        IconName::ChevronRight,
        next_enabled.then(|| on_change((current + 1).min(total.max(1)))),
    ));

    r.into()
}

#[derive(Debug, Clone, Copy)]
enum PageToken {
    Num(usize),
    Ellipsis,
}

fn visible_pages(current: usize, total: usize) -> Vec<PageToken> {
    if total <= 7 {
        return (1..=total).map(PageToken::Num).collect();
    }
    let mut out = Vec::new();
    out.push(PageToken::Num(1));
    if current > 4 {
        out.push(PageToken::Ellipsis);
    }
    let lo = current.saturating_sub(1).max(2);
    let hi = (current + 1).min(total - 1);
    for n in lo..=hi {
        out.push(PageToken::Num(n));
    }
    if current + 3 < total {
        out.push(PageToken::Ellipsis);
    }
    out.push(PageToken::Num(total));
    out
}

fn page_btn<'a, Message: Clone + 'a>(
    t: AppTheme,
    n: usize,
    active: bool,
    on_press: Message,
) -> Element<'a, Message> {
    let content = container(text(n.to_string()).size(13.0))
        .width(Length::Fixed(32.0))
        .height(Length::Fixed(32.0))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);
    button(content)
        .padding(Padding::from(0.0))
        .on_press(on_press)
        .style(move |_, status| {
            use button::Status::*;
            let (bg, fg, border) = if active {
                (t.primary, t.primary_foreground, t.primary)
            } else {
                match status {
                    Hovered => (t.accent, t.foreground, t.border),
                    Pressed => (t.muted, t.foreground, t.border),
                    _ => (t.background, t.foreground, t.border),
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}

fn arrow_btn<'a, Message: Clone + 'a>(
    t: AppTheme,
    glyph: IconName,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let enabled = on_press.is_some();
    let content = container(icon_colored::<Message>(
        glyph,
        14.0,
        if enabled { t.foreground } else { t.muted_foreground },
    ))
    .width(Length::Fixed(32.0))
    .height(Length::Fixed(32.0))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center);
    let mut btn = button(content)
        .padding(Padding::from(0.0))
        .style(move |_, status| {
            use button::Status::*;
            let bg = match status {
                Hovered => t.accent,
                Pressed => t.muted,
                Disabled => t.background,
                _ => t.background,
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: t.foreground,
                border: Border {
                    color: t.border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        });
    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }
    btn.into()
}
