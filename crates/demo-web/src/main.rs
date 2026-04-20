//! WASM entry point — compile with `trunk serve` from this directory.
//!
//! Differences from demo-app:
//! - No `window_size` (the browser controls the viewport).
//! - Uses the `webgl` renderer (no Vulkan / Metal / DX12 on the web).
//! - An embedded TTF is required because cosmic-text (iced's text engine)
//!   cannot access system or CSS fonts in a WASM context.

const UBUNTU_FONT: &[u8] = include_bytes!("../fonts/Ubuntu-R.ttf");

fn main() -> iced::Result {
    iced::application(demo::State::default, demo::update, demo::view)
        .title(title)
        .theme(demo::theme)
        .subscription(demo::subscription)
        .font(UBUNTU_FONT)
        .default_font(iced::Font::with_name("Ubuntu"))
        .run()
}

fn title(_: &demo::State) -> String {
    String::from("Iced-Longbridge Component Showcase")
}
