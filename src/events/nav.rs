/// Events related to navigation requests and actions
///
use super::{AnyEvent};

pub enum NavEvent {
    /// Go back in navigation controller
    Back,
    /// Go next in a sequence, provided by the current controller
    Next,
    /// Go to the Home screen
    Home,
    /// Navigate to first view controller in nav
    Root,
    /// Display modal
    Modal,
    /// Open detail view for selected index
    Selected(usize),
}

impl AnyEvent for NavEvent {}

pub type NavEventHandler = Fn(NavEvent) -> bool + 'static;

