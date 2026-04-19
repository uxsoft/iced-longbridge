//! Icon — glyph-based icon rendering. Uses Unicode symbols for simplicity.

use iced::{Color, Element, widget::text};

use crate::theme::AppTheme;

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
        }
    }
}

pub fn icon<'a, Message: 'a>(
    theme: &AppTheme,
    name: IconName,
    size: f32,
) -> Element<'a, Message> {
    icon_colored(name, size, theme.foreground)
}

pub fn icon_colored<'a, Message: 'a>(
    name: IconName,
    size: f32,
    color: Color,
) -> Element<'a, Message> {
    text(name.glyph()).size(size).color(color).into()
}
