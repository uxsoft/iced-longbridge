use iced::{Element, widget::column};

use crate::{
    Message, State,
    components::{
        button_group::{button_group, GroupButton},
        code_editor::{code_editor, Language},
    },
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

pub const SAMPLE_RUST: &str = r#"use iced::widget::button;

fn main() -> iced::Result {
    iced::application("Counter", update, view).run()
}

#[derive(Default)]
struct State { count: i64 }

#[derive(Debug, Clone, Copy)]
enum Message { Inc, Dec }

fn update(state: &mut State, message: Message) {
    match message {
        Message::Inc => state.count += 1,
        Message::Dec => state.count -= 1,
    }
}

fn view(state: &State) -> iced::Element<'_, Message> {
    iced::widget::column![
        button("+").on_press(Message::Inc),
        iced::widget::text(state.count),
        button("-").on_press(Message::Dec),
    ].into()
}
"#;

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let buttons = Language::ALL
        .iter()
        .map(|l| {
            GroupButton::new(l.label(), Message::CodeEditorLanguage(*l))
                .active(*l == state.code_editor_language)
        })
        .collect();
    let picker = button_group(theme, buttons);

    column![
        section_title(theme, "Code editor"),
        section_caption(theme, "Syntax-highlighted text editor (via iced::highlighter). Pick a language; edits are live."),
        picker,
        vspace(10.0),
        code_editor(
            theme,
            &state.code_editor_content,
            state.code_editor_language,
            Message::CodeEditorAction,
        ),
    ]
    .spacing(10)
    .into()
}
