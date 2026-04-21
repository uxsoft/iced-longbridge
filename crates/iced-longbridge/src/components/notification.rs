//! Notification — toast stack with auto-dismiss and kind-driven styling.
//!
//! The host owns a [`NotificationList`], calls [`NotificationList::push`] to
//! add entries, and advances [`NotificationList::tick`] once per frame so
//! expired entries disappear. Rendering is a two-step dance:
//!
//! 1. Build the page body as normal.
//! 2. Wrap it in [`notification_layer`] so the toast stack floats over the
//!    page in the configured corner.

use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow, Vector,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, text, Space},
};

use crate::{
    components::{
        icon::{icon_colored, IconName},
        overlay,
    },
    theme::AppTheme,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationKind {
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationKind {
    pub fn icon(self) -> IconName {
        match self {
            NotificationKind::Info => IconName::Info,
            NotificationKind::Success => IconName::CheckCircle,
            NotificationKind::Warning => IconName::Warning,
            NotificationKind::Error => IconName::XCircle,
        }
    }

    pub fn accent(self, t: &AppTheme) -> Color {
        match self {
            NotificationKind::Info => t.info,
            NotificationKind::Success => t.success,
            NotificationKind::Warning => t.warning,
            NotificationKind::Error => t.danger,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u64,
    pub kind: NotificationKind,
    pub title: String,
    pub message: Option<String>,
    /// Remaining lifetime in milliseconds. 0 means "persist until dismissed".
    pub remaining_ms: i64,
}

impl Notification {
    pub fn new(id: u64, kind: NotificationKind, title: impl Into<String>) -> Self {
        Self {
            id,
            kind,
            title: title.into(),
            message: None,
            remaining_ms: 5_000,
        }
    }

    pub fn message(mut self, m: impl Into<String>) -> Self {
        self.message = Some(m.into());
        self
    }

    pub fn ttl_ms(mut self, ms: i64) -> Self {
        self.remaining_ms = ms;
        self
    }

    pub fn sticky(mut self) -> Self {
        self.remaining_ms = 0;
        self
    }
}

#[derive(Debug, Clone)]
pub struct NotificationList {
    pub items: Vec<Notification>,
    pub placement: (Horizontal, Vertical),
    pub max_items: usize,
}

impl NotificationList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            placement: (Horizontal::Right, Vertical::Bottom),
            max_items: 6,
        }
    }

    pub fn placement(mut self, h: Horizontal, v: Vertical) -> Self {
        self.placement = (h, v);
        self
    }

    pub fn push(&mut self, n: Notification) {
        self.items.retain(|e| e.id != n.id);
        self.items.push(n);
        let over = self.items.len().saturating_sub(self.max_items);
        if over > 0 {
            self.items.drain(0..over);
        }
    }

    pub fn dismiss(&mut self, id: u64) {
        self.items.retain(|n| n.id != id);
    }

    /// Decrement all remaining lifetimes and drop expired entries. A remaining
    /// value of 0 is treated as sticky and is never decremented.
    pub fn tick(&mut self, elapsed_ms: i64) {
        for n in self.items.iter_mut() {
            if n.remaining_ms > 0 {
                n.remaining_ms -= elapsed_ms;
            }
        }
        self.items.retain(|n| !n.is_expired());
    }
}

impl Default for NotificationList {
    fn default() -> Self {
        Self::new()
    }
}

impl Notification {
    fn is_expired(&self) -> bool {
        self.remaining_ms < 0
    }
}

/// Wrap `base` with a floating stack of toasts positioned in the configured
/// corner. Returns `base` unchanged when the list is empty.
pub fn notification_layer<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    base: Element<'a, Message>,
    list: &'a NotificationList,
    on_dismiss: impl Fn(u64) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    if list.items.is_empty() {
        return base;
    }

    let mut stack_col = column![].spacing(8);
    // Render newest on top for Top-* placement; bottom for Bottom-* placement.
    let newest_first = matches!(list.placement.1, Vertical::Top);
    let ordered: Vec<&Notification> = if newest_first {
        list.items.iter().rev().collect()
    } else {
        list.items.iter().collect()
    };
    for n in ordered {
        stack_col = stack_col.push(toast(theme, n, on_dismiss));
    }

    let (h, v) = list.placement;
    overlay::anchored(
        base,
        container(stack_col)
            .width(Length::Shrink)
            .padding(Padding::from(0.0))
            .into(),
        h,
        v,
        20.0,
    )
}

fn toast<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    n: &Notification,
    on_dismiss: impl Fn(u64) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let t = *theme;
    let accent = n.kind.accent(&t);
    let id = n.id;

    let mut body = column![
        row![
            icon_colored::<Message>(n.kind.icon(), 16.0, accent),
            text(n.title.clone()).size(13.0).color(t.foreground),
            Space::new().width(Length::Fill),
            close_button(t, on_dismiss(id)),
        ]
        .spacing(8)
        .align_y(Vertical::Center),
    ]
    .spacing(4);

    if let Some(msg) = &n.message {
        body = body.push(text(msg.clone()).size(12.0).color(t.muted_foreground));
    }

    container(body)
        .padding(Padding::from([10.0, 12.0]))
        .width(Length::Fixed(320.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.popover)),
            text_color: Some(t.popover_foreground),
            border: Border {
                color: accent,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 14.0,
            },
            snap: true,
        })
        .into()
}

fn close_button<'a, Message: Clone + 'a>(
    t: AppTheme,
    on_press: Message,
) -> Element<'a, Message> {
    button(icon_colored::<Message>(IconName::Close, 12.0, t.muted_foreground))
        .padding(Padding::from(2.0))
        .on_press(on_press)
        .style(move |_, status| {
            use button::Status::*;
            let bg = match status {
                Hovered => t.muted,
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: t.muted_foreground,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}
