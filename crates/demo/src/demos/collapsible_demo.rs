use iced::{Element, widget::{column, text}};

use crate::{
    Message, State,
    components::collapsible::collapsible,
    demos::common::{section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    column![
        section_title(theme, "Stand-alone collapsible"),
        collapsible(
            theme,
            "Advanced options",
            state.collapsible_open,
            Message::CollapsibleToggled,
            column![
                text("Toggle this panel to reveal extra settings.").size(13.0).color(theme.muted_foreground),
                vspace(6.0),
                text("Internals: same widget the accordion is built from.").size(12.0).color(theme.muted_foreground),
            ].spacing(0).into(),
        ),
    ]
    .spacing(10)
    .into()
}
