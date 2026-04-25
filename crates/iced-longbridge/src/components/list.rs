//! List — scrollable column of pressable rows.

use iced::{
    Color, Element, Length, Padding,
    alignment::Vertical,
    widget::{button, column, container, row, scrollable, text, Space},
};

use crate::{
    components::icon::{icon, Icon},
    styles,
    theme::AppTheme,
};

pub struct ListItem<'a, Message> {
    pub leading: Option<Element<'a, Message>>,
    pub primary: String,
    pub secondary: Option<String>,
    pub trailing: Option<Element<'a, Message>>,
    pub selected: bool,
    pub disabled: bool,
    pub on_press: Option<Message>,
}

impl<'a, Message> ListItem<'a, Message> {
    pub fn new(primary: impl Into<String>) -> Self {
        Self {
            leading: None,
            primary: primary.into(),
            secondary: None,
            trailing: None,
            selected: false,
            disabled: false,
            on_press: None,
        }
    }

    pub fn secondary(mut self, s: impl Into<String>) -> Self {
        self.secondary = Some(s.into());
        self
    }

    #[allow(dead_code)]
    pub fn leading(mut self, el: Element<'a, Message>) -> Self {
        self.leading = Some(el);
        self
    }

    pub fn leading_icon(mut self, theme: &AppTheme, icon_src: impl Into<Icon>) -> Self
    where
        Message: 'a,
    {
        self.leading = Some(icon(theme, icon_src, 16.0));
        self
    }

    pub fn trailing(mut self, el: Element<'a, Message>) -> Self {
        self.trailing = Some(el);
        self
    }

    pub fn selected(mut self, s: bool) -> Self {
        self.selected = s;
        self
    }

    #[allow(dead_code)]
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn on_press(mut self, m: Message) -> Self {
        self.on_press = Some(m);
        self
    }
}

pub fn list_item<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    item: ListItem<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    let selected = item.selected;
    let disabled = item.disabled;

    let fg = if disabled {
        t.muted_foreground
    } else {
        t.foreground
    };

    let mut inner = row![].spacing(12).align_y(Vertical::Center);
    if let Some(l) = item.leading {
        inner = inner.push(l);
    }
    let mut labels = column![text(item.primary).size(13.0).color(fg)].spacing(2);
    if let Some(s) = item.secondary {
        labels = labels.push(text(s).size(11.0).color(t.muted_foreground));
    }
    inner = inner.push(labels);
    inner = inner.push(Space::new().width(Length::Fill));
    if let Some(tr) = item.trailing {
        inner = inner.push(tr);
    }

    let mut btn = button(
        container(inner)
            .padding(Padding::from([8.0, 12.0]))
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(0)
    .style(move |_, status| {
        use button::Status::*;
        let bg = if selected {
            t.accent
        } else if disabled {
            Color::TRANSPARENT
        } else {
            match status {
                Hovered => t.muted,
                Pressed => t.accent,
                _ => Color::TRANSPARENT,
            }
        };
        styles::button_style(Some(bg), fg, Color::TRANSPARENT, 0.0, 4.0)
    });
    if !disabled && let Some(m) = item.on_press {
        btn = btn.on_press(m);
    }
    btn.into()
}

pub fn list<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    items: Vec<ListItem<'a, Message>>,
) -> Element<'a, Message> {
    let t = *theme;
    let mut c = column![].spacing(2);
    for it in items {
        c = c.push(list_item(theme, it));
    }
    container(scrollable(c))
        .padding(Padding::from(6.0))
        .width(Length::Fill)
        .style(move |_| {
            styles::simple_container(Some(t.background), t.foreground, t.border, 1.0, 8.0)
        })
        .into()
}

