use super::*;
use crate::controllers::*;
use crate::events::*;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use quicksilver::{
    geom::{Rectangle, Vector},
    lifecycle::Window
};

use tweek::{
    core::{TKState},
    gui::{TKDisplayable, TKResponder, Theme},
};

// Magic numbers for different nav commands
pub const BACK_BUTTON: u32 = 10;
pub const NEXT_BUTTON: u32 = 20;

pub struct NavTarget {
    pub nav_event: NavEvent,
    pub controller: Rc<RefCell<Controller>>,
}

#[allow(dead_code)]
pub struct NavController {
    frame: Rectangle,
    /// The controllers in the navigation stack.
    controllers: Vec<Rc<RefCell<Controller>>>,
    /// Optional controller that can appear above this NavController
    modal_controller: Option<Rc<RefCell<Controller>>>,
    /// The index of the front view controller in the stack. Usually the last one, but not always.
    front_idx: usize,
    /// The standard nav bar which has buttons on left and right side. Should be optional later
    navbar: NavBar,
    pub show_nav: bool,
    events: Rc<RefCell<EventQueue>>,
    next_target: Option<NavTarget>,
}

// impl Copy for NavController { }

// impl Clone for NavController {
//     fn clone(&self) -> NavController {
//         *self
//     }
// }

impl NavController {
    pub fn new(frame: Rectangle) -> Self {
        let nav_frame = Rectangle::new((0.0, 0.0), (frame.x(), 50.0));
        let navbar = NavBar::new(&nav_frame);

        let nav = NavController {
            frame: frame,
            controllers: Vec::new(),
            modal_controller: None,
            front_idx: 0,
            navbar,
            show_nav: true,
            events: EventQueue::new(),
            next_target: None,
        };

        // This shit don't work
        // let weakself = Rc::downgrade(&Rc::new(RefCell::new(nav)));
        // nav.events.borrow_mut().set_delegate(weakself);
        nav
    }

    pub fn show(&mut self, controller: Rc<RefCell<Controller>>) {
        self.controllers.push(controller);
        self.front_idx = self.controllers.len() - 1;
        // TODO: Transition
    }

    pub fn notify(&self, message: &str) {
        eprintln!("nav message={:?}", message);
    }

    /// Unused
    pub fn process_events(&mut self, ctx: &mut AppContext) {
       // Create a list of all the nav events being requested. Usually, there's only 1.
        // However, there may be a need to resolve which nav action should take precedence.

        let mut nav_events: Vec<NavEvent> = Vec::new();
        // Review the EventBus, which will contain messages propogated from child controllers
        for event in ctx.event_bus.into_iter() {
            // Only listen for NavEvents
            if let Ok(_evt) = event.downcast_ref::<NavEvent>() {
                // match event {
            }
        }


        // Review events in the queue.
        for event in self.events.borrow_mut().queue() {
            match event.action {
                Action::Button(tag) => {
                    match tag {
                        BACK_BUTTON => {
                            // Create NavEvent to pop nav controller and add to EventBus
                        }
                        NEXT_BUTTON => {
                            // Create NavEvent to pop nav controller and add to EventBus
                            let nav_event = NavEvent::Next;
                            nav_events.push(nav_event);
                        }
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        if let Some(controller) = &mut self.controllers.get_mut(self.front_idx) {
            eprintln!("nav_events count={:?}", nav_events.len());
            if let Some(nav_event) = nav_events.pop() {
                if let Some(target) = controller.borrow_mut().get_nav_target(&nav_event) {
                    self.next_target = Some(target);
                    // self.show(target.controller);
                }
            }
        }
    }
}

impl Controller for NavController {
    fn view_will_load(&mut self) {

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
                let evt = Event::new(Action::Button(tag));
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
                let evt = Event::new(Action::Button(tag));
                notifier.notify(evt);
            });
            self.navbar.add_right_button(btn);
        }

        self.navbar.set_title(controller.screen_title());
        self.navbar.layout_views();
        (&mut *controller).view_will_load();
    }

    #[allow(dead_code)]
    #[allow(unreachable_patterns)]
    fn update(&mut self, ctx: &mut AppContext, window: &mut Window) {
        let mut nav_event: Option<NavEvent> = None;
        if let Some(event) = self.events.borrow_mut().queue().first() {
            match event.action {
                Action::Button(tag) => {
                    match tag {
                        BACK_BUTTON => { nav_event = Some(NavEvent::Back) },
                        NEXT_BUTTON => { nav_event = Some(NavEvent::Next) },
                        _ => {}
                    }
                },
                Action::Selected(idx) => { nav_event = Some(NavEvent::Selected(idx)) },
                _ => {}
            }
        }
        if let Some(evt) = nav_event {
            if let Some(controller) = &mut self.controllers.get_mut(self.front_idx) {
                if let Some(target) = controller.borrow_mut().get_nav_target(&evt) {
                    self.next_target = Some(target);
                }
            }
            ctx.event_bus.register_event(evt);
        }

        if let Some(controller) = &mut self.controllers.get_mut(self.front_idx) {
            controller.borrow_mut().update(ctx, window);
        }
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        // let _ = self.scene.render(theme, window);
        let _ = self.navbar.render(theme, window);
        if let Some(cell) = &mut self.controllers.get_mut(self.front_idx) {
            (cell.borrow_mut()).render(theme, window);
        }
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        self.navbar.scene.handle_mouse_at(pt)
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        println!(">>> NAV handle_mouse_down");
        self.navbar.scene.handle_mouse_down(pt, state);
        if let Some(cell) = &mut self.controllers.get_mut(self.front_idx) {
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

