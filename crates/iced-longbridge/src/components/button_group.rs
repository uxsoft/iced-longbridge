//! Button group — segmented buttons sharing an outer border.

use iced::{
    Background, Border, Element, Padding, Shadow,
    alignment::Vertical,
    widget::{button, row, text},
};

use crate::theme::{AppTheme, Size};

pub struct GroupButton<Message> {
    pub label: String,
    pub on_press: Message,
    pub active: bool,
}

impl<Message> GroupButton<Message> {
    pub fn new(label: impl Into<String>, on_press: Message) -> Self {
        Self {
            label: label.into(),
            on_press,
            active: false,
        }
    }

    pub fn active(mut self, a: bool) -> Self {
        self.active = a;
        self
    }
}

pub fn button_group<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    buttons: Vec<GroupButton<Message>>,
) -> Element<'a, Message> {
    button_group_sized(theme, Size::Md, buttons)
}

pub fn button_group_sized<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    size: Size,
    buttons: Vec<GroupButton<Message>>,
) -> Element<'a, Message> {
    let t = *theme;
    let radius = size.radius();
    let h = size.height();
    let text_size = size.font_size();
    let px = size.padding_x();
    let count = buttons.len();

    let mut r = row![].spacing(0).align_y(Vertical::Center);
    for (i, b) in buttons.into_iter().enumerate() {
        let is_first = i == 0;
        let is_last = i + 1 == count;
        let active = b.active;
        let label = b.label;
        r = r.push(
            button(text(label).size(text_size))
                .padding(Padding::from([0.0, px]))
                .height(iced::Length::Fixed(h))
                .on_press(b.on_press)
                .style(move |_, status| {
                    use button::Status::*;
                    let (bg, fg) = if active {
                        (t.primary, t.primary_foreground)
                    } else {
                        match status {
                            Hovered => (t.accent, t.foreground),
                            Pressed => (t.muted, t.foreground),
                            _ => (t.background, t.foreground),
                        }
                    };
                    let br = iced::border::Radius {
                        top_left: if is_first { radius } else { 0.0 },
                        bottom_left: if is_first { radius } else { 0.0 },
                        top_right: if is_last { radius } else { 0.0 },
                        bottom_right: if is_last { radius } else { 0.0 },
                    };
                    button::Style {
                        background: Some(Background::Color(bg)),
                        text_color: fg,
                        border: Border {
                            color: t.input_border,
                            width: 1.0,
                            radius: br,
                        },
                        shadow: Shadow::default(),
                        snap: true,
                    }
                }),
        );
    }
    r.into()
}
