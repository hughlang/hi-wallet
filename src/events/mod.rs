pub use self::nav::*;
pub use self::window::*;

mod nav;
mod window;

use std::{
    any::{Any, TypeId},
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

// *****************************************************************************************************
// Prototyping area below
// Most of this code was copied from OrbTk and modified for compatibility
// *****************************************************************************************************

/// Used to define an event trait.
pub trait AnyEvent: Any {}

pub struct EventSystem {
    // pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
}

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

/// Disclosure: This structure was copied from OrbTk, along with other code on this page.
/// TODO: discuss how to attribute code fragments copied from other projects.
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

    pub fn downcast<E: AnyEvent>(self) -> Result<E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(*self.event.downcast::<E>().unwrap());
        }
        Err(EventError::WrongType(TypeId::of::<E>()))
    }

    pub fn downcast_ref<E: Any>(&self) -> Result<&E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(&*self.event.downcast_ref::<E>().unwrap());
        }
        Err(EventError::WrongType(TypeId::of::<E>()))
    }
}

#[derive(Default, Debug)]
pub struct EventBus {
    pub event_queue: Vec<EventBox>,
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

impl<'a> IntoIterator for &'a mut EventBus {
    type Item = EventBox;
    type IntoIter = EventBusIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EventBusIterator { event_queue: self }
    }
}

pub struct EventBusIterator<'a> {
    event_queue: &'a mut EventBus,
}

// TODO: Verify if the iterator should dequeue and drain itself.
impl<'a> Iterator for EventBusIterator<'a> {
    type Item = EventBox;

    fn next(&mut self) -> Option<EventBox> {
        self.event_queue.dequeue()
    }
}


