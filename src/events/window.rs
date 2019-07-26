
use super::AnyEvent;

pub enum WindowEvent {
    Resize { width: f64, height: f64 },
}

impl AnyEvent for WindowEvent {}

pub type WindowEventHandler = dyn Fn(WindowEvent) -> bool + 'static;

