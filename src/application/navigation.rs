use super::*;
use crate::controllers::*;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::Window
};
use stretch::{
    geometry::*,
    node::Node,
    result::Layout,
    style::*
};

use tweek::{
    core::{TKState},
    gui::{Button, Label, Scene, TKDisplayable, TKResponder, Theme},
};

// Magic numbers for different nav commands
pub const BACK_BUTTON: u32 = 10;
pub const NEXT_BUTTON: u32 = 20;

pub struct NavController {
    /// The controllers in the navigation stack.
    pub controllers: Vec<Rc<RefCell<Controller>>>,
    /// Optional controller that can appear above this NavController
    pub modal_controller: Option<Rc<RefCell<Controller>>>,
    /// The index of the front view controller in the stack. Usually the last one, but not always.
    front_idx: usize,
    /// The standard nav bar which has buttons on left and right side. Should be optional later
    navbar: NavBar,
    pub show_nav: bool,
    events: Rc<RefCell<EventQueue>>,
}

impl NavController {
    pub fn new(screen: Vector) -> Self {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, 50.0));
        let navbar = NavBar::new(&frame);

        let nav = NavController {
            controllers: Vec::new(),
            modal_controller: None,
            front_idx: 0,
            navbar,
            show_nav: true,
            events: EventQueue::new(),
        };
        // nav.events.borrow_mut().set_delegate(Rc::new(RefCell::new(nav)));
        nav
    }

    pub fn show(&mut self, controller: Rc<RefCell<Controller>>) {
        self.controllers.push(controller);
        // TODO: Transition
    }

    pub fn notify(&self, message: &str) {
        eprintln!("nav message={:?}", message);
    }
}

impl Controller for NavController {
    fn view_will_load(&mut self) {
        // let rc = Rc::new(RefCell::new(self));
        // self.events.borrow_mut().add_handler(rc);

        let theme = ThemeManager::nav_theme();
        self.navbar.color = Some(theme.bg_color);

        if self.front_idx >= self.controllers.len() {
            return;
        }
        let mut controller = self.controllers[self.front_idx].borrow_mut();

        for item in controller.left_nav_items() {
            let events = self.events.clone();
            let mut btn = item.button;
            let tag = item.tag;
            btn.set_onclick(move |_action, _tk| {
                let mut notifier = Notifier::new();
                events.borrow().register_to(&mut notifier);
                let evt = Event::new(Action::Click(tag));
                notifier.notify(evt);
            });
            self.navbar.add_left_button(btn);
        }

        for item in controller.right_nav_items() {
            let events = self.events.clone();
            let mut btn = item.button;
            let tag = item.tag;
            btn.set_onclick(move |_action, _tk| {
                let mut notifier = Notifier::new();
                events.borrow().register_to(&mut notifier);
                let evt = Event::new(Action::Click(tag));
                notifier.notify(evt);
            });
            self.navbar.add_right_button(btn);
        }

        self.navbar.set_title(controller.screen_title());
        self.navbar.layout_views();
        (&mut *controller).view_will_load();
    }

    fn update(&mut self, window: &mut Window) {
        // let mut events = self.events.borrow_mut().queue();
        for event in self.events.borrow_mut().queue() {
            match event.action {
                Action::Click(_) => {

                }
            }
        }
        self.events.borrow_mut().queue().clear();
        if let Some(cell) = &mut self.controllers.last() {
            (cell.borrow_mut()).update(window);
        }
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        // let _ = self.scene.render(theme, window);
        let _ = self.navbar.render(theme, window);
        if let Some(cell) = &mut self.controllers.last() {
            (cell.borrow_mut()).render(theme, window);
        }
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        self.navbar.scene.handle_mouse_at(pt)
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        println!(">>> NAV handle_mouse_down");
        self.navbar.scene.handle_mouse_down(pt, state);
        if let Some(cell) = &mut self.controllers.last() {
            (cell.borrow_mut()).handle_mouse_down(pt, state);
        }
        false
        // self.scene.handle_mouse_down(pt, state)
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        self.navbar.scene.handle_mouse_up(pt, state)
        // self.scene.handle_mouse_up(pt, state)
    }

}

impl EventDelegate for NavController {
    fn handle_event(&mut self, event: Event) {
        eprintln!("NavController handle_event: {:?}", event);
    }
}
