# Context Menu + Shared Popover — Design

## Problem

Menu rendering is inconsistent and over-padded:

- `menu_bar.rs` re-implements the popover styling inline instead of using `popover.rs`.
- Three stacked padding layers (popover outer, menu inner, item button) create a visibly thick gap between the menu border and the first item.
- No context menu (right-click) component exists.

## Goals

- One shared popover panel renderer used by the menu bar, dropdown button, and new context menu.
- Compact look: menu items sit flush against the popover border.
- New `ContextMenu` widget that opens a menu at the right-click cursor position, with internal state (callers don't track a `Point`).

## Non-goals

- Nested submenus.
- Keyboard navigation of menu items (arrow keys, Enter to select).
- Touch / long-press triggering.
- Animation of open/close.

## Shared popover renderer

Add to `crates/iced-longbridge/src/components/popover.rs`:

```rust
pub fn popover_panel<'a, Message: 'a>(
    theme: &AppTheme,
    content: Element<'a, Message>,
) -> Element<'a, Message>
```

Returns the styled floating box: `theme.popover` background, 1px `theme.border`, 8px corner radius, shadow. **No padding** around `content` — the caller's content determines the edge-to-edge look.

Existing `popover_aligned()` / `popover_dismissable()` are rewritten to wrap their panel in `popover_panel()` instead of an inline styled container, and the `.padding([6.0, 8.0])` wrapper is removed. Other callers (time picker, date picker, color picker) continue to work via these existing functions and automatically adopt the tighter look.

## Padding changes

Zero out the two outer padding layers so menu items sit flush to the popover border:

1. **Popover outer wrapper** (currently `.padding([6.0, 8.0])` inside `popover_aligned`): removed.
2. **Menu items container** (currently `.padding(4.0)` around the items column in `menu::menu`): removed. The returned element becomes `container(column_of_items).width(Length::Fixed(220.0))` with no outer padding.
3. **Item button padding** (`.padding([6.0, 12.0])`): kept unchanged. Provides per-row breathing room.

## `menu_bar` refactor

Replace the inlined styled container in `menu_bar.rs`:

```rust
// before
container(menu(theme, m.items))
    .padding(Padding::from([6.0, 8.0]))
    .style(move |_| styles::popover_container(&t, 8.0))
    .into()
```

with a call to the shared helper:

```rust
popover_panel(theme, menu(theme, m.items))
```

The outer `FloatingPanel::new(trigger, panel).flip(false)` stays. The menu bar container's own `.padding([4.0, 6.0])` (the bar chrome itself, not the popover) stays.

## `dropdown_button` refactor

No source-level changes — it already calls `popover_dismissable`, which picks up the new look automatically.

## `ContextMenu` widget

New file `crates/iced-longbridge/src/components/context_menu.rs`.

### API

```rust
pub struct ContextMenu<'a, Message> {
    child: Element<'a, Message>,
    items: Vec<menu::Item<Message>>,
}

impl<'a, Message: Clone + 'a> ContextMenu<'a, Message> {
    pub fn new(
        child: impl Into<Element<'a, Message>>,
        items: Vec<menu::Item<Message>>,
    ) -> Self;

    pub fn view(self, theme: &AppTheme) -> Element<'a, Message>;
}
```

`view(theme)` returns the wrapped `Element` ready to drop into any layout. Callers hold no open-state for the menu.

### Internal state

Widget-tree state kept by iced:

```rust
#[derive(Default)]
struct State {
    open_at: Option<Point>, // absolute-coordinate panel origin
}
```

### Behavior

The widget mirrors `FloatingPanel`'s structure for child delegation:
- `layout`, `size`, `draw`, `mouse_interaction`, `operate` all pass through to the child.
- `children` returns the child tree; the overlay is produced lazily.

**Event interception in `update`:**
- On `Mouse(ButtonPressed(Right))` with cursor inside child bounds: set `state.open_at = Some(cursor_pos)` and capture the event so the child doesn't also receive it.
- On `Keyboard(KeyPressed(Escape))` while `open_at.is_some()`: clear `open_at`.
- All other events flow to the child unchanged.

**Overlay:**
- When `state.open_at = Some(pt)`, `overlay()` returns an `overlay::Element` that renders `popover_panel(theme, menu(theme, items))` anchored at `pt`.
- Viewport clamping: if the panel would overflow the right/bottom viewport edge, offset by its own width/height so it stays on-screen. The anchor point then effectively becomes the top-right or bottom-left corner.

**Overlay `update`:**
- Any mouse press whose cursor lands outside the panel bounds: clear `state.open_at`. The original click is not consumed — the element beneath still reacts, matching native context menu dismissal.
- Mouse press/release inside the panel: let the panel deliver the event so an item button fires its `on_press` Message, then clear `state.open_at` on the mouse release event (matches clicking a menu item).

### Shared code reuse

- Popover styling: `popover::popover_panel`.
- Item rendering: `menu::menu` (post-padding-removal).
- Point-anchored overlay: new code in `context_menu.rs`. Not added to `FloatingPanel` — `FloatingPanel` stays rect-anchored, and a generic anchor abstraction isn't justified with only one caller.

## Demo

New `crates/demo/src/demos/context_menu_demo.rs`:

- A bordered box labeled "Right-click here" wrapped in a `ContextMenu` with a small items list (Cut / Copy / Paste / separator / Delete (danger)).
- Each item dispatches `Message::MenuAction(String)` (already handled in `crates/demo/src/lib.rs:629` — stores the label into `state.last_action`). A small "Last action: …" line under the box gives visual feedback that items fired.

Wire a new `Page::ContextMenu` variant with "Context menu" label in the `Layout` category, alongside `Page::MenuBar` and `Page::DropdownMenu`.

## File changes summary

- Modify: `crates/iced-longbridge/src/components/popover.rs` — add `popover_panel`, rewrite `popover_aligned` to use it, drop outer `.padding([6, 8])`.
- Modify: `crates/iced-longbridge/src/components/menu.rs` — drop outer `.padding(4.0)`.
- Modify: `crates/iced-longbridge/src/components/menu_bar.rs` — use `popover_panel` instead of inline styled container.
- Create: `crates/iced-longbridge/src/components/context_menu.rs` — new widget.
- Modify: `crates/iced-longbridge/src/components/mod.rs` — register the module.
- Create: `crates/demo/src/demos/context_menu_demo.rs` — showcase.
- Modify: `crates/demo/src/demos/mod.rs` — register the demo module.
- Modify: `crates/demo/src/lib.rs` — add `Page::ContextMenu`, route in `build_content`, insert in `Page::ALL`.

No tests (codebase pattern: UI components are validated via `cargo build`, `cargo clippy`, and manual demo inspection).

## Open questions

None.
