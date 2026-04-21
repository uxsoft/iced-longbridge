//! Theme system mirroring gpui-component's shadcn-inspired design.
//!
//! The palette follows the neutral scale used by shadcn/ui and gpui-component.
//! Components read colors from [`AppTheme`] to produce `iced` styles.

use iced::{Color, Theme, theme::Palette};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Appearance {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Xs,
    Sm,
    Md,
    Lg,
}

/// Layout metrics for a [`Size`]. All values are in logical pixels.
#[derive(Debug, Clone, Copy)]
pub struct SizeMetrics {
    pub height: f32,
    pub padding_x: f32,
    pub font_size: f32,
    pub radius: f32,
}

impl Size {
    /// Single source of truth for per-size layout values. Individual accessors
    /// (`height()`, `padding_x()`, `font_size()`, `radius()`) delegate here.
    pub const fn metrics(self) -> SizeMetrics {
        match self {
            Size::Xs => SizeMetrics { height: 22.0, padding_x: 8.0,  font_size: 11.0, radius: 4.0 },
            Size::Sm => SizeMetrics { height: 28.0, padding_x: 12.0, font_size: 13.0, radius: 5.0 },
            Size::Md => SizeMetrics { height: 34.0, padding_x: 16.0, font_size: 14.0, radius: 6.0 },
            Size::Lg => SizeMetrics { height: 40.0, padding_x: 20.0, font_size: 16.0, radius: 8.0 },
        }
    }

    pub fn height(self) -> f32 { self.metrics().height }
    pub fn padding_x(self) -> f32 { self.metrics().padding_x }
    pub fn font_size(self) -> f32 { self.metrics().font_size }
    pub fn radius(self) -> f32 { self.metrics().radius }
}

/// Full palette used across the component library.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct AppTheme {
    pub appearance: Appearance,

    pub background: Color,
    pub foreground: Color,
    pub border: Color,
    pub ring: Color,

    pub muted: Color,
    pub muted_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,

    pub primary: Color,
    pub primary_hover: Color,
    pub primary_active: Color,
    pub primary_foreground: Color,

    pub secondary: Color,
    pub secondary_hover: Color,
    pub secondary_active: Color,
    pub secondary_foreground: Color,

    pub danger: Color,
    pub danger_foreground: Color,

    pub success: Color,
    pub success_foreground: Color,

    pub warning: Color,
    pub warning_foreground: Color,

    pub info: Color,
    pub info_foreground: Color,

    pub input_border: Color,
    pub popover: Color,
    pub popover_foreground: Color,

    pub sidebar: Color,
    pub sidebar_foreground: Color,
    pub sidebar_border: Color,
    pub sidebar_accent: Color,

    pub card: Color,
    pub card_foreground: Color,

    pub skeleton: Color,
    pub overlay: Color,

    pub chart: [Color; 5],

    pub link: Color,
    pub link_hover: Color,
}

impl AppTheme {
    pub const fn light() -> Self {
        Self {
            appearance: Appearance::Light,
            background: rgb(0xff, 0xff, 0xff),
            foreground: rgb(0x0a, 0x0a, 0x0a),
            border: rgb(0xe5, 0xe5, 0xe5),
            ring: rgb(0x0a, 0x0a, 0x0a),
            muted: rgb(0xf5, 0xf5, 0xf5),
            muted_foreground: rgb(0x73, 0x73, 0x73),
            accent: rgb(0xf5, 0xf5, 0xf5),
            accent_foreground: rgb(0x17, 0x17, 0x17),
            primary: rgb(0x17, 0x17, 0x17),
            primary_hover: rgb(0x30, 0x30, 0x30),
            primary_active: rgb(0x00, 0x00, 0x00),
            primary_foreground: rgb(0xfa, 0xfa, 0xfa),
            secondary: rgb(0xe5, 0xe5, 0xe5),
            secondary_hover: rgb(0xd4, 0xd4, 0xd4),
            secondary_active: rgb(0xa3, 0xa3, 0xa3),
            secondary_foreground: rgb(0x17, 0x17, 0x17),
            danger: rgb(0xef, 0x44, 0x44),
            danger_foreground: rgb(0xfa, 0xfa, 0xfa),
            success: rgb(0x22, 0xc5, 0x5e),
            success_foreground: rgb(0xfa, 0xfa, 0xfa),
            warning: rgb(0xea, 0xab, 0x08),
            warning_foreground: rgb(0xfa, 0xfa, 0xfa),
            info: rgb(0x06, 0xb6, 0xd4),
            info_foreground: rgb(0xfa, 0xfa, 0xfa),
            input_border: rgb(0xe5, 0xe5, 0xe5),
            popover: rgb(0xff, 0xff, 0xff),
            popover_foreground: rgb(0x0a, 0x0a, 0x0a),
            sidebar: rgb(0xfa, 0xfa, 0xfa),
            sidebar_foreground: rgb(0x17, 0x17, 0x17),
            sidebar_border: rgb(0xe5, 0xe5, 0xe5),
            sidebar_accent: rgb(0xe5, 0xe5, 0xe5),
            card: rgb(0xff, 0xff, 0xff),
            card_foreground: rgb(0x0a, 0x0a, 0x0a),
            skeleton: rgb(0xf5, 0xf5, 0xf5),
            overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            chart: [
                rgb(0x93, 0xc5, 0xfd),
                rgb(0x3b, 0x82, 0xf6),
                rgb(0x25, 0x63, 0xeb),
                rgb(0x1d, 0x4e, 0xd8),
                rgb(0x1e, 0x40, 0xaf),
            ],
            link: rgb(0x0a, 0x0a, 0x0a),
            link_hover: rgb(0x40, 0x40, 0x40),
        }
    }

    pub const fn dark() -> Self {
        Self {
            appearance: Appearance::Dark,
            background: rgb(0x0a, 0x0a, 0x0a),
            foreground: rgb(0xfa, 0xfa, 0xfa),
            border: rgb(0x26, 0x26, 0x26),
            ring: rgb(0xd4, 0xd4, 0xd4),
            muted: rgb(0x26, 0x26, 0x26),
            muted_foreground: rgb(0xa3, 0xa3, 0xa3),
            accent: rgb(0x26, 0x26, 0x26),
            accent_foreground: rgb(0xfa, 0xfa, 0xfa),
            primary: rgb(0xfa, 0xfa, 0xfa),
            primary_hover: rgb(0xe5, 0xe5, 0xe5),
            primary_active: rgb(0xd4, 0xd4, 0xd4),
            primary_foreground: rgb(0x17, 0x17, 0x17),
            secondary: rgb(0x26, 0x26, 0x26),
            secondary_hover: rgb(0x40, 0x40, 0x40),
            secondary_active: rgb(0x52, 0x52, 0x52),
            secondary_foreground: rgb(0xfa, 0xfa, 0xfa),
            danger: rgb(0xf8, 0x71, 0x71),
            danger_foreground: rgb(0xfa, 0xfa, 0xfa),
            success: rgb(0x4a, 0xde, 0x80),
            success_foreground: rgb(0x05, 0x24, 0x0d),
            warning: rgb(0xfa, 0xcc, 0x15),
            warning_foreground: rgb(0x17, 0x17, 0x17),
            info: rgb(0x67, 0xe8, 0xf9),
            info_foreground: rgb(0x08, 0x3a, 0x42),
            input_border: rgb(0x2f, 0x2f, 0x2f),
            popover: rgb(0x0a, 0x0a, 0x0a),
            popover_foreground: rgb(0xfa, 0xfa, 0xfa),
            sidebar: rgb(0x0d, 0x0d, 0x0d),
            sidebar_foreground: rgb(0xfa, 0xfa, 0xfa),
            sidebar_border: rgb(0x1f, 0x1f, 0x1f),
            sidebar_accent: rgb(0x26, 0x26, 0x26),
            card: rgb(0x17, 0x17, 0x17),
            card_foreground: rgb(0xfa, 0xfa, 0xfa),
            skeleton: rgb(0x26, 0x26, 0x26),
            overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
            chart: [
                rgb(0x93, 0xc5, 0xfd),
                rgb(0x3b, 0x82, 0xf6),
                rgb(0x25, 0x63, 0xeb),
                rgb(0x1d, 0x4e, 0xd8),
                rgb(0x1e, 0x40, 0xaf),
            ],
            link: rgb(0xfa, 0xfa, 0xfa),
            link_hover: rgb(0xff, 0xff, 0xff),
        }
    }

    /// Default 10-colour featured swatch palette used by the colour picker.
    /// Override per-theme if a particular palette makes sense for your app.
    pub fn featured_palette(&self) -> &'static [Color] {
        DEFAULT_FEATURED_PALETTE
    }

    pub fn iced_theme(&self) -> Theme {
        let palette = Palette {
            background: self.background,
            text: self.foreground,
            primary: self.primary,
            success: self.success,
            danger: self.danger,
            warning: self.warning,
        };
        match self.appearance {
            Appearance::Light => Theme::custom("Longbridge Light", palette),
            Appearance::Dark => Theme::custom("Longbridge Dark", palette),
        }
    }
}

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

pub fn with_alpha(color: Color, a: f32) -> Color {
    Color { a, ..color }
}

impl Default for AppTheme {
    fn default() -> Self {
        Self::light()
    }
}

/// Featured swatches shown across the colour picker and other palette UIs.
const DEFAULT_FEATURED_PALETTE: &[Color] = &[
    Color { r: 0.93, g: 0.27, b: 0.27, a: 1.0 }, // red
    Color { r: 0.98, g: 0.68, b: 0.13, a: 1.0 }, // orange
    Color { r: 0.92, g: 0.79, b: 0.11, a: 1.0 }, // yellow
    Color { r: 0.15, g: 0.77, b: 0.37, a: 1.0 }, // green
    Color { r: 0.02, g: 0.71, b: 0.83, a: 1.0 }, // cyan
    Color { r: 0.23, g: 0.51, b: 0.96, a: 1.0 }, // blue
    Color { r: 0.55, g: 0.36, b: 0.96, a: 1.0 }, // violet
    Color { r: 0.93, g: 0.33, b: 0.78, a: 1.0 }, // pink
    Color { r: 0.10, g: 0.10, b: 0.10, a: 1.0 }, // near-black
    Color { r: 0.95, g: 0.95, b: 0.95, a: 1.0 }, // near-white
];
