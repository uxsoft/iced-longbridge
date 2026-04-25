//! Sidebar builder — composable nav sidebar with header, groups, items, and footer.
//!
//! The existing showcase sidebar in `main.rs` remains in place; this module
//! exposes reusable primitives for other sidebars (e.g. inside the Setting
//! component) that want the same look without re-implementing styling.

use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, scrollable, stack, text, Space},
};

use crate::{
    components::{
        icon::{icon, Icon, IconName},
        tooltip::wrap as tooltip_wrap,
    },
    theme::AppTheme,
};

const COLLAPSED_WIDTH: f32 = 56.0;

pub struct Item<Message> {
    pub label: String,
    pub icon: Option<Icon>,
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

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
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
    header_collapsed: Option<Element<'a, Message>>,
    groups: Vec<Group<'a, Message>>,
    footer: Option<Element<'a, Message>>,
    footer_collapsed: Option<Element<'a, Message>>,
    width: f32,
    collapsed: bool,
    on_toggle: Option<Message>,
}

impl<'a, Message: Clone + 'a> Sidebar<'a, Message> {
    pub fn new() -> Self {
        Self {
            header: None,
            header_collapsed: None,
            groups: Vec::new(),
            footer: None,
            footer_collapsed: None,
            width: 240.0,
            collapsed: false,
            on_toggle: None,
        }
    }

    pub fn header(mut self, el: Element<'a, Message>) -> Self {
        self.header = Some(el);
        self
    }

    pub fn header_collapsed(mut self, el: Element<'a, Message>) -> Self {
        self.header_collapsed = Some(el);
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

    pub fn footer_collapsed(mut self, el: Element<'a, Message>) -> Self {
        self.footer_collapsed = Some(el);
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn collapsed(mut self, c: bool) -> Self {
        self.collapsed = c;
        self
    }

    pub fn on_toggle(mut self, msg: Message) -> Self {
        self.on_toggle = Some(msg);
        self
    }

    pub fn view(self, theme: &AppTheme) -> Element<'a, Message> {
        let t = *theme;
        let collapsed = self.collapsed;
        let effective_width = if collapsed { COLLAPSED_WIDTH } else { self.width };

        let mut col = if collapsed {
            column![].spacing(8).padding(Padding::from([12.0, 0.0]))
        } else {
            column![].spacing(12).padding(Padding::from([14.0, 10.0]))
        };

        let header_el = if collapsed { self.header_collapsed } else { self.header };
        if let Some(h) = header_el {
            col = col.push(h);
        }

        let group_count = self.groups.len();
        for (idx, g) in self.groups.into_iter().enumerate() {
            col = col.push(render_group(theme, g, collapsed));
            if collapsed && idx + 1 < group_count {
                col = col.push(
                    container(Space::new().height(Length::Fixed(1.0)).width(Length::Fill))
                        .padding(Padding::from([0.0, 10.0]))
                        .style(move |_| container::Style {
                            background: Some(Background::Color(t.sidebar_border)),
                            text_color: None,
                            border: Border {
                                color: Color::TRANSPARENT,
                                width: 0.0,
                                radius: 0.0.into(),
                            },
                            shadow: Shadow::default(),
                            snap: true,
                        }),
                );
            }
        }

        let body = scrollable(col).height(Length::Fill);
        let mut outer = column![body].width(Length::Fill).height(Length::Fill);

        let footer_el = if collapsed { self.footer_collapsed } else { self.footer };
        let toggle_el = self
            .on_toggle
            .map(|msg| render_toggle(theme, collapsed, msg));

        if footer_el.is_some() || toggle_el.is_some() {
            let mut footer_col = column![].spacing(8);
            if let Some(f) = footer_el {
                footer_col = footer_col.push(f);
            }
            if let Some(t_el) = toggle_el {
                footer_col = footer_col.push(t_el);
            }
            outer = outer.push(
                container(footer_col)
                    .padding(Padding::from([12.0, if collapsed { 6.0 } else { 14.0 }]))
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
            .width(Length::Fixed(effective_width))
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
    collapsed: bool,
) -> Element<'a, Message> {
    let t = *theme;
    let mut c = column![].spacing(if collapsed { 4 } else { 2 });
    if !collapsed && let Some(label) = group.label {
        c = c.push(text(label).size(11.0).color(t.muted_foreground));
        c = c.push(Space::new().height(Length::Fixed(2.0)));
    }
    for item in group.items {
        c = c.push(render_item(theme, item, collapsed));
    }
    if let Some(extra) = group.extra {
        c = c.push(extra);
    }
    c.into()
}

fn render_item<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    item: Item<Message>,
    collapsed: bool,
) -> Element<'a, Message> {
    let t = *theme;
    let active = item.active;
    let label = item.label;
    let glyph = item.icon;
    let badge = item.badge;

    if collapsed {
        render_item_collapsed(theme, label, glyph, badge, active, item.on_press)
    } else {
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
                        _ => (Color::TRANSPARENT, t.sidebar_foreground),
                    }
                };
                button::Style {
                    background: Some(Background::Color(bg)),
                    text_color: fg,
                    border: Border {
                        color: Color::TRANSPARENT,
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
}

fn render_item_collapsed<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    label: String,
    glyph: Option<Icon>,
    badge: Option<String>,
    active: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let t = *theme;

    // Icon or 6px dot fallback, centered in a 40x40 square.
    let icon_el: Element<'a, Message> = match glyph {
        Some(name) => icon(theme, name, 18.0),
        None => container(Space::new().width(Length::Fixed(6.0)).height(Length::Fixed(6.0)))
            .style(move |_| container::Style {
                background: Some(Background::Color(t.muted_foreground)),
                text_color: None,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 3.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            })
            .into(),
    };

    let icon_slot: Element<'a, Message> = container(icon_el)
        .width(Length::Fixed(40.0))
        .height(Length::Fixed(40.0))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();

    // Badge pill overlaid on the top-right corner using `stack!`.
    let content: Element<'a, Message> = if let Some(b) = badge {
        let pill = container(text(b).size(9.0).color(t.danger_foreground))
            .padding(Padding::from([0.0, 5.0]))
            .style(move |_| container::Style {
                background: Some(Background::Color(t.danger)),
                text_color: Some(t.danger_foreground),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 999.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            });

        let pill_layer = container(pill)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Right)
            .align_y(Vertical::Top)
            .padding(Padding::from([2.0, 2.0]));

        stack![icon_slot, pill_layer].into()
    } else {
        icon_slot
    };

    let mut btn = button(content)
        .padding(Padding::from([0.0, 0.0]))
        .style(move |_, status| {
            use button::Status::*;
            let (bg, fg) = if active {
                (t.sidebar_accent, t.foreground)
            } else {
                match status {
                    Hovered => (t.accent, t.foreground),
                    Pressed => (t.muted, t.foreground),
                    _ => (Color::TRANSPARENT, t.sidebar_foreground),
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        });
    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }

    let wrapped = tooltip_wrap(theme, btn.into(), label);

    container(wrapped)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into()
}

fn render_toggle<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    collapsed: bool,
    on_toggle: Message,
) -> Element<'a, Message> {
    let t = *theme;

    let btn: Element<'a, Message> = if collapsed {
        let icon_slot = container(icon(theme, IconName::ChevronRight, 16.0))
            .width(Length::Fixed(40.0))
            .height(Length::Fixed(40.0))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);
        let raw = button(icon_slot)
            .padding(Padding::from([0.0, 0.0]))
            .on_press(on_toggle)
            .style(move |_, status| toggle_style(&t, status));
        tooltip_wrap(theme, raw.into(), "Expand").into()
    } else {
        let inner = row![
            icon(theme, IconName::ChevronLeft, 14.0),
            text("Collapse").size(13.0).color(t.sidebar_foreground),
        ]
        .spacing(10)
        .align_y(Vertical::Center);
        button(inner)
            .padding(Padding::from([6.0, 10.0]))
            .width(Length::Fill)
            .on_press(on_toggle)
            .style(move |_, status| toggle_style(&t, status))
            .into()
    };

    container(btn)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into()
}

fn toggle_style(t: &AppTheme, status: button::Status) -> button::Style {
    use button::Status::*;
    let bg = match status {
        Hovered => t.accent,
        Pressed => t.muted,
        _ => Color::TRANSPARENT,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: t.sidebar_foreground,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}
