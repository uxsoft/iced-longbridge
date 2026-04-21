//! Floating panel — a trigger with an optional panel that renders as a true
//! overlay below it.
//!
//! Unlike [`popover`](super::popover), which renders the panel inline in a
//! column (and therefore pushes surrounding layout), this widget returns the
//! panel via `Widget::overlay()` so it floats over other content instead of
//! claiming vertical space. Used by [`menu_bar`](super::menu_bar) so opening
//! one menu doesn't shift the other triggers down.

use iced::{
    Element, Event, Length, Rectangle, Size, Vector,
    advanced::{
        Clipboard, Layout, Shell, Widget,
        layout::{self, Limits},
        mouse,
        overlay::{self, Overlay},
        renderer,
        widget::{self, Tree},
    },
    alignment::Horizontal,
};

pub struct FloatingPanel<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    trigger: Element<'a, Message, Theme, Renderer>,
    panel: Option<Element<'a, Message, Theme, Renderer>>,
    gap: f32,
    align: Horizontal,
}

impl<'a, Message, Theme, Renderer> FloatingPanel<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    pub fn new(
        trigger: impl Into<Element<'a, Message, Theme, Renderer>>,
        panel: Option<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            trigger: trigger.into(),
            panel,
            gap: 4.0,
            align: Horizontal::Left,
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn align(mut self, align: Horizontal) -> Self {
        self.align = align;
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for FloatingPanel<'_, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        let mut kids = vec![Tree::new(&self.trigger)];
        if let Some(p) = &self.panel {
            kids.push(Tree::new(p));
        }
        kids
    }

    fn diff(&self, tree: &mut Tree) {
        if let Some(p) = &self.panel {
            tree.diff_children(&[self.trigger.as_widget(), p.as_widget()]);
        } else {
            tree.diff_children(&[self.trigger.as_widget()]);
        }
    }

    fn size(&self) -> Size<Length> {
        self.trigger.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.trigger.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
    ) -> layout::Node {
        self.trigger
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.trigger.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.trigger.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.trigger.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.trigger.as_widget_mut().operate(
            &mut tree.children[0],
            layout,
            renderer,
            operation,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let (first, rest) = tree.children.split_at_mut(1);
        let trigger_tree = &mut first[0];

        let trigger_overlay = self.trigger.as_widget_mut().overlay(
            trigger_tree,
            layout,
            renderer,
            viewport,
            translation,
        );

        let panel_overlay = match (&mut self.panel, rest.first_mut()) {
            (Some(panel), Some(panel_tree)) => {
                Some(overlay::Element::new(Box::new(PanelOverlay {
                    panel,
                    tree: panel_tree,
                    trigger_bounds: layout.bounds() + translation,
                    gap: self.gap,
                    align: self.align,
                })))
            }
            _ => None,
        };

        match (trigger_overlay, panel_overlay) {
            (None, None) => None,
            (Some(o), None) | (None, Some(o)) => Some(o),
            (Some(a), Some(b)) => Some(
                overlay::Group::with_children(vec![a, b]).overlay(),
            ),
        }
    }
}

impl<'a, Message, Theme, Renderer> From<FloatingPanel<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(widget: FloatingPanel<'a, Message, Theme, Renderer>) -> Self {
        Element::new(widget)
    }
}

struct PanelOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    panel: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut Tree,
    trigger_bounds: Rectangle,
    gap: f32,
    align: Horizontal,
}

impl<Message, Theme, Renderer> Overlay<Message, Theme, Renderer>
    for PanelOverlay<'_, '_, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let limits = Limits::new(Size::ZERO, bounds);
        let node = self
            .panel
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);
        let panel_size = node.size();

        let x = match self.align {
            Horizontal::Left => self.trigger_bounds.x,
            Horizontal::Center => {
                self.trigger_bounds.x
                    + (self.trigger_bounds.width - panel_size.width) / 2.0
            }
            Horizontal::Right => {
                self.trigger_bounds.x + self.trigger_bounds.width - panel_size.width
            }
        };
        let mut y = self.trigger_bounds.y + self.trigger_bounds.height + self.gap;

        let max_x = (bounds.width - panel_size.width).max(0.0);
        let x = x.clamp(0.0, max_x);

        if y + panel_size.height > bounds.height {
            let flipped = self.trigger_bounds.y - self.gap - panel_size.height;
            if flipped >= 0.0 {
                y = flipped;
            } else {
                y = (bounds.height - panel_size.height).max(0.0);
            }
        }

        node.move_to(iced::Point::new(x, y))
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let viewport = layout.bounds();
        self.panel.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            &viewport,
        );
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let viewport = layout.bounds();
        self.panel.as_widget_mut().update(
            self.tree, event, layout, cursor, renderer, clipboard, shell, &viewport,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let viewport = layout.bounds();
        self.panel.as_widget().mouse_interaction(
            self.tree, layout, cursor, &viewport, renderer,
        )
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.panel
            .as_widget_mut()
            .operate(self.tree, layout, renderer, operation);
    }
}
