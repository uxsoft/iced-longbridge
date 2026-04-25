//! Dialog — modal dialog built on the overlay primitive.

use iced::{
    Element, Length, Padding,
    alignment::Vertical,
    widget::{button, column, container, row, text, Space},
};

use crate::{
    components::{icon::{icon_colored, lucide}, overlay},
    styles,
    theme::AppTheme,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Danger,
}

pub struct DialogAction<Message> {
    pub label: String,
    pub on_press: Message,
    pub primary: bool,
}

impl<Message> DialogAction<Message> {
    pub fn new(label: impl Into<String>, on_press: Message) -> Self {
        Self {
            label: label.into(),
            on_press,
            primary: false,
        }
    }

    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }
}

#[allow(clippy::too_many_arguments)]
pub fn dialog<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    base: Element<'a, Message>,
    open: bool,
    variant: DialogVariant,
    title: impl Into<String>,
    description: impl Into<String>,
    on_close: Message,
    actions: Vec<DialogAction<Message>>,
) -> Element<'a, Message> {
    if !open {
        return base;
    }
    let t = *theme;
    let title = title.into();
    let description = description.into();
    let accent = variant_color(t, variant);

    let header_icon: Option<Element<Message>> = match variant {
        DialogVariant::Default => None,
        DialogVariant::Info => Some(icon_colored(lucide::info(), 20.0, accent)),
        DialogVariant::Success => Some(icon_colored(lucide::circle_check(), 20.0, accent)),
        DialogVariant::Warning => Some(icon_colored(lucide::triangle_alert(), 20.0, accent)),
        DialogVariant::Danger => Some(icon_colored(lucide::circle_x(), 20.0, accent)),
    };

    let mut title_row = row![].spacing(10).align_y(Vertical::Center);
    if let Some(ic) = header_icon {
        title_row = title_row.push(ic);
    }
    title_row = title_row
        .push(text(title).size(16.0).color(t.foreground))
        .push(Space::new().width(Length::Fill))
        .push(close_button(t, on_close.clone()));

    let mut action_row = row![Space::new().width(Length::Fill)].spacing(8).align_y(Vertical::Center);
    let has_actions = !actions.is_empty();
    for action in actions {
        action_row = action_row.push(action_button(t, action));
    }

    let mut body = column![
        title_row,
        text(description).size(13.0).color(t.muted_foreground),
    ]
    .spacing(10)
    .width(Length::Fixed(420.0));
    if has_actions {
        body = body.push(Space::new().height(Length::Fixed(6.0)));
        body = body.push(action_row);
    }

    let panel = overlay::panel(theme, body.into(), 480.0);
    overlay::overlay(theme, base, Some(panel), Some(on_close))
}

fn variant_color(t: AppTheme, v: DialogVariant) -> iced::Color {
    match v {
        DialogVariant::Default => t.foreground,
        DialogVariant::Info => t.info,
        DialogVariant::Success => t.success,
        DialogVariant::Warning => t.warning,
        DialogVariant::Danger => t.danger,
    }
}

fn close_button<'a, Message: Clone + 'a>(t: AppTheme, on_press: Message) -> Element<'a, Message> {
    button(icon_colored::<Message>(lucide::x(), 14.0, t.muted_foreground))
        .padding(Padding::from([4.0, 6.0]))
        .on_press(on_press)
        .style(move |_, status| styles::icon_button_style(&t, status, 4.0))
        .into()
}

fn action_button<'a, Message: Clone + 'a>(
    t: AppTheme,
    action: DialogAction<Message>,
) -> Element<'a, Message> {
    let primary = action.primary;
    let label = action.label;
    button(
        container(text(label).size(13.0))
            .padding(Padding::from([0.0, 4.0])),
    )
    .padding(Padding::from([8.0, 16.0]))
    .on_press(action.on_press)
    .style(move |_, status| {
        use button::Status::*;
        let (bg, fg, border) = if primary {
            match status {
                Hovered => (t.primary_hover, t.primary_foreground, t.primary_hover),
                Pressed => (t.primary_active, t.primary_foreground, t.primary_active),
                _ => (t.primary, t.primary_foreground, t.primary),
            }
        } else {
            match status {
                Hovered => (t.accent, t.foreground, t.border),
                Pressed => (t.muted, t.foreground, t.border),
                _ => (t.background, t.foreground, t.border),
            }
        };
        let border_width = if primary { 0.0 } else { 1.0 };
        styles::button_style(Some(bg), fg, border, border_width, 6.0)
    })
    .into()
}
