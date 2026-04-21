//! Color picker — swatch trigger with an anchored panel (palette + HSL sliders + hex).

use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow,
    alignment::Vertical,
    widget::{button, column, container, row, slider, text, text_input, Space},
};
use palette::{FromColor, Hsl, IntoColor, Srgb};

use crate::{
    components::popover::popover,
    styles,
    theme::AppTheme,
};

const PANEL_WIDTH: f32 = 280.0;

/// A handful of featured colors shown at the top of the panel.
pub const DEFAULT_PALETTE: &[Color] = &[
    Color { r: 0.93, g: 0.27, b: 0.27, a: 1.0 }, // red
    Color { r: 0.98, g: 0.68, b: 0.13, a: 1.0 }, // orange
    Color { r: 0.92, g: 0.79, b: 0.11, a: 1.0 }, // yellow
    Color { r: 0.15, g: 0.77, b: 0.37, a: 1.0 }, // green
    Color { r: 0.02, g: 0.71, b: 0.83, a: 1.0 }, // cyan
    Color { r: 0.23, g: 0.51, b: 0.96, a: 1.0 }, // blue
    Color { r: 0.55, g: 0.36, b: 0.96, a: 1.0 }, // violet
    Color { r: 0.93, g: 0.33, b: 0.78, a: 1.0 }, // pink
    Color { r: 0.10, g: 0.10, b: 0.10, a: 1.0 }, // near-black
    Color { r: 0.95, g: 0.95, b: 0.95, a: 1.0 }, // near-white
];

pub fn color_picker<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    value: Color,
    hex_draft: &str,
    open: bool,
    on_toggle: Message,
    on_change: impl Fn(Color) -> Message + 'a + Copy,
    on_hex_change: impl Fn(String) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let trigger = swatch_button(theme, value, on_toggle.clone());
    let panel = open.then(|| {
        panel(theme, value, hex_draft, on_change, on_hex_change)
    });
    popover(theme, trigger, panel)
}

/// Just the 32×32 colored swatch without any popover behaviour. Useful in
/// palette pickers where the caller handles positioning themselves.
pub fn swatch<'a, Message: 'a>(theme: &AppTheme, color: Color) -> Element<'a, Message> {
    let t = *theme;
    container(Space::new().width(Length::Fixed(28.0)).height(Length::Fixed(28.0)))
        .style(move |_| container::Style {
            background: Some(Background::Color(color)),
            text_color: None,
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn swatch_button<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    value: Color,
    on_press: Message,
) -> Element<'a, Message> {
    let t = *theme;
    button(Space::new().width(Length::Fixed(28.0)).height(Length::Fixed(28.0)))
        .padding(Padding::from(0.0))
        .on_press(on_press)
        .style(move |_, _| button::Style {
            background: Some(Background::Color(value)),
            text_color: t.foreground,
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn panel<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    value: Color,
    hex_draft: &str,
    on_change: impl Fn(Color) -> Message + 'a + Copy,
    on_hex_change: impl Fn(String) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let t = *theme;
    let (h, s, l) = color_to_hsl(value);

    let mut featured = row![].spacing(6).align_y(Vertical::Center);
    for c in DEFAULT_PALETTE {
        let col = *c;
        featured = featured.push(
            button(Space::new().width(Length::Fixed(22.0)).height(Length::Fixed(22.0)))
                .padding(Padding::from(0.0))
                .on_press(on_change(col))
                .style(move |_, _| button::Style {
                    background: Some(Background::Color(col)),
                    text_color: t.foreground,
                    border: Border {
                        color: t.border,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    shadow: Shadow::default(),
                    snap: true,
                }),
        );
    }

    let hue_slider = slider(0.0..=360.0, h, move |new_h| {
        on_change(hsl_to_color(new_h, s, l, value.a))
    })
    .step(1.0);
    let sat_slider = slider(0.0..=1.0, s, move |new_s| {
        on_change(hsl_to_color(h, new_s, l, value.a))
    })
    .step(0.01);
    let light_slider = slider(0.0..=1.0, l, move |new_l| {
        on_change(hsl_to_color(h, s, new_l, value.a))
    })
    .step(0.01);
    let alpha_slider = slider(0.0..=1.0, value.a, move |new_a| {
        on_change(Color { a: new_a, ..value })
    })
    .step(0.01);

    let hex_field = text_input("#RRGGBB", hex_draft)
        .on_input(on_hex_change)
        .size(13.0)
        .padding(Padding::from([4.0, 8.0]))
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
                    radius: 6.0.into(),
                },
                icon: t.muted_foreground,
                placeholder: t.muted_foreground,
                value: t.foreground,
                selection: crate::theme::with_alpha(t.primary, 0.3),
            }
        });

    let body = column![
        featured,
        labelled_row(theme, "Hue", format!("{:.0}°", h), hue_slider.into()),
        labelled_row(theme, "Saturation", format!("{:.0}%", s * 100.0), sat_slider.into()),
        labelled_row(theme, "Lightness", format!("{:.0}%", l * 100.0), light_slider.into()),
        labelled_row(theme, "Alpha", format!("{:.0}%", value.a * 100.0), alpha_slider.into()),
        row![
            swatch(theme, value),
            container(hex_field).width(Length::Fixed(120.0)),
            text(format_hex(value)).size(12.0).color(t.muted_foreground),
        ]
        .spacing(8)
        .align_y(Vertical::Center),
    ]
    .spacing(10);

    container(body)
        .padding(Padding::from([16.0, 16.0]))
        .width(Length::Fixed(PANEL_WIDTH))
        .style(move |_| styles::popover_container(&t, 10.0))
        .into()
}

fn labelled_row<'a, Message: 'a>(
    theme: &AppTheme,
    label: &str,
    value_label: String,
    control: Element<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    column![
        row![
            text(label.to_string()).size(12.0).color(t.muted_foreground),
            Space::new().width(Length::Fill),
            text(value_label).size(12.0).color(t.foreground),
        ]
        .align_y(Vertical::Center),
        control,
    ]
    .spacing(4)
    .into()
}

pub fn format_hex(c: Color) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        (c.r.clamp(0.0, 1.0) * 255.0).round() as u8,
        (c.g.clamp(0.0, 1.0) * 255.0).round() as u8,
        (c.b.clamp(0.0, 1.0) * 255.0).round() as u8,
    )
}

pub fn parse_hex(input: &str) -> Option<Color> {
    let s = input.trim().trim_start_matches('#');
    let (r, g, b) = match s.len() {
        3 => (
            u8::from_str_radix(&s[0..1].repeat(2), 16).ok()?,
            u8::from_str_radix(&s[1..2].repeat(2), 16).ok()?,
            u8::from_str_radix(&s[2..3].repeat(2), 16).ok()?,
        ),
        6 => (
            u8::from_str_radix(&s[0..2], 16).ok()?,
            u8::from_str_radix(&s[2..4], 16).ok()?,
            u8::from_str_radix(&s[4..6], 16).ok()?,
        ),
        _ => return None,
    };
    Some(Color::from_rgb(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
    ))
}

fn color_to_hsl(c: Color) -> (f32, f32, f32) {
    let srgb = Srgb::new(c.r, c.g, c.b);
    let hsl: Hsl = srgb.into_color();
    let hue = hsl.hue.into_positive_degrees();
    (hue, hsl.saturation, hsl.lightness)
}

fn hsl_to_color(h: f32, s: f32, l: f32, a: f32) -> Color {
    let hsl = Hsl::new(h, s, l);
    let srgb = Srgb::from_color(hsl);
    Color {
        r: srgb.red.clamp(0.0, 1.0),
        g: srgb.green.clamp(0.0, 1.0),
        b: srgb.blue.clamp(0.0, 1.0),
        a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn color_eq(a: Color, b: Color, eps: f32) -> bool {
        (a.r - b.r).abs() < eps
            && (a.g - b.g).abs() < eps
            && (a.b - b.b).abs() < eps
            && (a.a - b.a).abs() < eps
    }

    #[test]
    fn format_hex_round_trip_primaries() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        assert_eq!(format_hex(red), "#FF0000");
        assert!(color_eq(parse_hex("#FF0000").unwrap(), red, 1.0 / 255.0));

        let green = Color::from_rgb(0.0, 1.0, 0.0);
        assert_eq!(format_hex(green), "#00FF00");

        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        assert_eq!(format_hex(blue), "#0000FF");
    }

    #[test]
    fn parse_hex_short_form() {
        let c = parse_hex("#f0a").unwrap();
        assert!(color_eq(c, Color::from_rgb(1.0, 0.0, 0xaa as f32 / 255.0), 0.001));
    }

    #[test]
    fn parse_hex_tolerates_no_leading_hash() {
        assert!(parse_hex("00ff00").is_some());
    }

    #[test]
    fn parse_hex_rejects_junk() {
        assert!(parse_hex("").is_none());
        assert!(parse_hex("#gg00ff").is_none());
        assert!(parse_hex("#1234").is_none());
        assert!(parse_hex("not-a-color").is_none());
    }

    #[test]
    fn hsl_round_trip_preserves_color() {
        let original = Color::from_rgb(0.4, 0.7, 0.2);
        let (h, s, l) = color_to_hsl(original);
        let back = hsl_to_color(h, s, l, 1.0);
        assert!(color_eq(original, back, 0.001), "{:?} vs {:?}", original, back);
    }

    #[test]
    fn format_hex_clamps_out_of_range() {
        let c = Color { r: -0.5, g: 1.5, b: 0.5, a: 1.0 };
        assert_eq!(format_hex(c), "#00FF80");
    }
}
