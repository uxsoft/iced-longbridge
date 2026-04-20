//! Stepper — horizontal progress indicator with numbered/labelled steps.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{column, container, row, text, Space},
};

use crate::theme::AppTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepState {
    Completed,
    Current,
    Upcoming,
}

pub struct Step {
    pub label: String,
    pub description: Option<String>,
}

impl Step {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
        }
    }

    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub fn stepper<'a, Message: 'a>(
    theme: &AppTheme,
    steps: Vec<Step>,
    current: usize,
    orientation: Orientation,
) -> Element<'a, Message> {
    let count = steps.len();
    match orientation {
        Orientation::Horizontal => {
            let mut r = row![].align_y(Vertical::Center).width(Length::Fill);
            for (i, step) in steps.into_iter().enumerate() {
                let state = step_state(i, current);
                r = r.push(step_marker_horizontal(theme, i + 1, &step, state));
                if i + 1 < count {
                    r = r.push(connector(theme, i < current, Length::Fill, Length::Fixed(2.0)));
                }
            }
            r.into()
        }
        Orientation::Vertical => {
            let mut c = column![].width(Length::Fill);
            for (i, step) in steps.into_iter().enumerate() {
                let state = step_state(i, current);
                c = c.push(step_marker_vertical(theme, i + 1, &step, state));
                if i + 1 < count {
                    c = c.push(
                        container(connector(theme, i < current, Length::Fixed(2.0), Length::Fixed(24.0)))
                            .padding(Padding { top: 0.0, right: 0.0, bottom: 0.0, left: 14.0 }),
                    );
                }
            }
            c.into()
        }
    }
}

fn step_state(i: usize, current: usize) -> StepState {
    if i < current {
        StepState::Completed
    } else if i == current {
        StepState::Current
    } else {
        StepState::Upcoming
    }
}

fn step_marker_horizontal<'a, Message: 'a>(
    theme: &AppTheme,
    n: usize,
    step: &Step,
    state: StepState,
) -> Element<'a, Message> {
    let t = *theme;
    let (bg, fg, border) = colors(t, state);
    let circle = container(text(if state == StepState::Completed {
        "✓".to_string()
    } else {
        n.to_string()
    }).size(13.0).color(fg))
    .width(Length::Fixed(28.0))
    .height(Length::Fixed(28.0))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .style(move |_| container::Style {
        background: Some(Background::Color(bg)),
        text_color: Some(fg),
        border: Border {
            color: border,
            width: 2.0,
            radius: 999.0.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    });

    let body = column![
        circle,
        Space::new().height(Length::Fixed(6.0)),
        text(step.label.clone()).size(12.0).color(t.foreground),
    ]
    .spacing(0)
    .align_x(Horizontal::Center);

    container(body).into()
}

fn step_marker_vertical<'a, Message: 'a>(
    theme: &AppTheme,
    n: usize,
    step: &Step,
    state: StepState,
) -> Element<'a, Message> {
    let t = *theme;
    let (bg, fg, border) = colors(t, state);
    let circle = container(text(if state == StepState::Completed {
        "✓".to_string()
    } else {
        n.to_string()
    }).size(13.0).color(fg))
    .width(Length::Fixed(28.0))
    .height(Length::Fixed(28.0))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .style(move |_| container::Style {
        background: Some(Background::Color(bg)),
        text_color: Some(fg),
        border: Border {
            color: border,
            width: 2.0,
            radius: 999.0.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    });

    let mut labels = column![text(step.label.clone()).size(13.0).color(t.foreground)].spacing(2);
    if let Some(d) = &step.description {
        labels = labels.push(text(d.clone()).size(12.0).color(t.muted_foreground));
    }

    row![circle, Space::new().width(Length::Fixed(12.0)), labels]
        .align_y(Vertical::Center)
        .into()
}

fn connector<'a, Message: 'a>(
    theme: &AppTheme,
    completed: bool,
    width: Length,
    height: Length,
) -> Element<'a, Message> {
    let t = *theme;
    let color = if completed { t.primary } else { t.border };
    container(Space::new().width(width).height(height))
        .width(width)
        .height(height)
        .style(move |_| container::Style {
            background: Some(Background::Color(color)),
            text_color: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn colors(t: AppTheme, state: StepState) -> (iced::Color, iced::Color, iced::Color) {
    match state {
        StepState::Completed => (t.primary, t.primary_foreground, t.primary),
        StepState::Current => (t.background, t.primary, t.primary),
        StepState::Upcoming => (t.background, t.muted_foreground, t.border),
    }
}
