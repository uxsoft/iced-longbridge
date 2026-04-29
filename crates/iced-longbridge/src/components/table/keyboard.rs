//! Keyboard-focus wrapper around a rendered table. Click anywhere inside the
//! table to claim focus; click outside to release it. While focused, nav keys
//! are translated to [`super::NavEvent`]s and emitted via the caller's
//! callback. Other keys fall through unchanged.
//!
//! Boilerplate-heavy because there is no generic "focusable container" in
//! iced 0.14 — every `Widget` trait method other than `update` is a verbatim
//! delegate to the inner element.

use iced::{
    Element, Event, Length, Rectangle, Size, Vector,
    advanced::{
        Clipboard, Layout, Shell, Widget,
        layout::{self, Limits},
        mouse, overlay, renderer,
        widget::{Tree, tree},
    },
    keyboard,
};

use super::NavEvent;

/// Wrap a rendered element so it captures keyboard navigation while focused.
pub(super) fn wrap<'a, Message: Clone + 'a>(
    inner: Element<'a, Message>,
    on_nav: Box<dyn Fn(NavEvent) -> Message + 'a>,
) -> Element<'a, Message> {
    Element::new(KeyboardCapture { inner, on_nav })
}

fn key_to_nav(key: &keyboard::Key) -> Option<NavEvent> {
    use keyboard::key::Named;
    let named = match key {
        keyboard::Key::Named(n) => n,
        _ => return None,
    };
    Some(match named {
        Named::ArrowUp => NavEvent::Up,
        Named::ArrowDown => NavEvent::Down,
        Named::Home => NavEvent::Home,
        Named::End => NavEvent::End,
        Named::PageUp => NavEvent::PageUp,
        Named::PageDown => NavEvent::PageDown,
        Named::Enter => NavEvent::Activate,
        _ => return None,
    })
}

struct KeyboardCapture<'a, Message> {
    inner: Element<'a, Message>,
    on_nav: Box<dyn Fn(NavEvent) -> Message + 'a>,
}

#[derive(Default)]
struct State {
    focused: bool,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for KeyboardCapture<'_, Message>
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
        vec![Tree::new(&self.inner)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.inner.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.inner.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.inner.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &Limits,
    ) -> layout::Node {
        self.inner
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
        // Track focus from left-clicks: gain on click inside, lose on click
        // outside. Run before the inner update so the focus flag reflects this
        // event's click target by the time any nav key (theoretically same
        // frame) is checked.
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            let state = tree.state.downcast_mut::<State>();
            state.focused = cursor.is_over(layout.bounds());
        }

        // While focused, intercept navigation keys before they reach the
        // inner tree. Other keys fall through.
        if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event {
            let focused = tree.state.downcast_ref::<State>().focused;
            if focused
                && let Some(nav) = key_to_nav(key)
            {
                shell.publish((self.on_nav)(nav));
                shell.capture_event();
                return;
            }
        }

        self.inner.as_widget_mut().update(
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
        self.inner.as_widget().mouse_interaction(
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
        self.inner.as_widget().draw(
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
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        self.inner
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, iced::Theme, iced::Renderer>> {
        self.inner.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}
