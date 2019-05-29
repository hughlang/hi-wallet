// use super::*;
// use crate::prelude::*;
use tweek::prelude::*;

use quicksilver::{
    lifecycle::Window,
};

/// The Controller trait will behave like iOS controllers that are generally view controllers
/// that can load the objects to display in a Scene. Alternatively, a Controller could also be
/// a NavController, so that a modal controller could actually start a new navigation stack in
/// a modal view.
pub trait Controller {

    fn update(&mut self, window: &mut Window);

    fn render(&mut self, theme: &mut Theme, window: &mut Window);
    /*
    Ideas:
    – notify: willLoad, didLoad, willAppear, willDisappear, etc
    – getNavController

    */

    // Similar to iOS init (with resources)
    // fn init() -> Self;
    fn view_will_load(&mut self) {}

}

/// See: https://developer.apple.com/documentation/uikit/uimodaltransitionstyle
pub enum ModalTransitionStyle {
    CoverVertical,
    FlipHorizontal,
    CrossDissolve,

}