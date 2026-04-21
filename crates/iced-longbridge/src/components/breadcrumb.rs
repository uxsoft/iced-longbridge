//! Breadcrumb — a navigation trail of links separated by chevrons.

use iced::{
    Background, Border, Element, Padding, Shadow,
    alignment::Vertical,
    widget::{button, row, text},
};

use crate::{
    components::icon::{icon_colored, IconName},
    theme::AppTheme,
};

pub struct Crumb<Message> {
    pub label: String,
    pub on_press: Option<Message>,
}

impl<Message> Crumb<Message> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            on_press: None,
        }
    }

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }
}

pub fn breadcrumb<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    crumbs: Vec<Crumb<Message>>,
) -> Element<'a, Message> {
    let t = *theme;
    let count = crumbs.len();
    let mut r = row![].spacing(6).align_y(Vertical::Center);

    for (i, crumb) in crumbs.into_iter().enumerate() {
        let is_last = i + 1 == count;
        let is_link = !is_last && crumb.on_press.is_some();
        let fg = if is_last { t.foreground } else { t.muted_foreground };

        if let (true, Some(msg)) = (is_link, crumb.on_press) {
            let btn = button(text(crumb.label).size(13.0).color(fg))
                .padding(Padding::from([2.0, 4.0]))
                .on_press(msg)
                .style(move |_, status| {
                    use button::Status::*;
                    let color = match status {
                        Hovered | Pressed => t.foreground,
                        _ => t.muted_foreground,
                    };
                    button::Style {
                        background: Some(Background::Color(iced::Color::TRANSPARENT)),
                        text_color: color,
                        border: Border {
                            color: iced::Color::TRANSPARENT,
                            width: 0.0,
                            radius: 4.0.into(),
                        },
                        shadow: Shadow::default(),
                        snap: true,
                    }
                });
            r = r.push(btn);
        } else {
            r = r.push(text(crumb.label).size(13.0).color(fg));
        }

        if !is_last {
            r = r.push(icon_colored::<Message>(
                IconName::ChevronRight,
                12.0,
                t.muted_foreground,
            ));
        }
    }
    r.into()
}
