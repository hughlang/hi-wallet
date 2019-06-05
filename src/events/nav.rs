/// Events related to navigation requests and actions
///
use super::{AnyEvent, EventBox, EventHandler};

pub enum NavEvent {
    Back,
    Home,
    Detail(usize), // usize is the index of the selected item from datasource
}

pub struct NavButtonEvent {

}

impl AnyEvent for NavButtonEvent {}

// pub type NavEventHandler = Fn(Key) -> bool + 'static;
