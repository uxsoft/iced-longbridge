# Context Menu + Shared Popover Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Share the menu popover implementation across menu bar, dropdown, and a new context menu widget; tighten padding so items sit flush against the popover border.

**Architecture:** Add a new `popover_panel(theme, content)` helper as the single source of truth for the styled floating box (bg, border, radius, shadow, zero padding). Rewrite `popover_aligned` and `menu_bar::menu_bar` to use it, and strip the outer `.padding(4.0)` from `menu::menu`. Add a new `ContextMenu` custom widget (in `context_menu.rs`) that delegates to a child element, holds `Option<Point>` in iced widget-tree state, intercepts right-click inside the child's bounds, and returns a point-anchored overlay that renders `popover_panel(menu(...))` at the cursor. Dismissal (outside-click, Esc, item-click) is handled inside the overlay and clears the stored point.

**Tech Stack:** Rust, iced 0.14 (widgets: `button`, `container`, `stack!`, `mouse_area`; advanced: custom `Widget` trait + `overlay::Overlay` for the context menu).

**Verification loop:** UI components in this repo have no unit tests — validate with `cargo build -p iced-longbridge -p demo -p demo-app`, `cargo clippy ... -- -D warnings`, and manual inspection via the demo app.

**File map:**

- Modify: `crates/iced-longbridge/src/components/popover.rs` — add `popover_panel`, rewrite `popover_aligned` to use it, remove `.padding([6.0, 8.0])` wrapper.
- Modify: `crates/iced-longbridge/src/components/menu.rs` — remove outer `.padding(4.0)`.
- Modify: `crates/iced-longbridge/src/components/menu_bar.rs` — replace inline styled container with `popover_panel`.
- Create: `crates/iced-longbridge/src/components/context_menu.rs` — new widget, ~230 LOC.
- Modify: `crates/iced-longbridge/src/components/mod.rs` — register `context_menu`.
- Create: `crates/demo/src/demos/context_menu_demo.rs` — demo page.
- Modify: `crates/demo/src/demos/mod.rs` — register demo.
- Modify: `crates/demo/src/lib.rs` — add `Page::ContextMenu`, route, description, nav entry.

---

## Task 1: Shared popover renderer + zero outer padding

Goal: extract the styled popover box into `popover_panel`, route `popover_aligned` and `menu_bar` through it, drop the outer padding layers so items sit flush against the popover border.

**Files:**
- Modify: `crates/iced-longbridge/src/components/popover.rs`
- Modify: `crates/iced-longbridge/src/components/menu.rs`
- Modify: `crates/iced-longbridge/src/components/menu_bar.rs`

- [ ] **Step 1: Replace `crates/iced-longbridge/src/components/popover.rs` in full.**

```rust
//! Popover — trigger that reveals a floating panel anchored to it.
//!
//! The panel is returned through [`FloatingPanel`]'s `Widget::overlay()` so it
//! floats above siblings instead of pushing them down. Used by the dropdown
//! button, time/date pickers, and the color picker.

use iced::{
    Background, Border, Element, Shadow,
    alignment::Horizontal,
    widget::container,
};

use crate::{components::floating_panel::FloatingPanel, theme::AppTheme};

/// Renders `content` inside the shared styled popover box (popover bg, 1px
/// border, 8px radius, shadow, no padding). Used by every menu-style popover
/// (menu bar, dropdown, context menu) and by the time/date/color pickers via
/// [`popover_aligned`].
pub fn popover_panel<'a, Message: 'a>(
    theme: &AppTheme,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    container(content)
        .style(move |_| container::Style {
            background: Some(Background::Color(t.popover)),
            text_color: Some(t.popover_foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

pub fn popover<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    trigger: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    popover_aligned(theme, trigger, panel, Horizontal::Left, None)
}

/// Like [`popover`], but also closes the panel when the user clicks outside
/// it (the `on_dismiss` message fires on any mouse press beyond the panel's
/// bounds — typically the same message that toggled it open).
pub fn popover_dismissable<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    trigger: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
    on_dismiss: Message,
) -> Element<'a, Message> {
    popover_aligned(theme, trigger, panel, Horizontal::Left, Some(on_dismiss))
}

pub fn popover_aligned<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    trigger: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
    align: Horizontal,
    on_dismiss: Option<Message>,
) -> Element<'a, Message> {
    let wrapped = panel.map(|p| popover_panel(theme, p));

    let mut fp = FloatingPanel::new(trigger, wrapped).align(align);
    if let Some(msg) = on_dismiss {
        fp = fp.on_dismiss(msg);
    }
    fp.into()
}
```

- [ ] **Step 2: Remove outer padding in `menu::menu`.**

In `crates/iced-longbridge/src/components/menu.rs`, locate the final lines (around 143–146):

```rust
    container(c)
        .padding(Padding::from(4.0))
        .width(Length::Fixed(220.0))
        .into()
```

Replace with:

```rust
    container(c)
        .width(Length::Fixed(220.0))
        .into()
```

(Drop the `.padding(Padding::from(4.0))` call only. Nothing else in this file changes.)

- [ ] **Step 3: Update `menu_bar.rs` to use `popover_panel`.**

In `crates/iced-longbridge/src/components/menu_bar.rs`:

Change the `use` block to import the new helper and drop the now-unused `styles` + `Padding`:

```rust
use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow,
    alignment::Vertical,
    widget::{button, container, mouse_area, row, text},
};

use crate::{
    components::{
        floating_panel::FloatingPanel,
        menu::{menu, Item},
        popover::popover_panel,
    },
    theme::AppTheme,
};
```

(The `styles` import is gone; `Padding` stays because the outer menu bar `.padding([4.0, 6.0])` still uses it.)

Inside `menu_bar(...)`, replace the existing `panel` construction:

```rust
        let panel = is_open.then(|| {
            container(menu(theme, m.items))
                .padding(Padding::from([6.0, 8.0]))
                .style(move |_| styles::popover_container(&t, 8.0))
                .into()
        });
```

with:

```rust
        let panel = is_open.then(|| popover_panel(theme, menu(theme, m.items)));
```

No other changes in this file.

- [ ] **Step 4: Verify build.**

Run: `cargo build -p iced-longbridge -p demo`
Expected: compiles cleanly.

Run: `cargo clippy -p iced-longbridge -p demo -- -D warnings`
Expected: clean.

If clippy flags unused imports (e.g., `styles` or `Padding`) in a file, remove them inline.

- [ ] **Step 5: Commit.**

```bash
git add crates/iced-longbridge/src/components/popover.rs crates/iced-longbridge/src/components/menu.rs crates/iced-longbridge/src/components/menu_bar.rs
git commit -m "refactor(menu): share popover panel; remove outer padding"
```

---

## Task 2: ContextMenu custom widget

Goal: new `ContextMenu<'a, Message>` builder + widget that opens a menu at the right-click cursor. Internal widget-tree state holds the open position; outside clicks and Esc clear it; clicking an item fires the item's Message and then dismisses the menu.

**Files:**
- Create: `crates/iced-longbridge/src/components/context_menu.rs`
- Modify: `crates/iced-longbridge/src/components/mod.rs`

- [ ] **Step 1: Create `crates/iced-longbridge/src/components/context_menu.rs` with the full widget.**

```rust
//! Context menu — right-click on a child to open a menu at the cursor.
//!
//! Unlike [`menu_bar`](super::menu_bar) and [`dropdown_button`](super::dropdown_button),
//! which expose an `open` flag to the caller, this widget manages its own open
//! state via iced's widget tree — callers hold no `Option<Point>` of their own.
//! Dismissal fires on outside-press, Escape, or any mouse release inside the
//! panel (i.e. after an item action fires).

use iced::{
    Element, Event, Length, Point, Rectangle, Size, Vector,
    advanced::{
        Clipboard, Layout, Shell, Widget,
        layout::{self, Limits},
        mouse,
        overlay::{self, Overlay},
        renderer,
        widget::{self, Tree, tree},
    },
    keyboard,
};

use crate::{
    components::{
        menu::{self, Item},
        popover::popover_panel,
    },
    theme::AppTheme,
};

/// Builder. Call [`Self::view`] to obtain an `Element`.
pub struct ContextMenu<'a, Message> {
    child: Element<'a, Message>,
    items: Vec<Item<Message>>,
}

impl<'a, Message: Clone + 'a> ContextMenu<'a, Message> {
    pub fn new(
        child: impl Into<Element<'a, Message>>,
        items: Vec<Item<Message>>,
    ) -> Self {
        Self {
            child: child.into(),
            items,
        }
    }

    pub fn view(self, theme: &AppTheme) -> Element<'a, Message> {
        let panel = popover_panel(theme, menu::menu(theme, self.items));
        Element::new(ContextMenuWidget {
            child: self.child,
            panel,
        })
    }
}

struct ContextMenuWidget<'a, Message> {
    child: Element<'a, Message>,
    panel: Element<'a, Message>,
}

#[derive(Default)]
struct State {
    open_at: Option<Point>,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer>
    for ContextMenuWidget<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.child), Tree::new(&self.panel)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.child.as_widget(), self.panel.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.child.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.child.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &Limits,
    ) -> layout::Node {
        self.child
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // Intercept right-click inside child bounds → open at cursor.
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) = event {
            if let Some(pt) = cursor.position_over(layout.bounds()) {
                let state = tree.state.downcast_mut::<State>();
                state.open_at = Some(pt);
                shell.capture_event();
                return;
            }
        }

        // Escape dismisses while open.
        if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event {
            if matches!(key, keyboard::Key::Named(keyboard::key::Named::Escape)) {
                let state = tree.state.downcast_mut::<State>();
                if state.open_at.is_some() {
                    state.open_at = None;
                    shell.capture_event();
                    return;
                }
            }
        }

        // Otherwise pass through to child.
        self.child.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.child.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.child.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.child.as_widget_mut().operate(
            &mut tree.children[0],
            layout,
            renderer,
            operation,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        _layout: Layout<'b>,
        _renderer: &iced::Renderer,
        _viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, iced::Theme, iced::Renderer>> {
        // Borrow-split: state (tree.state) and panel_tree (tree.children[1]) disjoint.
        let Tree { state: state_slot, children, .. } = tree;
        let state = state_slot.downcast_mut::<State>();
        let open_at = state.open_at?;
        let panel_tree = &mut children[1];

        let anchor = Point::new(open_at.x + translation.x, open_at.y + translation.y);

        Some(overlay::Element::new(Box::new(ContextMenuOverlay {
            panel: &mut self.panel,
            tree: panel_tree,
            state,
            anchor,
        })))
    }
}

struct ContextMenuOverlay<'a, 'b, Message> {
    panel: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    state: &'b mut State,
    anchor: Point,
}

impl<Message> Overlay<Message, iced::Theme, iced::Renderer>
    for ContextMenuOverlay<'_, '_, Message>
where
    Message: Clone,
{
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let limits = Limits::new(Size::ZERO, bounds);
        let node = self
            .panel
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);
        let panel_size = node.size();

        // Clamp within viewport: if overflow on the right, shift left so the
        // panel's right edge lines up with `anchor.x`; same for bottom.
        let mut x = self.anchor.x;
        if x + panel_size.width > bounds.width {
            x = (self.anchor.x - panel_size.width).max(0.0);
        }
        let mut y = self.anchor.y;
        if y + panel_size.height > bounds.height {
            y = (self.anchor.y - panel_size.height).max(0.0);
        }

        node.move_to(Point::new(x, y))
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let viewport = layout.bounds();
        self.panel.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            &viewport,
        );
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let viewport = layout.bounds();
        let inside_panel = cursor
            .position()
            .is_some_and(|p| layout.bounds().contains(p));

        // Pass all events into the panel (so item buttons can fire their on_press).
        self.panel.as_widget_mut().update(
            self.tree, event, layout, cursor, renderer, clipboard, shell, &viewport,
        );

        // Dismissal rules:
        //  - mouse press outside panel → close, don't consume (element below stays reactive).
        //  - mouse release inside panel → close (item click finished).
        //  - escape → handled by outer widget's update, not here.
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(_)) if !inside_panel => {
                self.state.open_at = None;
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) if inside_panel => {
                self.state.open_at = None;
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let viewport = layout.bounds();
        self.panel.as_widget().mouse_interaction(
            self.tree, layout, cursor, &viewport, renderer,
        )
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.panel
            .as_widget_mut()
            .operate(self.tree, layout, renderer, operation);
    }
}
```

- [ ] **Step 2: Register the module in `crates/iced-longbridge/src/components/mod.rs`.**

Find the existing list of `pub mod ...;` declarations and add `context_menu` alphabetically. The file currently has `pub mod color_picker;` at line 13 followed by `pub mod data_table;` at line 14 — insert `context_menu` between them:

```rust
pub mod color_picker;
pub mod context_menu;
pub mod data_table;
```

- [ ] **Step 3: Verify build.**

Run: `cargo build -p iced-longbridge -p demo`
Expected: compiles cleanly.

If `shell.capture_event()` is missing in your iced 0.14 version, replace those calls with `shell.invalidate_layout()` or simply remove them — captures are an optimization, the event being delivered to the child anyway is not a correctness issue for this widget. If iced's `Overlay::update` signature differs (e.g. `fn on_event`), match it to the local `FloatingPanel::PanelOverlay::update` signature which is proven to work against the installed iced version.

Run: `cargo clippy -p iced-longbridge -p demo -- -D warnings`
Expected: clean.

- [ ] **Step 4: Commit.**

```bash
git add crates/iced-longbridge/src/components/context_menu.rs crates/iced-longbridge/src/components/mod.rs
git commit -m "feat(context-menu): add right-click context menu widget"
```

---

## Task 3: Context menu demo

Goal: a demo page with a bordered "Right-click here" box wrapped in `ContextMenu`. Items fire `Message::MenuAction(String)` (already handled in the demo — writes to `state.last_action`), and a small line below shows the last fired action so users can verify clicks close the menu and fire messages.

**Files:**
- Create: `crates/demo/src/demos/context_menu_demo.rs`
- Modify: `crates/demo/src/demos/mod.rs`
- Modify: `crates/demo/src/lib.rs`

- [ ] **Step 1: Create `crates/demo/src/demos/context_menu_demo.rs`.**

```rust
use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{column, container, text},
};

use crate::{
    Message, State,
    components::{
        context_menu::ContextMenu,
        icon::IconName,
        menu::Item,
    },
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let t = *theme;

    let items = vec![
        Item::new("Cut", Message::MenuAction("Cut".into())).shortcut("⌘X"),
        Item::new("Copy", Message::MenuAction("Copy".into())).shortcut("⌘C"),
        Item::new("Paste", Message::MenuAction("Paste".into())).shortcut("⌘V"),
        Item::Separator,
        Item::new("Delete", Message::MenuAction("Delete".into()))
            .icon(IconName::Trash)
            .danger(),
    ];

    let target = container(
        text("Right-click here").size(14.0).color(theme.muted_foreground),
    )
    .width(Length::Fixed(320.0))
    .height(Length::Fixed(160.0))
    .padding(Padding::from(12.0))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .style(move |_| container::Style {
        background: Some(Background::Color(t.muted)),
        text_color: Some(t.foreground),
        border: Border {
            color: t.border,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    });

    let ctx = ContextMenu::new(target, items).view(theme);

    let last = if state.last_action.is_empty() {
        String::from("Last action: (none yet)")
    } else {
        format!("Last action: {}", state.last_action)
    };

    column![
        section_title(theme, "Context menu"),
        section_caption(theme, "Right-click the area below to open a menu at the cursor."),
        vspace(6.0),
        ctx,
        vspace(6.0),
        text(last).size(12.0).color(theme.muted_foreground),
    ]
    .spacing(8)
    .into()
}
```

- [ ] **Step 2: Register the demo module in `crates/demo/src/demos/mod.rs`.**

Add a `pub mod context_menu_demo;` line. The existing file lists modules alphabetically; insert it between `collapsible_demo` and `color_picker_demo` (or simply after `color_picker_demo` — either position keeps the file tidy). Concretely, change:

```rust
pub mod collapsible_demo;
pub mod color_picker_demo;
```

to:

```rust
pub mod collapsible_demo;
pub mod color_picker_demo;
pub mod context_menu_demo;
```

- [ ] **Step 3: Add `Page::ContextMenu` variant in `crates/demo/src/lib.rs`.**

In the `pub enum Page` block (around line 30), find `MenuBar,` in the Layout section (around line 68). Add `ContextMenu,` directly below it:

```rust
    MenuBar,
    ContextMenu,
```

- [ ] **Step 4: Add the navigation entry to `Page::ALL`.**

Find the existing `(Page::MenuBar, "Menu bar", Category::Layout),` entry (around line 157) in the `Page::ALL` slice. Insert directly after it:

```rust
        (Page::MenuBar, "Menu bar", Category::Layout),
        (Page::ContextMenu, "Context menu", Category::Layout),
```

- [ ] **Step 5: Route the page in `build_content`.**

Find the existing `Page::MenuBar => demos::menu_bar_demo::view(state, t),` match arm (around line 940). Add directly below it:

```rust
        Page::MenuBar => demos::menu_bar_demo::view(state, t),
        Page::ContextMenu => demos::context_menu_demo::view(state, t),
```

- [ ] **Step 6: Add the short description.**

Find the existing `Page::MenuBar => "Horizontal strip of labeled menus — desktop app chrome.",` line (around line 1025) in the `page_description` function. Add directly below:

```rust
        Page::MenuBar => "Horizontal strip of labeled menus — desktop app chrome.",
        Page::ContextMenu => "Right-click anywhere in the child to open a menu at the cursor.",
```

- [ ] **Step 7: Verify build.**

Run: `cargo build -p iced-longbridge -p demo -p demo-app`
Expected: compiles cleanly.

Run: `cargo clippy -p iced-longbridge -p demo -p demo-app -- -D warnings`
Expected: clean. If the `Shadow` / `Color` / `Horizontal` imports are flagged unused, remove the flagged ones from the `use` block at the top of `context_menu_demo.rs`.

- [ ] **Step 8: Visual verification.**

This step requires a display — the implementer should skip this and report it to the human to run manually:

```
cargo run -p demo-app
```

Expected in the demo: navigate to "Context menu" in the Layout section. Right-click inside the bordered "Right-click here" box → menu appears at the cursor with Cut/Copy/Paste/Separator/Delete. Left-click any item → menu closes and "Last action" line updates with the clicked label. Right-click near the right or bottom edge of the window → menu stays on-screen (anchors to its right/bottom edge). Esc while open → menu closes. Click outside the menu (but inside the window) → menu closes.

Also verify the existing pages still work: the dropdown menu on `Page::DropdownMenu` and every menu on `Page::MenuBar` should now have items flush against the popover border (no visible gap between border and first row).

- [ ] **Step 9: Commit.**

```bash
git add crates/demo/src/demos/context_menu_demo.rs crates/demo/src/demos/mod.rs crates/demo/src/lib.rs
git commit -m "feat(demo): add context menu showcase page"
```

---

## Summary of commits

1. `refactor(menu): share popover panel; remove outer padding`
2. `feat(context-menu): add right-click context menu widget`
3. `feat(demo): add context menu showcase page`
