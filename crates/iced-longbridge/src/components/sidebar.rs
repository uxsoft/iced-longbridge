//! Sidebar builder — composable nav sidebar with header, groups, items, and footer.
//!
//! The existing showcase sidebar in `main.rs` remains in place; this module
//! exposes reusable primitives for other sidebars (e.g. inside the Setting
//! component) that want the same look without re-implementing styling.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::Vertical,
    widget::{button, column, container, row, scrollable, text, Space},
};

use crate::{
    components::icon::{icon, IconName},
    theme::AppTheme,
};

pub struct Item<Message> {
    pub label: String,
    pub icon: Option<IconName>,
    pub badge: Option<String>,
    pub on_press: Option<Message>,
    pub active: bool,
}

impl<Message> Item<Message> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            badge: None,
            on_press: None,
            active: false,
        }
    }

    pub fn icon(mut self, name: IconName) -> Self {
        self.icon = Some(name);
        self
    }

    pub fn badge(mut self, b: impl Into<String>) -> Self {
        self.badge = Some(b.into());
        self
    }

    pub fn active(mut self, a: bool) -> Self {
        self.active = a;
        self
    }

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }
}

pub struct Group<'a, Message> {
    pub label: Option<String>,
    pub items: Vec<Item<Message>>,
    pub extra: Option<Element<'a, Message>>,
}

impl<'a, Message> Group<'a, Message> {
    pub fn new() -> Self {
        Self {
            label: None,
            items: Vec::new(),
            extra: None,
        }
    }

    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }

    pub fn push(mut self, item: Item<Message>) -> Self {
        self.items.push(item);
        self
    }

    #[allow(dead_code)]
    pub fn extra(mut self, el: Element<'a, Message>) -> Self {
        self.extra = Some(el);
        self
    }
}

impl<'a, Message> Default for Group<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Sidebar<'a, Message> {
    header: Option<Element<'a, Message>>,
    groups: Vec<Group<'a, Message>>,
    footer: Option<Element<'a, Message>>,
    width: f32,
}

impl<'a, Message: Clone + 'a> Sidebar<'a, Message> {
    pub fn new() -> Self {
        Self {
            header: None,
            groups: Vec::new(),
            footer: None,
            width: 240.0,
        }
    }

    pub fn header(mut self, el: Element<'a, Message>) -> Self {
        self.header = Some(el);
        self
    }

    pub fn push(mut self, group: Group<'a, Message>) -> Self {
        self.groups.push(group);
        self
    }

    pub fn footer(mut self, el: Element<'a, Message>) -> Self {
        self.footer = Some(el);
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn view(self, theme: &AppTheme) -> Element<'a, Message> {
        let t = *theme;

        let mut col = column![].spacing(12).padding(Padding::from([14.0, 10.0]));
        if let Some(h) = self.header {
            col = col.push(h);
        }
        for g in self.groups {
            col = col.push(render_group(theme, g));
        }
        let body = scrollable(col).height(Length::Fill);

        let mut outer = column![body].width(Length::Fill).height(Length::Fill);
        if let Some(f) = self.footer {
            outer = outer.push(
                container(f)
                    .padding(Padding::from([12.0, 14.0]))
                    .width(Length::Fill)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(t.sidebar)),
                        text_color: Some(t.sidebar_foreground),
                        border: Border {
                            color: t.sidebar_border,
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        shadow: Shadow::default(),
                        snap: true,
                    }),
            );
        }

        container(outer)
            .width(Length::Fixed(self.width))
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(t.sidebar)),
                text_color: Some(t.sidebar_foreground),
                border: Border {
                    color: t.sidebar_border,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            })
            .into()
    }
}

impl<'a, Message: Clone + 'a> Default for Sidebar<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

fn render_group<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    group: Group<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    let mut c = column![].spacing(2);
    if let Some(label) = group.label {
        c = c.push(text(label).size(11.0).color(t.muted_foreground));
        c = c.push(Space::new().height(Length::Fixed(2.0)));
    }
    for item in group.items {
        c = c.push(render_item(theme, item));
    }
    if let Some(extra) = group.extra {
        c = c.push(extra);
    }
    c.into()
}

fn render_item<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    item: Item<Message>,
) -> Element<'a, Message> {
    let t = *theme;
    let active = item.active;
    let label = item.label;
    let glyph = item.icon;
    let badge = item.badge;

    let mut inner = row![].spacing(10).align_y(Vertical::Center);
    if let Some(name) = glyph {
        inner = inner.push(icon(theme, name, 14.0));
    }
    inner = inner.push(text(label).size(13.0));
    inner = inner.push(Space::new().width(Length::Fill));
    if let Some(b) = badge {
        inner = inner.push(
            container(text(b).size(11.0).color(t.muted_foreground))
                .padding(Padding::from([1.0, 6.0]))
                .style(move |_| container::Style {
                    background: Some(Background::Color(t.muted)),
                    text_color: Some(t.muted_foreground),
                    border: Border {
                        color: t.border,
                        width: 1.0,
                        radius: 999.0.into(),
                    },
                    shadow: Shadow::default(),
                    snap: true,
                }),
        );
    }

    let mut btn = button(inner)
        .padding(Padding::from([6.0, 10.0]))
        .width(Length::Fill)
        .style(move |_, status| {
            use button::Status::*;
            let (bg, fg) = if active {
                (t.sidebar_accent, t.foreground)
            } else {
                match status {
                    Hovered => (t.accent, t.foreground),
                    Pressed => (t.muted, t.foreground),
                    _ => (iced::Color::TRANSPARENT, t.sidebar_foreground),
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        });
    if let Some(msg) = item.on_press {
        btn = btn.on_press(msg);
    }
    btn.into()
}
