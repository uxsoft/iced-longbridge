use iced::{
    Element, Length,
    widget::{column, container, row, text},
};

use crate::{
    Message,
    components::{
        icon::IconName,
        sidebar::{Group, Item, Sidebar},
    },
    demos::common::section_title,
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let nav: Element<'a, Message> = Sidebar::new()
        .header(text("Workspace").size(14.0).color(theme.foreground).into())
        .push(
            Group::new()
                .label("Inbox")
                .push(Item::new("All mail").icon(IconName::Mail).badge("12").active(true).on_press(Message::NoOp))
                .push(Item::new("Starred").icon(IconName::Star).on_press(Message::NoOp))
                .push(Item::new("Drafts").icon(IconName::File).badge("2").on_press(Message::NoOp)),
        )
        .push(
            Group::new()
                .label("Folders")
                .push(Item::new("Work").icon(IconName::Folder).on_press(Message::NoOp))
                .push(Item::new("Personal").icon(IconName::Folder).on_press(Message::NoOp))
                .push(Item::new("Archive").icon(IconName::Folder).on_press(Message::NoOp)),
        )
        .push(
            Group::new()
                .label("Account")
                .push(Item::new("Settings").icon(IconName::Settings).on_press(Message::NoOp))
                .push(Item::new("Sign out").icon(IconName::Unlock).on_press(Message::NoOp)),
        )
        .footer(text("v0.1.0").size(11.0).color(theme.muted_foreground).into())
        .width(240.0)
        .view(theme);

    let preview = container(row![nav, text("Main content area").size(14.0).color(theme.muted_foreground)])
        .height(Length::Fixed(420.0));

    column![section_title(theme, "Sidebar builder"), preview]
        .spacing(10)
        .into()
}
