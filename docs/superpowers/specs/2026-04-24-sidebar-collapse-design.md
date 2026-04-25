# Sidebar Collapse — Design

## Problem

The `sidebar` component in `crates/iced-longbridge/src/components/sidebar.rs` is a stateless builder that renders a vertical nav with header, grouped items, and footer. It has no way to render in a narrow icon-only form. We want callers to be able to collapse the sidebar to show only icons, with an optional built-in toggle button.

## Goals

- Allow callers to render the existing sidebar content as an icon-only rail without rewriting groups/items.
- Keep the sidebar stateless — collapsed/expanded is driven by the caller.
- Provide an optional built-in toggle button so callers don't have to wire one up every time.
- Preserve existing content: no badge, item, or group is silently dropped on collapse.

## Non-goals

- Animated transitions between states.
- Auto-collapse based on window width.
- Keyboard shortcut handling.
- Supporting multiple collapsed widths.

## API

Replace the positional-arg `sidebar(...)` function with a builder struct. The only call site is `crates/demo/src/demos/sidebar_demo.rs`; it is updated to the new builder. (The main showcase nav in `crates/demo/src/lib.rs` builds its own inline column of buttons and does not use the reusable component.)

```rust
pub struct Sidebar<'a, Message> {
    header: Option<Element<'a, Message>>,
    header_collapsed: Option<Element<'a, Message>>,
    groups: Vec<Group<'a, Message>>,
    footer: Option<Element<'a, Message>>,
    footer_collapsed: Option<Element<'a, Message>>,
    width: f32,            // expanded width; default 240.0
    collapsed: bool,       // default false
    on_toggle: Option<Message>,
}

impl<'a, Message: Clone + 'a> Sidebar<'a, Message> {
    pub fn new() -> Self;
    pub fn header(self, el: Element<'a, Message>) -> Self;
    pub fn header_collapsed(self, el: Element<'a, Message>) -> Self;
    pub fn push(self, group: Group<'a, Message>) -> Self;
    pub fn footer(self, el: Element<'a, Message>) -> Self;
    pub fn footer_collapsed(self, el: Element<'a, Message>) -> Self;
    pub fn width(self, w: f32) -> Self;
    pub fn collapsed(self, c: bool) -> Self;
    pub fn on_toggle(self, msg: Message) -> Self;
    pub fn view(self, theme: &AppTheme) -> Element<'a, Message>;
}
```

`Item` and `Group` types are unchanged. Item's `icon` remains optional — icon-less items render a dot placeholder when collapsed.

## Rendering — expanded state

Unchanged from current behavior, with one addition: if `on_toggle` is set, a toggle row is appended at the bottom (inside the footer-styled container) showing `ChevronLeft` + "Collapse" text, left-aligned, full-width.

## Rendering — collapsed state

**Container:** width is fixed at 56px; the `width()` setter is ignored when `collapsed == true`.

**Items:**
- 40×40 centered icon button with 8px horizontal container padding.
- Icon rendered at 18px (up from 14px).
- If `item.icon` is `None`: render a 6px filled dot in `muted_foreground` where the icon would be.
- If `item.badge` is `Some(_)`: render a small pill (9px text, `theme.danger` background, `theme.danger_foreground` text, ~6px horizontal padding) overlaid on the icon's top-right corner using iced's `stack` widget.
- Each item is wrapped in `tooltip::wrap(theme, btn, item.label)` so hovering reveals the label.
- Active state uses the same `sidebar_accent` background as expanded mode.

**Groups:**
- `Group.label` is not rendered.
- A 1px horizontal divider in `sidebar_border` color is inserted between consecutive groups, with ~8px vertical margin.
- `Group.extra` is still rendered as-is (rare enough that caller can adapt if needed).

**Header / footer:**
- If `header_collapsed` is set, render it; otherwise omit the header region entirely.
- Same rule for `footer_collapsed`.
- No automatic transformation of the expanded `header` / `footer` elements.

**Toggle button:**
- Rendered only when `on_toggle` is set.
- Lives at the bottom of the sidebar, inside the footer-styled bordered container, below any `footer_collapsed` element.
- Collapsed: `ChevronRight` icon only, tooltip "Expand".
- Expanded: `ChevronLeft` icon + "Collapse" text, left-aligned, full-width.
- Caller toggles their own `collapsed` state on press.

## Demo updates

- `sidebar_demo.rs` gets local state: `collapsed: bool` added to the demo's `State`.
- New `Message::ToggleSidebarDemo` flips the flag.
- Demo renders `Sidebar::new().collapsed(state.collapsed).on_toggle(Message::ToggleSidebarDemo)` and provides both a `header_collapsed` (single workspace icon) and a `footer_collapsed` (small "v" text) so users can see the alternate-content API.

## Out of scope

- Animations between states.
- Auto-collapse at narrow window widths.
- Keyboard shortcuts.
- Customizable collapsed width (can be added later if a caller needs it).

## Open questions

None.
