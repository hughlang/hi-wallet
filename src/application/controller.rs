use super::*;
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

    fn screen_title(&self) -> &str { "" }

    /// The controller provides the list of nav items to appear in the navbar from left-to-right
    fn left_nav_items(&self) -> Vec<NavItem> { Vec::new() }

    /// The controller provides the list of nav items to appear in the navbar from left-to-right
    fn right_nav_items(&self) -> Vec<NavItem> { Vec::new() }

    fn view_will_load(&mut self) {}

    fn update(&mut self, window: &mut Window);

    fn render(&mut self, theme: &mut Theme, window: &mut Window);

    fn handle_mouse_at(&mut self, _pt: &Vector) -> bool { false }

    fn handle_mouse_down(&mut self, _pt: &Vector, _state: &mut TKState) -> bool { false }

    fn handle_mouse_up(&mut self, _pt: &Vector, _state: &mut TKState) -> bool { false }

    fn handle_mouse_scroll(&mut self, _pt: &Vector) {}

}

// UNUSED
pub trait Container {
    fn render_views(&mut self, theme: &mut Theme, window: &mut Window);
    fn handle_mouse_down(&mut self, _pt: Vector) {}
    fn handle_mouse_up(&mut self, _pt: Vector) {}
}

