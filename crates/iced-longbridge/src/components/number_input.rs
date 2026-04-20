//! Number input — a text input with inc/dec buttons.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{button, container, row, text, text_input},
};

use crate::theme::{AppTheme, Size};

pub fn number_input<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    value: f64,
    on_change: impl Fn(f64) -> Message + 'a + Copy,
    step: f64,
    min: Option<f64>,
    max: Option<f64>,
) -> Element<'a, Message> {
    number_input_sized(theme, Size::Md, value, on_change, step, min, max)
}

pub fn number_input_sized<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    size: Size,
    value: f64,
    on_change: impl Fn(f64) -> Message + 'a + Copy,
    step: f64,
    min: Option<f64>,
    max: Option<f64>,
) -> Element<'a, Message> {
    let t = *theme;
    let radius = size.radius();
    let h = size.height();
    let text_size = size.font_size();

    let decreased = clamp((value - step) * 1.0, min, max);
    let increased = clamp(value + step, min, max);

    let dec_disabled = matches!(min, Some(m) if value <= m);
    let inc_disabled = matches!(max, Some(m) if value >= m);

    let formatted = format_number(value);

    let input = text_input("0", &formatted)
        .on_input(move |s| {
            let parsed = s.parse::<f64>().unwrap_or(value);
            on_change(clamp(parsed, min, max))
        })
        .size(text_size)
        .padding(Padding::from([0.0, 8.0]))
        .style(move |_, status| {
            use text_input::Status::*;
            let border = match status {
                Focused { .. } => t.ring,
                Hovered => t.muted_foreground,
                _ => t.input_border,
            };
            text_input::Style {
                background: Background::Color(t.background),
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: radius.into(),
                },
                icon: t.muted_foreground,
                placeholder: t.muted_foreground,
                value: t.foreground,
                selection: crate::theme::with_alpha(t.primary, 0.3),
            }
        });

    let stepper_btn = |glyph: &str, disabled: bool, on_press: Option<Message>| {
        let content = container(text(glyph.to_string()).size(text_size).color(if disabled { t.muted_foreground } else { t.foreground }))
            .width(Length::Fixed(h * 0.9))
            .height(Length::Fixed(h))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);
        let mut btn = button(content)
            .padding(Padding::from(0.0))
            .style(move |_, status| {
                use button::Status::*;
                let bg = match status {
                    Hovered => t.accent,
                    Pressed => t.muted,
                    _ => t.background,
                };
                button::Style {
                    background: Some(Background::Color(bg)),
                    text_color: t.foreground,
                    border: Border {
                        color: t.input_border,
                        width: 1.0,
                        radius: radius.into(),
                    },
                    shadow: Shadow::default(),
                    snap: true,
                }
            });
        if let Some(m) = on_press {
            btn = btn.on_press(m);
        }
        btn
    };

    row![
        stepper_btn("−", dec_disabled, (!dec_disabled).then(|| on_change(decreased))),
        container(input).width(Length::Fixed(110.0)).height(Length::Fixed(h)),
        stepper_btn("+", inc_disabled, (!inc_disabled).then(|| on_change(increased))),
    ]
    .spacing(4)
    .align_y(Vertical::Center)
    .into()
}

fn clamp(v: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let mut v = v;
    if let Some(m) = min
        && v < m
    {
        v = m;
    }
    if let Some(m) = max
        && v > m
    {
        v = m;
    }
    v
}

fn format_number(v: f64) -> String {
    if (v - v.round()).abs() < f64::EPSILON {
        format!("{}", v as i64)
    } else {
        format!("{}", v)
    }
}
