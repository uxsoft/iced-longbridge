# Sidebar Collapse Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let callers render the existing `sidebar` component as an icon-only rail, with an optional built-in toggle button.

**Architecture:** Rewrite `sidebar()` as a `Sidebar<'a, Message>` builder struct in `crates/iced-longbridge/src/components/sidebar.rs`. Add `collapsed: bool`, `on_toggle: Option<Message>`, and alternate-content fields `header_collapsed` / `footer_collapsed`. When `collapsed` is true, the builder renders each item as a 40×40 icon button with a tooltip, hides group labels in favor of dividers, and overlays badges on icon corners using `stack!`. The component stays stateless — callers flip `collapsed` themselves in response to the `on_toggle` message. Only one call site exists (`crates/demo/src/demos/sidebar_demo.rs`); the main showcase nav in `crates/demo/src/lib.rs` builds its own inline column and is untouched.

**Tech Stack:** Rust, iced (widgets: `button`, `column`, `row`, `container`, `stack!`, `tooltip` via `components::tooltip::wrap`, `Space`), existing `IconName` / `AppTheme` from the crate.

**Verification loop:** UI components in this repo have no unit tests; build success (`cargo build -p iced-longbridge -p demo`) plus running the demo app for visual confirmation is the validation pattern. `cargo clippy` should stay clean.

**File map:**

- Modify: `crates/iced-longbridge/src/components/sidebar.rs` — full rewrite to builder + collapsed rendering.
- Modify: `crates/demo/src/demos/sidebar_demo.rs` — switch to builder, add collapsed state, toggle message, alternate header/footer.
- Modify: `crates/demo/src/lib.rs` — add `collapsed_sidebar_demo: bool` to `State`, `Message::ToggleSidebarDemo`, handler in `update`.

---

## Task 1: Introduce `Sidebar` builder struct (API change only, same rendering)

Goal: replace the positional `sidebar(...)` function with a `Sidebar` builder. Output should be visually identical to what exists today. No new behavior.

**Files:**
- Modify: `crates/iced-longbridge/src/components/sidebar.rs`
- Modify: `crates/demo/src/demos/sidebar_demo.rs`

- [ ] **Step 1: Rewrite `crates/iced-longbridge/src/components/sidebar.rs` to a builder.**

Replace the entire file with the following. `Item` and `Group` keep their current API; only the top-level `sidebar()` function is replaced by `Sidebar::new()` with a `.view()` method. The rendering logic in `render_group` / `render_item` stays as-is in this task.

```rust
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
```

Note: `IconName` import is retained even though `render_item` no longer references it directly through the top-level `sidebar` function — `Item::icon` still exposes `IconName` to callers.

- [ ] **Step 2: Update `crates/demo/src/demos/sidebar_demo.rs` to use the builder.**

Replace the entire file with:

```rust
use iced::{
    Element, Length,
    widget::{column, container, row, text},
};

use crate::{
    Message,
    components::{
        icon::IconName,
        sidebar::{Group, Item, Sidebar},
    },
    demos::common::section_title,
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let nav: Element<'a, Message> = Sidebar::new()
        .header(text("Workspace").size(14.0).color(theme.foreground).into())
        .push(
            Group::new()
                .label("Inbox")
                .push(Item::new("All mail").icon(IconName::Mail).badge("12").active(true).on_press(Message::NoOp))
                .push(Item::new("Starred").icon(IconName::Star).on_press(Message::NoOp))
                .push(Item::new("Drafts").icon(IconName::File).badge("2").on_press(Message::NoOp)),
        )
        .push(
            Group::new()
                .label("Folders")
                .push(Item::new("Work").icon(IconName::Folder).on_press(Message::NoOp))
                .push(Item::new("Personal").icon(IconName::Folder).on_press(Message::NoOp))
                .push(Item::new("Archive").icon(IconName::Folder).on_press(Message::NoOp)),
        )
        .push(
            Group::new()
                .label("Account")
                .push(Item::new("Settings").icon(IconName::Settings).on_press(Message::NoOp))
                .push(Item::new("Sign out").icon(IconName::Unlock).on_press(Message::NoOp)),
        )
        .footer(text("v0.1.0").size(11.0).color(theme.muted_foreground).into())
        .width(240.0)
        .view(theme);

    let preview = container(row![nav, text("Main content area").size(14.0).color(theme.muted_foreground)])
        .height(Length::Fixed(420.0));

    column![section_title(theme, "Sidebar builder"), preview]
        .spacing(10)
        .into()
}
```

- [ ] **Step 3: Verify build.**

Run: `cargo build -p iced-longbridge -p demo`
Expected: compiles cleanly, no warnings newer than baseline.

Run: `cargo clippy -p iced-longbridge -p demo -- -D warnings`
Expected: no new clippy errors.

- [ ] **Step 4: Commit.**

```bash
git add crates/iced-longbridge/src/components/sidebar.rs crates/demo/src/demos/sidebar_demo.rs
git commit -m "refactor(sidebar): convert to builder API"
```

---

## Task 2: Add collapse-related builder fields (wiring only, no rendering yet)

Goal: add the `collapsed`, `on_toggle`, `header_collapsed`, `footer_collapsed` fields and their setter methods. Rendering still uses only the expanded path — the new fields have no effect yet. This lets us commit the API surface separately and keeps the next diff focused on rendering.

**Files:**
- Modify: `crates/iced-longbridge/src/components/sidebar.rs`

- [ ] **Step 1: Extend the `Sidebar` struct.**

In `crates/iced-longbridge/src/components/sidebar.rs`, replace the existing `pub struct Sidebar` and its `impl` block with:

```rust
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
        // Temporary: ignore collapsed/on_toggle fields; next task implements them.
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
```

Leave `impl Default for Sidebar`, `Item`, `Group`, `render_group`, `render_item` unchanged.

- [ ] **Step 2: Verify build.**

Run: `cargo build -p iced-longbridge -p demo`
Expected: compiles cleanly. There may be a dead-code warning on the new fields; silence it inline with `#[allow(dead_code)]` attributes on each new field if clippy complains in the next step.

Run: `cargo clippy -p iced-longbridge -p demo -- -D warnings`
Expected: no errors. If clippy warns about unused fields, add `#[allow(dead_code)]` on `header_collapsed`, `footer_collapsed`, `collapsed`, `on_toggle` — those warnings will go away in Task 3 where they get used.

- [ ] **Step 3: Commit.**

```bash
git add crates/iced-longbridge/src/components/sidebar.rs
git commit -m "feat(sidebar): add collapsed/on_toggle builder fields (no rendering yet)"
```

---

## Task 3: Implement collapsed rendering (items, groups, header/footer)

Goal: when `collapsed == true`, render a 56px-wide icon-only rail with tooltips, dot placeholders for icon-less items, badge overlays, and divider-separated groups. Alternate `header_collapsed`/`footer_collapsed` content is used when provided. The toggle button is still deferred to Task 4.

**Files:**
- Modify: `crates/iced-longbridge/src/components/sidebar.rs`

- [ ] **Step 1: Update imports in `crates/iced-longbridge/src/components/sidebar.rs`.**

Replace the existing `use` block at the top of the file with:

```rust
use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, scrollable, stack, text, Space},
};

use crate::{
    components::{
        icon::{icon, IconName},
        tooltip::wrap as tooltip_wrap,
    },
    theme::AppTheme,
};
```

- [ ] **Step 2: Add the `COLLAPSED_WIDTH` constant below the imports.**

Add:

```rust
const COLLAPSED_WIDTH: f32 = 56.0;
```

- [ ] **Step 3: Update `Sidebar::view` to branch on `self.collapsed`.**

Replace the `pub fn view(self, theme: &AppTheme) -> Element<'a, Message>` body with:

```rust
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
        if let Some(f) = footer_el {
            outer = outer.push(
                container(f)
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
```

Note: `self.on_toggle` is still unused in this task. Keep its `#[allow(dead_code)]` attribute if you added one in Task 2.

- [ ] **Step 4: Replace `render_group` to take a `collapsed` flag.**

Replace the existing `render_group` function body with:

```rust
fn render_group<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    group: Group<'a, Message>,
    collapsed: bool,
) -> Element<'a, Message> {
    let t = *theme;
    let mut c = column![].spacing(if collapsed { 4 } else { 2 });
    if !collapsed {
        if let Some(label) = group.label {
            c = c.push(text(label).size(11.0).color(t.muted_foreground));
            c = c.push(Space::new().height(Length::Fixed(2.0)));
        }
    }
    for item in group.items {
        c = c.push(render_item(theme, item, collapsed));
    }
    if let Some(extra) = group.extra {
        c = c.push(extra);
    }
    c.into()
}
```

- [ ] **Step 5: Replace `render_item` to handle both states.**

Replace the existing `render_item` function body with:

```rust
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
}
```

- [ ] **Step 6: Add the `render_item_collapsed` helper at the bottom of the file.**

Append:

```rust
fn render_item_collapsed<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    label: String,
    glyph: Option<IconName>,
    badge: Option<String>,
    active: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let t = *theme;

    // Icon or 6px dot fallback, centered in a 40x40 square.
    let icon_el: Element<'a, Message> = match glyph {
        Some(name) => icon(theme, name, 18.0).into(),
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
            let bg = if active {
                t.sidebar_accent
            } else {
                match status {
                    Hovered => t.accent,
                    Pressed => t.muted,
                    _ => Color::TRANSPARENT,
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: t.sidebar_foreground,
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

    let label_for_tip = label.clone();
    let wrapped = tooltip_wrap(theme, btn.into(), label_for_tip);

    container(wrapped)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into()
}
```

- [ ] **Step 7: Verify build.**

Run: `cargo build -p iced-longbridge -p demo`
Expected: compiles cleanly. If the `Color` import is flagged unused elsewhere in the file, remove any duplicate imports.

Run: `cargo clippy -p iced-longbridge -p demo -- -D warnings`
Expected: no errors.

- [ ] **Step 8: Commit.**

```bash
git add crates/iced-longbridge/src/components/sidebar.rs
git commit -m "feat(sidebar): render collapsed icon-only rail"
```

---

## Task 4: Implement the built-in toggle button

Goal: when `on_toggle` is set, render a toggle row at the very bottom of the sidebar (below any footer content), using `ChevronLeft` + "Collapse" text when expanded, `ChevronRight` icon only (with tooltip "Expand") when collapsed. The button emits `on_toggle` on press.

**Files:**
- Modify: `crates/iced-longbridge/src/components/sidebar.rs`

- [ ] **Step 1: Add the `render_toggle` helper at the bottom of the file.**

Append:

```rust
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
```

- [ ] **Step 2: Integrate the toggle into `Sidebar::view`.**

In `Sidebar::view`, find the block that pushes the footer onto `outer`:

```rust
        let footer_el = if collapsed { self.footer_collapsed } else { self.footer };
        if let Some(f) = footer_el {
            outer = outer.push(
                container(f)
                    .padding(Padding::from([12.0, if collapsed { 6.0 } else { 14.0 }]))
                    ...
            );
        }
```

Replace that whole block (footer-push only — leave the outer-container code below it alone) with:

```rust
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
```

This keeps the styled footer container and folds both the user's footer element and the toggle button into a single column inside it.

- [ ] **Step 3: Remove any `#[allow(dead_code)]` attribute on `on_toggle`.**

The field is now used by `view()`. Drop the attribute if present so we don't hide real dead code later.

- [ ] **Step 4: Verify build.**

Run: `cargo build -p iced-longbridge -p demo`
Expected: compiles cleanly.

Run: `cargo clippy -p iced-longbridge -p demo -- -D warnings`
Expected: no errors.

- [ ] **Step 5: Commit.**

```bash
git add crates/iced-longbridge/src/components/sidebar.rs
git commit -m "feat(sidebar): add built-in collapse/expand toggle button"
```

---

## Task 5: Wire up the demo and verify visually

Goal: the sidebar demo page shows a toggle button, can flip between expanded and collapsed, and uses both `header_collapsed` and `footer_collapsed`. Users can click the chevron to see both states.

**Files:**
- Modify: `crates/demo/src/lib.rs`
- Modify: `crates/demo/src/demos/sidebar_demo.rs`

- [ ] **Step 1: Add demo state to `crates/demo/src/lib.rs`.**

In `crates/demo/src/lib.rs`, find the `State` struct (around line 278). Add this field — group it with the other layout state around the `resizable_state` / `dock_state` lines:

```rust
    pub sidebar_demo_collapsed: bool,
```

In `impl Default for State` (around line 356), initialize it near the other layout defaults:

```rust
            sidebar_demo_collapsed: false,
```

- [ ] **Step 2: Add the message variant.**

In the `pub enum Message` block (around line 193), add under the Layout section (near `DockResized`):

```rust
    SidebarDemoToggle,
```

- [ ] **Step 3: Handle the message in `update`.**

In `pub fn update(state: &mut State, message: Message) -> Task<Message>` (around line 503), add a handler alongside the other layout matches:

```rust
        Message::SidebarDemoToggle => {
            state.sidebar_demo_collapsed = !state.sidebar_demo_collapsed;
            Task::none()
        }
```

- [ ] **Step 4: Update `sidebar_demo.rs` to use the new state.**

The demo view signature needs access to `State`. Check how other demos receive state — some take only `&AppTheme`, others take `&State`. Look at `accordion_demo::view` for an example of a demo that reads `state`.

Find where `sidebar_demo::view` is called in `build_content` (around line 938 in `crates/demo/src/lib.rs`):

```rust
        Page::Sidebar => demos::sidebar_demo::view(t),
```

Replace with:

```rust
        Page::Sidebar => demos::sidebar_demo::view(state),
```

Then rewrite `crates/demo/src/demos/sidebar_demo.rs` to:

```rust
use iced::{
    Element, Length,
    widget::{column, container, row, text},
};

use crate::{
    Message, State,
    components::{
        icon::{icon, IconName},
        sidebar::{Group, Item, Sidebar},
    },
    demos::common::section_title,
};

pub fn view<'a>(state: &'a State) -> Element<'a, Message> {
    let theme = &state.theme;
    let collapsed = state.sidebar_demo_collapsed;

    let nav: Element<'a, Message> = Sidebar::new()
        .header(text("Workspace").size(14.0).color(theme.foreground).into())
        .header_collapsed(icon(theme, IconName::Home, 20.0).into())
        .push(
            Group::new()
                .label("Inbox")
                .push(Item::new("All mail").icon(IconName::Mail).badge("12").active(true).on_press(Message::NoOp))
                .push(Item::new("Starred").icon(IconName::Star).on_press(Message::NoOp))
                .push(Item::new("Drafts").icon(IconName::File).badge("2").on_press(Message::NoOp)),
        )
        .push(
            Group::new()
                .label("Folders")
                .push(Item::new("Work").icon(IconName::Folder).on_press(Message::NoOp))
                .push(Item::new("Personal").icon(IconName::Folder).on_press(Message::NoOp))
                .push(Item::new("Archive").icon(IconName::Folder).on_press(Message::NoOp)),
        )
        .push(
            Group::new()
                .label("Account")
                .push(Item::new("Settings").icon(IconName::Settings).on_press(Message::NoOp))
                .push(Item::new("Sign out").icon(IconName::Unlock).on_press(Message::NoOp)),
        )
        .footer(text("v0.1.0").size(11.0).color(theme.muted_foreground).into())
        .footer_collapsed(text("v0.1").size(10.0).color(theme.muted_foreground).into())
        .width(240.0)
        .collapsed(collapsed)
        .on_toggle(Message::SidebarDemoToggle)
        .view(theme);

    let preview = container(row![nav, text("Main content area").size(14.0).color(theme.muted_foreground)])
        .height(Length::Fixed(420.0));

    column![section_title(theme, "Sidebar builder"), preview]
        .spacing(10)
        .into()
}
```

`IconName::Home` is defined in `crates/iced-longbridge/src/components/icon.rs:25`.

- [ ] **Step 5: Verify build.**

Run: `cargo build -p iced-longbridge -p demo -p demo-app`
Expected: compiles cleanly.

Run: `cargo clippy -p iced-longbridge -p demo -p demo-app -- -D warnings`
Expected: no errors.

- [ ] **Step 6: Visual verification.**

Run: `cargo run -p demo-app`
Expected: the demo-app window opens. Navigate to the "Sidebar" page under the Layout section.
- The sidebar on that page should render at 240px wide with all groups visible.
- At the bottom of the sidebar there should be a "Collapse" button with a left-chevron icon.
- Clicking it collapses the rail to 56px: only icons are visible, the "Workspace" header is replaced by a single icon, groups are separated by a thin horizontal line, "All mail" shows a red "12" badge overlaid on the top-right of its envelope icon, "Drafts" shows a "2" badge. Hovering any icon shows a tooltip with the label. The footer shows "v0.1".
- The toggle button at the bottom is now a single right-chevron icon; hovering it shows an "Expand" tooltip. Clicking it returns the sidebar to the expanded state.

Close the window when satisfied.

- [ ] **Step 7: Commit.**

```bash
git add crates/demo/src/lib.rs crates/demo/src/demos/sidebar_demo.rs
git commit -m "feat(sidebar-demo): showcase collapse/expand toggle"
```

---

## Summary of commits

1. `refactor(sidebar): convert to builder API`
2. `feat(sidebar): add collapsed/on_toggle builder fields (no rendering yet)`
3. `feat(sidebar): render collapsed icon-only rail`
4. `feat(sidebar): add built-in collapse/expand toggle button`
5. `feat(sidebar-demo): showcase collapse/expand toggle`
