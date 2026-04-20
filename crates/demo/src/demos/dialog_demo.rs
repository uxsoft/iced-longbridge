use iced::{
    Element, Length,
    widget::{column, container, row, text, Space},
};

use crate::{
    Message, State,
    components::{
        button::{button_ex, Variant},
        dialog::{dialog, DialogAction, DialogVariant},
    },
    demos::common::section_title,
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let t = theme;
    let triggers = row![
        button_ex(
            t,
            "Open default",
            Variant::Outline,
            Size::Md,
            Some(Message::DialogOpen(DialogKind::Default)),
            false,
            false,
        ),
        button_ex(
            t,
            "Info",
            Variant::Info,
            Size::Md,
            Some(Message::DialogOpen(DialogKind::Info)),
            false,
            false,
        ),
        button_ex(
            t,
            "Success",
            Variant::Success,
            Size::Md,
            Some(Message::DialogOpen(DialogKind::Success)),
            false,
            false,
        ),
        button_ex(
            t,
            "Warning",
            Variant::Warning,
            Size::Md,
            Some(Message::DialogOpen(DialogKind::Warning)),
            false,
            false,
        ),
        button_ex(
            t,
            "Delete…",
            Variant::Danger,
            Size::Md,
            Some(Message::DialogOpen(DialogKind::Danger)),
            false,
            false,
        ),
    ]
    .spacing(8);

    let body: Element<'a, Message> = column![
        section_title(t, "Dialog triggers"),
        triggers,
        Space::new().height(Length::Fixed(12.0)),
        text("Click a button to show a modal. Click the backdrop or Close to dismiss.").size(13.0).color(t.muted_foreground),
    ]
    .spacing(10)
    .into();

    match state.dialog_open {
        None => body,
        Some(kind) => {
            let (variant, title, description) = dialog_copy(kind);
            let actions = match kind {
                DialogKind::Danger => vec![
                    DialogAction::new("Cancel", Message::DialogClose),
                    DialogAction::new("Delete", Message::DialogClose).primary(),
                ],
                _ => vec![DialogAction::new("Got it", Message::DialogClose).primary()],
            };
            let placeholder = container(Space::new().width(Length::Fill).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill);
            let underlying = column![body, placeholder].spacing(0).into();
            dialog(t, underlying, true, variant, title, description, Message::DialogClose, actions)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogKind {
    Default,
    Info,
    Success,
    Warning,
    Danger,
}

fn dialog_copy(kind: DialogKind) -> (DialogVariant, &'static str, &'static str) {
    match kind {
        DialogKind::Default => (
            DialogVariant::Default,
            "Untitled dialog",
            "A plain dialog with a single primary action.",
        ),
        DialogKind::Info => (
            DialogVariant::Info,
            "Update available",
            "Version 1.2.0 is ready to install. You can continue working; the update installs in the background.",
        ),
        DialogKind::Success => (
            DialogVariant::Success,
            "Saved",
            "Your changes have been saved to the cloud.",
        ),
        DialogKind::Warning => (
            DialogVariant::Warning,
            "Unsaved changes",
            "You have unsaved edits. Leaving this page will discard them.",
        ),
        DialogKind::Danger => (
            DialogVariant::Danger,
            "Delete project?",
            "This action cannot be undone. All data associated with this project will be permanently removed.",
        ),
    }
}
