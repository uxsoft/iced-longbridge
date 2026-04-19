//! Iced-Longbridge — a component library and showcase modeled after
//! <https://github.com/longbridge/gpui-component>, implemented on iced 0.14.

mod components;
mod demos;
mod theme;

use std::time::Duration;

use iced::{
    Background, Border, Element, Length, Padding, Shadow, Subscription, Task, Theme,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, scrollable, text, Space},
};

use crate::{
    components::{button::{button_ex, Variant}, divider, icon::{icon, IconName}},
    theme::{AppTheme, Appearance, Size},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Home,
    Button,
    Input,
    Checkbox,
    Radio,
    Switch,
    Badge,
    Tag,
    Alert,
    Divider,
    Progress,
    Slider,
    Spinner,
    Skeleton,
    Tooltip,
    Avatar,
    Link,
    Kbd,
    Rating,
    Label,
    Icon,
}

impl Page {
    pub const ALL: &'static [(Page, &'static str)] = &[
        (Page::Home, "Introduction"),
        (Page::Button, "Button"),
        (Page::Input, "Input"),
        (Page::Checkbox, "Checkbox"),
        (Page::Radio, "Radio"),
        (Page::Switch, "Switch"),
        (Page::Slider, "Slider"),
        (Page::Rating, "Rating"),
        (Page::Badge, "Badge"),
        (Page::Tag, "Tag"),
        (Page::Alert, "Alert"),
        (Page::Progress, "Progress"),
        (Page::Spinner, "Spinner"),
        (Page::Skeleton, "Skeleton"),
        (Page::Tooltip, "Tooltip"),
        (Page::Avatar, "Avatar"),
        (Page::Link, "Link"),
        (Page::Kbd, "Kbd"),
        (Page::Label, "Label"),
        (Page::Icon, "Icon"),
        (Page::Divider, "Divider"),
    ];
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation / chrome
    PageSelected(Page),
    ThemeToggle,
    Tick,

    // Component demos
    ButtonPressed(String),
    InputChanged(String),
    PasswordChanged(String),
    CheckboxToggled(usize, bool),
    RadioSelected(u8),
    SwitchToggled(usize, bool),
    SliderChanged(f32),
    SliderFloatChanged(f32),
    RatingChanged(u8),
    ProgressChanged(f32),
    ProgressReset,
    LinkClicked,

    NoOp,
}

pub struct State {
    theme: AppTheme,
    page: Page,

    // Demo state
    pub input_value: String,
    pub password_value: String,
    pub checkboxes: [bool; 3],
    pub radio_value: u8,
    pub switches: [bool; 4],
    pub slider_value: f32,
    pub slider_float: f32,
    pub rating_value: u8,
    pub progress_value: f32,
    pub last_action: String,
    pub tick_rotation: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: AppTheme::light(),
            page: Page::Home,
            input_value: String::new(),
            password_value: String::new(),
            checkboxes: [true, false, true],
            radio_value: 1,
            switches: [true, false, true, false],
            slider_value: 50.0,
            slider_float: 0.5,
            rating_value: 4,
            progress_value: 60.0,
            last_action: String::from("—"),
            tick_rotation: 0.0,
        }
    }
}

pub fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title(|_state: &State| String::from("Iced-Longbridge Component Showcase"))
        .theme(theme)
        .subscription(subscription)
        .window_size((1180.0, 760.0))
        .run()
}

fn theme(state: &State) -> Theme {
    state.theme.iced_theme()
}

fn subscription(_state: &State) -> Subscription<Message> {
    iced::time::every(Duration::from_millis(32)).map(|_| Message::Tick)
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::PageSelected(p) => state.page = p,
        Message::ThemeToggle => {
            state.theme = match state.theme.appearance {
                Appearance::Light => AppTheme::dark(),
                Appearance::Dark => AppTheme::light(),
            };
        }
        Message::Tick => {
            state.tick_rotation =
                (state.tick_rotation + 0.08) % (std::f32::consts::TAU);
        }
        Message::ButtonPressed(s) => state.last_action = format!("Pressed: {s}"),
        Message::InputChanged(s) => state.input_value = s,
        Message::PasswordChanged(s) => state.password_value = s,
        Message::CheckboxToggled(i, v) => {
            if let Some(b) = state.checkboxes.get_mut(i) {
                *b = v;
            }
        }
        Message::RadioSelected(v) => state.radio_value = v,
        Message::SwitchToggled(i, v) => {
            if let Some(b) = state.switches.get_mut(i) {
                *b = v;
            }
        }
        Message::SliderChanged(v) => state.slider_value = v,
        Message::SliderFloatChanged(v) => state.slider_float = v,
        Message::RatingChanged(v) => state.rating_value = v,
        Message::ProgressChanged(delta) => {
            state.progress_value = (state.progress_value + delta).clamp(0.0, 100.0);
        }
        Message::ProgressReset => state.progress_value = 0.0,
        Message::LinkClicked => state.last_action = "Link clicked".into(),
        Message::NoOp => {}
    }
    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    let t = &state.theme;

    let title_bar = container(
        row![
            text("Iced ✦ Longbridge").size(16.0).color(t.foreground),
            Space::new().width(Length::Fill),
            text(state.last_action.clone()).size(12.0).color(t.muted_foreground),
            Space::new().width(Length::Fixed(16.0)),
            theme_toggle(t),
        ]
        .spacing(8)
        .align_y(Vertical::Center),
    )
    .padding(Padding::from([10.0, 16.0]))
    .width(Length::Fill)
    .style({
        let t = *t;
        move |_| container::Style {
            background: Some(Background::Color(t.background)),
            text_color: Some(t.foreground),
            border: Border {
                color: t.border,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        }
    });

    let sidebar = build_sidebar(state);
    let content = build_content(state);

    let body = row![sidebar, divider::vertical(t), content]
        .height(Length::Fill)
        .width(Length::Fill);

    column![title_bar, divider::horizontal(t), body]
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

fn theme_toggle<'a>(t: &AppTheme) -> Element<'a, Message> {
    let tt = *t;
    let glyph = match t.appearance {
        Appearance::Light => IconName::Moon,
        Appearance::Dark => IconName::Sun,
    };
    button(
        row![icon(t, glyph, 16.0), text("Toggle theme").size(13.0).color(t.foreground)]
            .spacing(8)
            .align_y(Vertical::Center),
    )
    .padding(Padding::from([6.0, 12.0]))
    .on_press(Message::ThemeToggle)
    .style(move |_, status| {
        use button::Status::*;
        let bg = match status {
            Hovered => tt.accent,
            Pressed => tt.muted,
            _ => iced::Color::TRANSPARENT,
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: tt.foreground,
            border: Border {
                color: tt.border,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        }
    })
    .into()
}

fn build_sidebar<'a>(state: &'a State) -> Element<'a, Message> {
    let t = &state.theme;
    let mut nav = column![
        text("Components").size(12.0).color(t.muted_foreground),
        Space::new().height(Length::Fixed(4.0)),
    ]
    .spacing(2)
    .padding(Padding::from([12.0, 10.0]));

    for (page, label) in Page::ALL {
        nav = nav.push(sidebar_link(t, *page, label, state.page == *page));
    }

    let tt = *t;
    container(scrollable(nav))
        .width(Length::Fixed(220.0))
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(tt.sidebar)),
            text_color: Some(tt.sidebar_foreground),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn sidebar_link<'a>(
    t: &AppTheme,
    page: Page,
    label: &'static str,
    active: bool,
) -> Element<'a, Message> {
    let tt = *t;
    button(text(label).size(13.0))
        .width(Length::Fill)
        .padding(Padding::from([6.0, 10.0]))
        .on_press(Message::PageSelected(page))
        .style(move |_, status| {
            use button::Status::*;
            let (bg, fg) = if active {
                (tt.sidebar_accent, tt.foreground)
            } else {
                match status {
                    Hovered => (tt.accent, tt.foreground),
                    Pressed => (tt.muted, tt.foreground),
                    _ => (iced::Color::TRANSPARENT, tt.sidebar_foreground),
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}

fn build_content<'a>(state: &'a State) -> Element<'a, Message> {
    let t = &state.theme;
    let page_label = Page::ALL
        .iter()
        .find(|(p, _)| *p == state.page)
        .map(|(_, l)| *l)
        .unwrap_or("");

    let header = column![
        text(page_label).size(28.0).color(t.foreground),
        text(page_description(state.page))
            .size(14.0)
            .color(t.muted_foreground),
    ]
    .spacing(6);

    let body: Element<'_, Message> = match state.page {
        Page::Home => home_page(state),
        Page::Button => demos::button_demo::view(t),
        Page::Input => demos::input_demo::view(state, t),
        Page::Checkbox => demos::checkbox_demo::view(state, t),
        Page::Radio => demos::radio_demo::view(state, t),
        Page::Switch => demos::switch_demo::view(state, t),
        Page::Badge => demos::badge_demo::view(t),
        Page::Tag => demos::tag_demo::view(t),
        Page::Alert => demos::alert_demo::view(t),
        Page::Divider => demos::divider_demo::view(t),
        Page::Progress => demos::progress_demo::view(state, t),
        Page::Slider => demos::slider_demo::view(state, t),
        Page::Spinner => demos::spinner_demo::view(state, t),
        Page::Skeleton => demos::skeleton_demo::view(t),
        Page::Tooltip => demos::tooltip_demo::view(t),
        Page::Avatar => demos::avatar_demo::view(t),
        Page::Link => demos::link_demo::view(t),
        Page::Kbd => demos::kbd_demo::view(t),
        Page::Rating => demos::rating_demo::view(state, t),
        Page::Label => demos::label_demo::view(state, t),
        Page::Icon => demos::icon_demo::view(t),
    };

    let tt = *t;
    let content = column![header, Space::new().height(Length::Fixed(16.0)), body]
        .spacing(8)
        .padding(Padding::from([24.0, 32.0]));

    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(tt.background)),
            text_color: Some(tt.foreground),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn page_description(page: Page) -> &'static str {
    match page {
        Page::Home => "A faithful iced 0.14 port of longbridge/gpui-component.",
        Page::Button => "Clickable button with variants, sizes, and states.",
        Page::Input => "Text input with sizes, labels, and password mode.",
        Page::Checkbox => "Boolean selection with label.",
        Page::Radio => "Mutually-exclusive single selection.",
        Page::Switch => "Toggle between on / off states.",
        Page::Badge => "Small inline status indicator.",
        Page::Tag => "Lightweight text label.",
        Page::Alert => "Block-level contextual message.",
        Page::Divider => "Horizontal or vertical separator.",
        Page::Progress => "Linear progress indicator.",
        Page::Slider => "Continuous value selector.",
        Page::Spinner => "Animated loading indicator.",
        Page::Skeleton => "Placeholder for loading content.",
        Page::Tooltip => "Contextual popover shown on hover.",
        Page::Avatar => "Circular user identity.",
        Page::Link => "Inline anchor-style link.",
        Page::Kbd => "Keyboard shortcut display.",
        Page::Rating => "Star-based rating input.",
        Page::Label => "Form field label with optional required marker.",
        Page::Icon => "Icon set using Unicode glyphs.",
    }
}

fn home_page<'a>(state: &'a State) -> Element<'a, Message> {
    let t = &state.theme;
    column![
        text("Welcome!").size(22.0).color(t.foreground),
        text("This app showcases a set of iced 0.14 components modeled after the gpui-component library from longbridge.").size(14.0).color(t.muted_foreground),
        Space::new().height(Length::Fixed(12.0)),
        row![
            button_ex(t, "Explore buttons", Variant::Primary, Size::Md, Some(Message::PageSelected(Page::Button)), false, false),
            button_ex(t, "View inputs", Variant::Secondary, Size::Md, Some(Message::PageSelected(Page::Input)), false, false),
        ].spacing(8),
        Space::new().height(Length::Fixed(24.0)),
        container(
            column![
                text("Theme").size(14.0).color(t.foreground),
                text("Click the button in the top right to toggle between light and dark.").size(13.0).color(t.muted_foreground),
            ].spacing(6)
        )
        .padding(Padding::from([14.0, 16.0]))
        .style({
            let tt = *t;
            move |_| container::Style {
                background: Some(Background::Color(tt.muted)),
                text_color: Some(tt.foreground),
                border: Border {
                    color: tt.border,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .max_width(540),
    ]
    .spacing(8)
    .align_x(Horizontal::Left)
    .into()
}
