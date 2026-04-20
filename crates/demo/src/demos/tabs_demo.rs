use iced::{Element, widget::{column, text}};

use crate::{
    Message, State,
    components::tabs::{tabs, Tab},
    demos::common::{section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let t = theme;
    let content = tabs(
        theme,
        vec![
            Tab::new(
                "Overview",
                column![
                    text("Overview content").size(14.0).color(t.foreground),
                    vspace(4.0),
                    text("Tabs are stateless — the parent manages which tab is selected.").size(13.0).color(t.muted_foreground),
                ].spacing(0).into(),
            ),
            Tab::new(
                "Analytics",
                text("Analytics charts would live here.").size(13.0).color(t.muted_foreground).into(),
            ),
            Tab::new(
                "Reports",
                text("Downloadable reports list would live here.").size(13.0).color(t.muted_foreground).into(),
            ),
            Tab::new(
                "Notifications",
                text("Notification feed with filtering controls.").size(13.0).color(t.muted_foreground).into(),
            ),
        ],
        state.tabs_selected,
        Message::TabsSelected,
    );

    column![section_title(theme, "Tabs"), content].spacing(10).into()
}
