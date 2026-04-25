//! Icon — glyph-based icon rendering plus a path for caller-supplied SVGs.
//!
//! The library ships a curated [`IconName`] enum mapped to Unicode glyphs so
//! components can render icons without bundling a font. Consumers who need
//! something outside that set can build an [`Icon`] from their own SVG bytes
//! or path; everywhere a built-in icon is accepted, an [`Icon`] is too.
//!
//! Custom SVGs are tinted with the same color used for named glyphs (theme
//! foreground for [`icon`], explicit color for [`icon_colored`]). This works
//! best with single-color icon sets (Lucide, Heroicons, Tabler, ...); a
//! multi-color SVG will be flattened to the requested tint.
//!
//! Sizing note: a Unicode glyph rendered at `size` lives inside the font's
//! line box (~`size` tall), while an SVG is laid out as a `size × size`
//! square. For 12–24px icons the two are visually interchangeable.

use std::{borrow::Cow, path::PathBuf};

use iced::{
    Color, Element, Length,
    widget::{svg, text},
};

use crate::theme::AppTheme;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    Check,
    Close,
    Info,
    Warning,
    Success,
    Error,
    Search,
    Plus,
    Minus,
    ChevronDown,
    ChevronUp,
    ChevronLeft,
    ChevronRight,
    Star,
    Heart,
    Home,
    Settings,
    User,
    Mail,
    Phone,
    Lock,
    Unlock,
    Menu,
    Dots,
    Calendar,
    Clock,
    File,
    Folder,
    Trash,
    Edit,
    Copy,
    Save,
    Refresh,
    Download,
    Upload,
    Eye,
    EyeOff,
    Sun,
    Moon,
    Circle,
    CircleFilled,
    CheckCircle,
    XCircle,
    Square,
    SquareMinus,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    GripVertical,
    GripHorizontal,
    Filter,
    SortAsc,
    SortDesc,
    MoreVertical,
    Palette,
}

impl IconName {
    pub const fn glyph(self) -> &'static str {
        match self {
            Self::Check => "✓",
            Self::Close => "✕",
            Self::Info => "ℹ",
            Self::Warning => "⚠",
            Self::Success => "✔",
            Self::Error => "✖",
            Self::Search => "🔍",
            Self::Plus => "+",
            Self::Minus => "−",
            Self::ChevronDown => "▾",
            Self::ChevronUp => "▴",
            Self::ChevronLeft => "◂",
            Self::ChevronRight => "▸",
            Self::Star => "★",
            Self::Heart => "♥",
            Self::Home => "⌂",
            Self::Settings => "⚙",
            Self::User => "👤",
            Self::Mail => "✉",
            Self::Phone => "☎",
            Self::Lock => "🔒",
            Self::Unlock => "🔓",
            Self::Menu => "☰",
            Self::Dots => "⋯",
            Self::Calendar => "📅",
            Self::Clock => "🕐",
            Self::File => "📄",
            Self::Folder => "📁",
            Self::Trash => "🗑",
            Self::Edit => "✎",
            Self::Copy => "⎘",
            Self::Save => "💾",
            Self::Refresh => "⟳",
            Self::Download => "⤓",
            Self::Upload => "⤒",
            Self::Eye => "👁",
            Self::EyeOff => "⊘",
            Self::Sun => "☀",
            Self::Moon => "☾",
            Self::Circle => "○",
            Self::CircleFilled => "●",
            Self::CheckCircle => "⊙",
            Self::XCircle => "⊗",
            Self::Square => "▢",
            Self::SquareMinus => "▭",
            Self::ArrowLeft => "←",
            Self::ArrowRight => "→",
            Self::ArrowUp => "↑",
            Self::ArrowDown => "↓",
            Self::GripVertical => "⋮",
            Self::GripHorizontal => "⋯",
            Self::Filter => "⧩",
            Self::SortAsc => "▲",
            Self::SortDesc => "▼",
            Self::MoreVertical => "⋮",
            Self::Palette => "🎨",
        }
    }
}

/// An icon source — either one of the curated [`IconName`] glyphs or a
/// caller-supplied SVG handle.
///
/// Construct one of these directly via [`Icon::from_svg_path`],
/// [`Icon::from_svg_bytes`], or [`Icon::from_svg_handle`], or rely on the
/// `From<IconName>` impl so `IconName::Foo` flows through builders that
/// accept `impl Into<Icon>`.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Icon {
    Named(IconName),
    Svg(svg::Handle),
}

impl Icon {
    /// Load an SVG from the given filesystem path.
    pub fn from_svg_path(path: impl Into<PathBuf>) -> Self {
        Icon::Svg(svg::Handle::from_path(path.into()))
    }

    /// Build an icon from in-memory SVG bytes (e.g. via `include_bytes!`).
    pub fn from_svg_bytes(bytes: impl Into<Cow<'static, [u8]>>) -> Self {
        Icon::Svg(svg::Handle::from_memory(bytes))
    }

    /// Wrap an existing iced [`svg::Handle`].
    pub fn from_svg_handle(handle: svg::Handle) -> Self {
        Icon::Svg(handle)
    }
}

impl From<IconName> for Icon {
    fn from(name: IconName) -> Self {
        Icon::Named(name)
    }
}

pub fn icon<'a, Message: 'a>(
    theme: &AppTheme,
    icon: impl Into<Icon>,
    size: f32,
) -> Element<'a, Message> {
    icon_colored(icon, size, theme.foreground)
}

pub fn icon_colored<'a, Message: 'a>(
    icon: impl Into<Icon>,
    size: f32,
    color: Color,
) -> Element<'a, Message> {
    match icon.into() {
        Icon::Named(name) => text(name.glyph()).size(size).color(color).into(),
        Icon::Svg(handle) => svg(handle)
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .style(move |_, _| svg::Style { color: Some(color) })
            .into(),
    }
}
