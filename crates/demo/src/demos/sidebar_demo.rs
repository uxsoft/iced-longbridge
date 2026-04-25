use iced::{
    Element, Length,
    widget::{column, container, row, text},
};

use crate::{
    Message, State,
    components::{
        icon::icon,
        sidebar::{Group, Item, Sidebar},
    },
    demos::common::section_title,
    lucide,
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let collapsed = state.sidebar_demo_collapsed;

    let nav: Element<'a, Message> = Sidebar::new()
        .header(text("Workspace").size(14.0).color(theme.foreground).into())
        .header_collapsed(icon(theme, lucide::house(), 20.0))
        .push(
            Group::new()
                .label("Inbox")
                .push(Item::new("All mail").icon(lucide::mail()).badge("12").active(true).on_press(Message::NoOp))
                .push(Item::new("Starred").icon(lucide::star()).on_press(Message::NoOp))
                .push(Item::new("Drafts").icon(lucide::file()).badge("2").on_press(Message::NoOp)),
        )
        .push(
            Group::new()
                .label("Folders")
                .push(Item::new("Work").icon(lucide::folder()).on_press(Message::NoOp))
                .push(Item::new("Personal").icon(lucide::folder()).on_press(Message::NoOp))
                .push(Item::new("Archive").icon(lucide::folder()).on_press(Message::NoOp)),
        )
        .push(
            Group::new()
                .label("Account")
                .push(Item::new("Settings").icon(lucide::settings()).on_press(Message::NoOp))
                .push(Item::new("Sign out").icon(lucide::lock_open()).on_press(Message::NoOp)),
        )
        .footer(text("v0.1.0").size(11.0).color(theme.muted_foreground).into())
        .footer_collapsed(text("v0.1").size(10.0).color(theme.muted_foreground).into())
        .width(240.0)
        .collapsed(collapsed)
        .on_toggle(Message::SidebarDemoToggle)
        .view(theme);

    let preview = container(row![nav, text("Main content area").size(14.0).color(theme.muted_foreground)])
        .height(Length::Fixed(420.0));

    column![section_title(theme, "Sidebar builder"), preview]
        .spacing(10)
        .into()
}
