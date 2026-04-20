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

impl Size {
    pub fn height(self) -> f32 {
        match self {
            Size::Xs => 22.0,
            Size::Sm => 28.0,
            Size::Md => 34.0,
            Size::Lg => 40.0,
        }
    }

    pub fn padding_x(self) -> f32 {
        match self {
            Size::Xs => 8.0,
            Size::Sm => 12.0,
            Size::Md => 16.0,
            Size::Lg => 20.0,
        }
    }

    pub fn font_size(self) -> f32 {
        match self {
            Size::Xs => 11.0,
            Size::Sm => 13.0,
            Size::Md => 14.0,
            Size::Lg => 16.0,
        }
    }

    pub fn radius(self) -> f32 {
        match self {
            Size::Xs => 4.0,
            Size::Sm => 5.0,
            Size::Md => 6.0,
            Size::Lg => 8.0,
        }
    }
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
