/// This events system follows the design pattern described here:
/// https://blog.rom1v.com/2017/09/gnirehtet-rewritten-in-rust/#observer

pub trait EventListener {
    fn on_event(&self, event: Event);
}

pub struct Notifier {
    listeners: Vec<Box<EventListener>>,
}

impl Notifier {
    pub fn new() -> Self {
        Self { listeners: Vec::new() }
    }

    pub fn register<T: EventListener + 'static>(&mut self, listener: T) {
        self.listeners.push(Box::new(listener));
    }

    pub fn notify(&self, event: Event) {
        for listener in &self.listeners {
            listener.on_event(event);
        }
    }
}

// *****************************************************************************************************
// Model objects for passing around event info
// *****************************************************************************************************

pub enum Action {
    Click(u32),
}

pub struct Event {
    pub action: Action,
}

impl Event {
    pub fn new(action: Action) -> Self {
        Event { action }
    }
}

pub struct EventQueue {
    pub events: Vec<Event>
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            events: Vec::new()
        }
    }

    pub fn store(&mut self, evt: Event) {
        self.events.push(evt);
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }
}

