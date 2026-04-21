//! WASM entry point — compile with `trunk serve` or
//! `wasm-pack build --target web`.
//!
//! Differences from demo-app:
//! - No `window_size` (the browser controls the viewport).
//! - Uses the `webgl` renderer (no Vulkan / Metal / DX12 on the web).
//! - Tokio is replaced by the browser event loop; remove the `tokio` feature
//!   from *this* crate's iced dependency (see Cargo.toml).

pub fn main() -> iced::Result {
    iced::application(demo::State::default, demo::update, demo::view)
        .title(title)
        .theme(demo::theme)
        .subscription(demo::subscription)
        .run()
}

fn title(_: &demo::State) -> String {
    String::from("Iced-Longbridge Component Showcase")
}
