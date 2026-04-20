# iced-longbridge

A component library for [iced 0.14](https://github.com/iced-rs/iced), modeled after
[longbridge/gpui-component](https://github.com/longbridge/gpui-component). It provides
50+ shadcn/ui-inspired widgets, a light/dark theme system, and five canvas-based chart
types — all usable in both native desktop and WebAssembly applications.

---

## Components

### Basic
Button · Input · Number input · OTP input · Checkbox · Radio · Switch · Slider · Rating

### Display
Badge · Tag · Alert · Progress · Spinner · Skeleton · Tooltip · Avatar · Link · Kbd · Label · Icon · Divider

### Layout
Tabs · Accordion · Collapsible · Breadcrumb · Pagination · Stepper · Group box · Title bar · Dialog · Resizable · Dock · Sidebar

### Form & Overlay
Select · Calendar · Date picker · Hover card · Sheet · Dropdown menu · Button group · Toggle button · Form · Popover · Menu

### Data
Table · Data table · List · Description list · Tree

### Charts
Line · Bar · Area · Pie · Candlestick (OHLC)

---

## Workspace layout

```
crates/
  iced-longbridge/   # reusable component library
  demo/              # shared demo state, views, and 54 demo pages
  demo-app/          # native desktop binary
  demo-web/          # WebAssembly binary (served via Trunk)
```

---

## Running

### Native app

```sh
cargo run -p demo-app
```

### Web app (requires [Trunk](https://trunkrs.dev))

```sh
trunk serve crates/demo-web
```

### Docker

```sh
docker build -t iced-longbridge .
docker run -p 8080:80 iced-longbridge
```

The image builds on both `linux/amd64` and `linux/arm64`. The Rust toolchain and
Trunk are used in a build stage; only the compiled WASM bundle is copied into the
final `nginx:alpine` image.

---

## Theme

`AppTheme` exposes a full shadcn/ui-aligned color palette. Toggle between light and
dark at runtime — every component reads its colors from the theme at render time.

```rust
let theme = AppTheme::light(); // or AppTheme::dark()
```

Four size variants are available across all interactive components: `Xs`, `Sm`, `Md`, `Lg`.

---

## Using the library

Add `iced-longbridge` to your `Cargo.toml`:

```toml
[dependencies]
iced-longbridge = { path = "crates/iced-longbridge" }
```

Then import components and the theme:

```rust
use iced_longbridge::theme::AppTheme;
use iced_longbridge::components::button::{button_ex, Variant};
use iced_longbridge::theme::Size;

let theme = AppTheme::light();
let btn = button_ex(&theme, "Click me", Variant::Primary, Size::Md, Some(Message::Clicked), false, false);
```

---

## Dependencies

| Crate | Version |
|---|---|
| iced | 0.14 |
| chrono | 0.4 |
| palette | 0.7 |
| tokio | 1 |
