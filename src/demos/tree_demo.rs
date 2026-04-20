use iced::{Element, widget::{column, container}};

use crate::{
    Message, State,
    components::{
        icon::IconName,
        tree::{tree, TreeNode},
    },
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    column![
        section_title(theme, "File tree"),
        section_caption(theme, "Click the chevron to expand, click a row to select."),
        container(
            tree(
                theme,
                &state.tree_nodes,
                |id: &str| state.tree_expanded.contains(&id.to_string()),
                state.tree_selected.as_deref(),
                Message::TreeToggle,
                Message::TreeSelect,
            ),
        )
        .padding(iced::Padding::from(8.0))
        .style({
            let t = *theme;
            move |_| iced::widget::container::Style {
                background: Some(iced::Background::Color(t.background)),
                text_color: Some(t.foreground),
                border: iced::Border {
                    color: t.border,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: iced::Shadow::default(),
                snap: true,
            }
        })
        .max_width(500),
        vspace(8.0),
        iced::widget::text(match state.tree_selected.as_deref() {
            Some(s) => format!("Selected: {s}"),
            None => "No node selected".into(),
        })
        .size(12.0)
        .color(theme.muted_foreground),
    ]
    .spacing(10)
    .into()
}

pub fn sample_nodes() -> Vec<TreeNode> {
    vec![
        TreeNode::new("src", "src")
            .icon(IconName::Folder)
            .children(vec![
                TreeNode::new("src/components", "components")
                    .icon(IconName::Folder)
                    .children(vec![
                        TreeNode::new("src/components/button.rs", "button.rs").icon(IconName::File),
                        TreeNode::new("src/components/table.rs", "table.rs").icon(IconName::File),
                        TreeNode::new("src/components/tree.rs", "tree.rs").icon(IconName::File),
                    ]),
                TreeNode::new("src/demos", "demos")
                    .icon(IconName::Folder)
                    .children(vec![
                        TreeNode::new("src/demos/tree_demo.rs", "tree_demo.rs").icon(IconName::File),
                    ]),
                TreeNode::new("src/main.rs", "main.rs").icon(IconName::File),
                TreeNode::new("src/theme.rs", "theme.rs").icon(IconName::File),
            ]),
        TreeNode::new("Cargo.toml", "Cargo.toml").icon(IconName::File),
        TreeNode::new("README.md", "README.md").icon(IconName::File),
    ]
}
