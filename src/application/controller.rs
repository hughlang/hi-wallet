use super::*;
use crate::events::*;

use quicksilver::{geom::Vector, lifecycle::Window};

use tweek::{
    core::{TKState},
    gui::{Theme},
};

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

    /// Provides the title for NavController and NavBar to display
    fn screen_title(&self) -> &str { "" }

    /// The controller provides the list of nav items to appear in the navbar from left-to-right
    fn left_nav_items(&self) -> Vec<NavItem> { Vec::new() }

    /// The controller provides the list of nav items to appear in the navbar from left-to-right
    fn right_nav_items(&self) -> Vec<NavItem> { Vec::new() }

    /// Get wrapper for next view controller to navigate to
    fn get_nav_target(&mut self, _event: NavEvent) -> Option<NavTarget> { None }

    /// This is the first stage in the view lifecycle after new() is called. Here is where you should
    /// layout subviews, load data, and prepare for display.
    fn view_will_load(&mut self) {}

    /// Method to signal to controller that it will be leaving or entering the parent controller
    fn will_transition_in(&mut self) {}

    /// The sync method is called from Quicksilver's update loop and eventually gets passed down
    /// to the Scene and lower level Tweek gui objects. It carries the AppContext as a mutable ref
    /// which contains the EventBus where events are propogated up (and possibly down, TBD)
    fn update(&mut self, ctx: &mut AppContext, window: &mut Window);

    /// This is generally a passthru method to the Tweek gui components
    fn render(&mut self, theme: &mut Theme, window: &mut Window);

    /// This is generally a passthru method to the Tweek gui controls
    fn handle_mouse_at(&mut self, _pt: &Vector) -> bool { false }

    /// This is generally a passthru method to the Tweek gui controls
    fn handle_mouse_down(&mut self, _pt: &Vector, _state: &mut TKState) -> bool { false }

    /// This is generally a passthru method to the Tweek gui controls
    fn handle_mouse_up(&mut self, _pt: &Vector, _state: &mut TKState) -> bool { false }

    /// This is generally a passthru method to the Tweek gui controls
    fn handle_mouse_scroll(&mut self, _pt: &Vector) {}


}

// UNUSED
pub trait Container {
    fn render_views(&mut self, theme: &mut Theme, window: &mut Window);
    fn handle_mouse_down(&mut self, _pt: Vector) {}
    fn handle_mouse_up(&mut self, _pt: Vector) {}
}

