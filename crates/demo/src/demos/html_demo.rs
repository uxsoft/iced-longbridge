use iced::{Element, Length, widget::{column, container, row, text}};

use crate::{
    Message, State,
    components::html,
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

pub const SAMPLE: &str = r#"<h1>Iced HTML</h1>
<p>This renderer handles a <strong>subset</strong> of HTML by translating it to Markdown on the fly, then feeding the result through the existing <code>markdown</code> pipeline.</p>
<h2>Supported tags</h2>
<ul>
  <li>Headings — <code>h1</code> through <code>h6</code></li>
  <li>Inline emphasis — <strong>bold</strong> and <em>italic</em></li>
  <li>Inline <code>code</code> and <pre><code>fenced blocks</code></pre></li>
  <li>Ordered and unordered lists</li>
  <li>Links such as <a href="https://github.com/longbridge/gpui-component">longbridge/gpui-component</a></li>
</ul>
<blockquote>Unknown tags are stripped; text content is preserved.</blockquote>
<hr/>
<p>Brought to you by a hand-rolled tag tokenizer.</p>
"#;

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    column![
        section_title(theme, "HTML renderer"),
        section_caption(theme, "Raw HTML (left) rendered through iced-longbridge::components::html (right)."),
        vspace(4.0),
        row![
            container(
                text(SAMPLE.to_string()).size(11.0).color(theme.muted_foreground)
            )
            .width(Length::FillPortion(1)),
            container(html::render(theme, &state.html_items, Message::HtmlLinkClicked))
                .width(Length::FillPortion(1)),
        ]
        .spacing(16),
    ]
    .spacing(10)
    .into()
}
