// use super::*;
// use crate::prelude::*;
use tweek::prelude::*;

use quicksilver::{geom::Vector, lifecycle::Window};

/// See: https://developer.apple.com/documentation/uikit/uimodaltransitionstyle
pub enum ModalTransitionStyle {
    CoverVertical,
    FlipHorizontal,
    CrossDissolve,
}
/// The Controller trait will behave like iOS controllers that are generally view controllers
/// that can load the objects to display in a Scene. Alternatively, a Controller could also be
/// a NavController, so that a modal controller could actually start a new navigation stack in
/// a modal view.
pub trait Controller {
    fn update(&mut self, window: &mut Window);

    fn render(&mut self, theme: &mut Theme, window: &mut Window);

    fn handle_mouse_at(&mut self, _pt: &Vector) -> bool { false }

    fn handle_mouse_down(&mut self, _pt: &Vector, _state: &mut TKState) -> bool { false }

    fn handle_mouse_up(&mut self, _pt: &Vector, _state: &mut TKState) -> bool { false }

    fn handle_mouse_scroll(&mut self, _pt: &Vector) {}

    /*
    Ideas:
    – notify: willLoad, didLoad, willAppear, willDisappear, etc
    – getNavController

    */

    // Similar to iOS init (with resources)
    // fn init() -> Self;
    fn view_will_load(&mut self) {}
}

pub trait Container {
    fn render_views(&mut self, theme: &mut Theme, window: &mut Window);
    fn handle_mouse_down(&mut self, _pt: Vector) {}
    fn handle_mouse_up(&mut self, _pt: Vector) {}
}

pub trait UserEvent {
    fn on_click(&mut self, _id: u32) {}
}