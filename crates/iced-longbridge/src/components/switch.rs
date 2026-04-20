//! Switch component, using iced's toggler widget.

use iced::widget::{toggler, Toggler};

use crate::theme::AppTheme;

pub fn switch<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    is_toggled: bool,
    label: Option<String>,
) -> Toggler<'a, Message> {
    let t = *theme;
    let mut tg = toggler(is_toggled).size(20).spacing(10).style(move |_, status| {
        use iced::widget::toggler::Status::*;
        let (bg, fg) = match status {
            Active { is_toggled } => {
                if is_toggled {
                    (t.primary, t.background)
                } else {
                    (t.muted, t.background)
                }
            }
            Hovered { is_toggled } => {
                if is_toggled {
                    (t.primary_hover, t.background)
                } else {
                    (t.muted_foreground, t.background)
                }
            }
            Disabled { .. } => (crate::theme::with_alpha(t.muted, 0.5), t.background),
        };
        iced::widget::toggler::Style {
            background: bg.into(),
            background_border_width: 0.0,
            background_border_color: iced::Color::TRANSPARENT,
            foreground: fg.into(),
            foreground_border_width: 0.0,
            foreground_border_color: iced::Color::TRANSPARENT,
            text_color: Some(t.foreground),
            border_radius: None,
            padding_ratio: 0.15,
        }
    });
    if let Some(l) = label {
        tg = tg.label(l);
    }
    tg
}
