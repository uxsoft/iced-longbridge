//! Icon — caller-supplied SVG handle wrapped to integrate with the theme tint.
//!
//! For convenience the library bundles the [Lucide](https://lucide.dev/icons/)
//! icon set in [`lucide`]; any function returning [`Icon`] also accepts a raw
//! [`iced::widget::svg::Handle`] or anything implementing `Into<Icon>`.
//!
//! Icons render at `size × size` and are tinted by rewriting `currentColor`
//! (theme foreground for [`icon`], explicit color for [`icon_colored`]).
//! Single-color icon sets like Lucide work by design; multi-color SVGs get
//! flattened to the requested tint.

use std::{borrow::Cow, path::PathBuf};

use iced::{
    Color, Element, Length,
    widget::svg,
};

use crate::theme::AppTheme;

/// An icon source — a thin wrapper around an iced [`svg::Handle`].
///
/// Build one with [`Icon::from_svg_path`], [`Icon::from_svg_bytes`], or
/// [`Icon::from_svg_handle`], or use one of the bundled [`lucide`] icons.
#[derive(Debug, Clone)]
pub struct Icon(svg::Handle);

impl Icon {
    /// Load an SVG from the given filesystem path.
    pub fn from_svg_path(path: impl Into<PathBuf>) -> Self {
        Icon(svg::Handle::from_path(path.into()))
    }

    /// Build an icon from in-memory SVG bytes (e.g. via `include_bytes!`).
    pub fn from_svg_bytes(bytes: impl Into<Cow<'static, [u8]>>) -> Self {
        Icon(svg::Handle::from_memory(bytes))
    }

    /// Wrap an existing iced [`svg::Handle`].
    pub fn from_svg_handle(handle: svg::Handle) -> Self {
        Icon(handle)
    }
}

impl From<svg::Handle> for Icon {
    fn from(handle: svg::Handle) -> Self {
        Icon(handle)
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
    let Icon(handle) = icon.into();
    svg(handle)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |_, _| svg::Style { color: Some(color) })
        .into()
}

/// Bundled [Lucide](https://lucide.dev/icons/) icons.
///
/// Each function returns a fresh [`Icon`] backed by `iced::svg::Handle`'s
/// internal `Arc` cache, so calling these per-render is cheap.
pub mod lucide {
    use super::Icon;

    macro_rules! lucide_icon {
        ($name:ident, $slug:literal) => {
            #[doc = concat!("Lucide [`", $slug, "`](https://lucide.dev/icons/", $slug, ").")]
            pub fn $name() -> Icon {
                const BYTES: &[u8] = include_bytes!(concat!(
                    "../../assets/lucide/",
                    $slug,
                    ".svg"
                ));
                Icon::from_svg_bytes(BYTES)
            }
        };
    }

    lucide_icon!(arrow_down, "arrow-down");
    lucide_icon!(arrow_down_wide_narrow, "arrow-down-wide-narrow");
    lucide_icon!(arrow_left, "arrow-left");
    lucide_icon!(arrow_right, "arrow-right");
    lucide_icon!(arrow_up, "arrow-up");
    lucide_icon!(arrow_up_narrow_wide, "arrow-up-narrow-wide");
    lucide_icon!(calendar, "calendar");
    lucide_icon!(check, "check");
    lucide_icon!(chevron_down, "chevron-down");
    lucide_icon!(chevron_left, "chevron-left");
    lucide_icon!(chevron_right, "chevron-right");
    lucide_icon!(chevron_up, "chevron-up");
    lucide_icon!(circle, "circle");
    lucide_icon!(circle_check, "circle-check");
    lucide_icon!(circle_x, "circle-x");
    lucide_icon!(clock, "clock");
    lucide_icon!(copy, "copy");
    lucide_icon!(download, "download");
    lucide_icon!(ellipsis, "ellipsis");
    lucide_icon!(ellipsis_vertical, "ellipsis-vertical");
    lucide_icon!(eye, "eye");
    lucide_icon!(eye_off, "eye-off");
    lucide_icon!(file, "file");
    lucide_icon!(filter, "filter");
    lucide_icon!(folder, "folder");
    lucide_icon!(grip_horizontal, "grip-horizontal");
    lucide_icon!(grip_vertical, "grip-vertical");
    lucide_icon!(heart, "heart");
    lucide_icon!(house, "house");
    lucide_icon!(info, "info");
    lucide_icon!(lock, "lock");
    lucide_icon!(lock_open, "lock-open");
    lucide_icon!(mail, "mail");
    lucide_icon!(menu, "menu");
    lucide_icon!(minus, "minus");
    lucide_icon!(moon, "moon");
    lucide_icon!(palette, "palette");
    lucide_icon!(pencil, "pencil");
    lucide_icon!(phone, "phone");
    lucide_icon!(plus, "plus");
    lucide_icon!(refresh_cw, "refresh-cw");
    lucide_icon!(save, "save");
    lucide_icon!(search, "search");
    lucide_icon!(settings, "settings");
    lucide_icon!(square, "square");
    lucide_icon!(square_minus, "square-minus");
    lucide_icon!(star, "star");
    lucide_icon!(sun, "sun");
    lucide_icon!(trash_2, "trash-2");
    lucide_icon!(triangle_alert, "triangle-alert");
    lucide_icon!(upload, "upload");
    lucide_icon!(user, "user");
    lucide_icon!(x, "x");

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn all_icons_load_bytes() {
            // Forces every include_bytes! to be exercised at compile time
            // already, and confirms each constructor returns a valid Icon.
            let _ = (
                arrow_down(), arrow_down_wide_narrow(), arrow_left(), arrow_right(),
                arrow_up(), arrow_up_narrow_wide(), calendar(), check(),
                chevron_down(), chevron_left(), chevron_right(), chevron_up(),
                circle(), circle_check(), circle_x(), clock(),
                copy(), download(), ellipsis(), ellipsis_vertical(),
                eye(), eye_off(), file(), filter(),
                folder(), grip_horizontal(), grip_vertical(), heart(),
                house(), info(), lock(), lock_open(),
                mail(), menu(), minus(), moon(),
                palette(), pencil(), phone(), plus(),
                refresh_cw(), save(), search(), settings(),
                square(), square_minus(), star(), sun(),
                trash_2(), triangle_alert(), upload(), user(),
                x(),
            );
        }
    }
}
