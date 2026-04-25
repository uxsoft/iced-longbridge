//! Context menu — right-click on a child to open a menu at the cursor.
//!
//! Unlike [`menu_bar`](super::menu_bar) and [`dropdown_button`](super::dropdown_button),
//! which expose an `open` flag to the caller, this widget manages its own open
//! state via iced's widget tree — callers hold no `Option<Point>` of their own.
//! Dismissal fires on outside-press, Escape, or any mouse release inside the
//! panel (i.e. after an item action fires).

use iced::{
    Element, Event, Length, Point, Rectangle, Size, Vector,
    advanced::{
        Clipboard, Layout, Shell, Widget,
        layout::{self, Limits},
        mouse,
        overlay::{self, Overlay},
        renderer,
        widget::{self, Tree, tree},
    },
    keyboard,
};

use crate::{
    components::{
        menu::{self, Item},
        popover::popover_panel,
    },
    theme::AppTheme,
};

/// Builder. Call [`Self::view`] to obtain an `Element`.
pub struct ContextMenu<'a, Message> {
    child: Element<'a, Message>,
    items: Vec<Item<Message>>,
}

impl<'a, Message: Clone + 'a> ContextMenu<'a, Message> {
    pub fn new(
        child: impl Into<Element<'a, Message>>,
        items: Vec<Item<Message>>,
    ) -> Self {
        Self {
            child: child.into(),
            items,
        }
    }

    pub fn view(self, theme: &AppTheme) -> Element<'a, Message> {
        let panel = popover_panel(theme, menu::menu(theme, self.items));
        Element::new(ContextMenuWidget {
            child: self.child,
            panel,
        })
    }
}

struct ContextMenuWidget<'a, Message> {
    child: Element<'a, Message>,
    panel: Element<'a, Message>,
}

#[derive(Default)]
struct State {
    open_at: Option<Point>,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer>
    for ContextMenuWidget<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.child), Tree::new(&self.panel)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.child.as_widget(), self.panel.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.child.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.child.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &Limits,
    ) -> layout::Node {
        self.child
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // Intercept right-click inside child bounds → open at cursor.
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) = event
            && let Some(pt) = cursor.position_over(layout.bounds())
        {
            let state = tree.state.downcast_mut::<State>();
            state.open_at = Some(pt);
            shell.capture_event();
            return;
        }

        // Escape dismisses while open.
        if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event
            && matches!(key, keyboard::Key::Named(keyboard::key::Named::Escape))
        {
            let state = tree.state.downcast_mut::<State>();
            if state.open_at.is_some() {
                state.open_at = None;
                shell.capture_event();
                return;
            }
        }

        // Otherwise pass through to child.
        self.child.as_widget_mut().update(
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
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.child.as_widget().mouse_interaction(
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
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.child.as_widget().draw(
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
        renderer: &iced::Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.child.as_widget_mut().operate(
            &mut tree.children[0],
            layout,
            renderer,
            operation,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        _layout: Layout<'b>,
        _renderer: &iced::Renderer,
        _viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, iced::Theme, iced::Renderer>> {
        // Borrow-split: state (tree.state) and panel_tree (tree.children[1]) disjoint.
        let Tree { state: state_slot, children, .. } = tree;
        let state = state_slot.downcast_mut::<State>();
        let open_at = state.open_at?;
        let panel_tree = &mut children[1];

        let anchor = Point::new(open_at.x + translation.x, open_at.y + translation.y);

        Some(overlay::Element::new(Box::new(ContextMenuOverlay {
            panel: &mut self.panel,
            tree: panel_tree,
            state,
            anchor,
        })))
    }
}

struct ContextMenuOverlay<'a, 'b, Message> {
    panel: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    state: &'b mut State,
    anchor: Point,
}

impl<Message> Overlay<Message, iced::Theme, iced::Renderer>
    for ContextMenuOverlay<'_, '_, Message>
where
    Message: Clone,
{
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let limits = Limits::new(Size::ZERO, bounds);
        let node = self
            .panel
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);
        let panel_size = node.size();

        // Clamp within viewport: if overflow on the right, shift left so the
        // panel's right edge lines up with `anchor.x`; same for bottom.
        let mut x = self.anchor.x;
        if x + panel_size.width > bounds.width {
            x = (self.anchor.x - panel_size.width).max(0.0);
        }
        let mut y = self.anchor.y;
        if y + panel_size.height > bounds.height {
            y = (self.anchor.y - panel_size.height).max(0.0);
        }

        node.move_to(Point::new(x, y))
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
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
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let viewport = layout.bounds();
        let inside_panel = cursor
            .position()
            .is_some_and(|p| layout.bounds().contains(p));

        // Pass all events into the panel (so item buttons can fire their on_press).
        self.panel.as_widget_mut().update(
            self.tree, event, layout, cursor, renderer, clipboard, shell, &viewport,
        );

        // Dismissal rules:
        //  - mouse press outside panel → close, don't consume (element below stays reactive).
        //  - mouse release inside panel → close (item click finished).
        //  - escape → handled by outer widget's update, not here.
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(_)) if !inside_panel => {
                self.state.open_at = None;
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) if inside_panel => {
                self.state.open_at = None;
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let viewport = layout.bounds();
        self.panel.as_widget().mouse_interaction(
            self.tree, layout, cursor, &viewport, renderer,
        )
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.panel
            .as_widget_mut()
            .operate(self.tree, layout, renderer, operation);
    }
}
