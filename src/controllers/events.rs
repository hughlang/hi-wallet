

pub trait EventListener {
    fn on_event(&self, event: u32);
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

    pub fn notify(&self, event: u32) {
        for listener in &self.listeners {
            listener.on_event(event);
        }
    }
}