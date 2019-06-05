pub use self::nav::*;
pub use self::window::*;

mod nav;
mod window;

// *****************************************************************************************************
// Prototyping area below
// *****************************************************************************************************

use std::{
    any::{Any, TypeId},
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

/// Used to define an event.
pub trait AnyEvent: Any {}


pub trait EventHandler {
    /// Handles an `event` by the given `widget`. If it returns `true` the event will not be forwarded.
    fn handle_event(&self, event: &EventBox) -> bool;
}

#[derive(Debug)]
pub enum EventError {
    WrongType(TypeId),
}

#[derive(Debug)]
pub struct EventBox {
    event: Box<dyn Any>,
    event_type: TypeId,
}

impl EventBox {
    pub fn new<E: AnyEvent>(event: E) -> Self {
        EventBox { event: Box::new(event), event_type: TypeId::of::<E>() }
    }

    pub fn is_type<E: AnyEvent>(&self) -> bool {
        self.event_type == TypeId::of::<E>()
    }

    pub fn event_type(&self) -> TypeId {
        self.event_type
    }
}

#[derive(Default, Debug)]
pub struct EventBus {
    event_queue: Vec<EventBox>,
}

impl EventBus {
    pub fn append(&mut self, other: &mut Vec<EventBox>) {
        self.event_queue.append(other);
    }

    pub fn register_event<E: AnyEvent>(&mut self, event: E) {
        self.event_queue.push(EventBox::new::<E>(event));
    }

    pub fn dequeue(&mut self) -> Option<EventBox> {
        if !self.event_queue.is_empty() {
            return Some(self.event_queue.remove(0));
        }
        None
    }

    pub fn len(&self) -> usize {
        self.event_queue.len()
    }
}

pub struct EventSystem {
    // pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
}

