//! Menu items — shared renderer for dropdown, context, and popup menus.

use iced::{
    Color, Element, Length, Padding,
    alignment::Vertical,
    widget::{button, column, container, row, text, Space},
};

use crate::{
    components::icon::{icon, Icon},
    styles,
    theme::AppTheme,
};

pub enum Item<Message> {
    Separator,
    Action {
        label: String,
        icon: Option<Icon>,
        shortcut: Option<String>,
        on_press: Message,
        danger: bool,
        disabled: bool,
    },
    Header(String),
}

impl<Message> Item<Message> {
    pub fn new(label: impl Into<String>, on_press: Message) -> Self {
        Self::Action {
            label: label.into(),
            icon: None,
            shortcut: None,
            on_press,
            danger: false,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon_src: impl Into<Icon>) -> Self {
        if let Self::Action { ref mut icon, .. } = self {
            *icon = Some(icon_src.into());
        }
        self
    }

    pub fn shortcut(mut self, s: impl Into<String>) -> Self {
        if let Self::Action { ref mut shortcut, .. } = self {
            *shortcut = Some(s.into());
        }
        self
    }

    pub fn danger(mut self) -> Self {
        if let Self::Action { ref mut danger, .. } = self {
            *danger = true;
        }
        self
    }

    pub fn disabled(mut self) -> Self {
        if let Self::Action { ref mut disabled, .. } = self {
            *disabled = true;
        }
        self
    }
}

pub fn menu<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    items: Vec<Item<Message>>,
) -> Element<'a, Message> {
    let t = *theme;
    let mut c = column![].spacing(2).width(Length::Shrink);
    for item in items {
        match item {
            Item::Separator => {
                c = c.push(
                    container(Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
                        .width(Length::Fill)
                        .height(Length::Fixed(1.0))
                        .padding(Padding { top: 4.0, bottom: 4.0, left: 0.0, right: 0.0 })
                        .style(move |_| {
                            styles::simple_container(Some(t.border), t.foreground, Color::TRANSPARENT, 0.0, 0.0)
                        }),
                );
            }
            Item::Header(s) => {
                c = c.push(
                    container(text(s).size(11.0).color(t.muted_foreground))
                        .padding(Padding::from([6.0, 10.0])),
                );
            }
            Item::Action {
                label,
                icon: maybe_icon,
                shortcut,
                on_press,
                danger,
                disabled,
            } => {
                let fg = if danger {
                    t.danger
                } else if disabled {
                    t.muted_foreground
                } else {
                    t.foreground
                };
                let mut inner = row![].spacing(10).align_y(Vertical::Center);
                if let Some(name) = maybe_icon {
                    inner = inner.push(icon(theme, name, 14.0));
                }
                inner = inner.push(text(label).size(13.0).color(fg));
                inner = inner.push(Space::new().width(Length::Fill));
                if let Some(s) = shortcut {
                    inner = inner.push(text(s).size(11.0).color(t.muted_foreground));
                }

                let mut btn = button(inner)
                    .padding(Padding::from([6.0, 12.0]))
                    .width(Length::Fill)
                    .style(move |_, status| {
                        use button::Status::*;
                        let bg = if disabled {
                            Color::TRANSPARENT
                        } else {
                            match status {
                                Hovered => t.accent,
                                Pressed => t.muted,
                                _ => Color::TRANSPARENT,
                            }
                        };
                        styles::button_style(Some(bg), fg, Color::TRANSPARENT, 0.0, 4.0)
                    });
                if !disabled {
                    btn = btn.on_press(on_press);
                }
                c = c.push(btn);
            }
        }
    }

    container(c)
        .width(Length::Fixed(220.0))
        .into()
}
