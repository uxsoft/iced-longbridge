//! WASM entry point — compile with `trunk serve` from this directory.
//!
//! Differences from demo-app:
//! - No `window_size` (the browser controls the viewport).
//! - Uses the `webgl` renderer (no Vulkan / Metal / DX12 on the web).
//! - Tokio is replaced by the browser event loop; the `tokio` feature is not
//!   enabled for this crate's iced dependency (see Cargo.toml).

fn main() -> iced::Result {
    iced::application(demo::State::default, demo::update, demo::view)
        .title(title)
        .theme(demo::theme)
        .subscription(demo::subscription)
        .run()
}

fn title(_: &demo::State) -> String {
    String::from("Iced-Longbridge Component Showcase")
}
