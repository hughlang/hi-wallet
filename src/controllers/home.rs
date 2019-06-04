use super::*;
use crate::application::*;

use std::cell::RefCell;
use std::rc::Rc;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color},
    lifecycle::{Window},
};

use tweek::{
    core::{TKState},
    gui::{Button, Scene, ShapeView, TKDisplayable, TKResponder, Theme},
    shared::DrawShape,
};

pub struct HomeController {
    scene: Scene,
    navbar: NavBar,
    events: Rc<RefCell<EventQueue>>,
}

impl HomeController {
    pub fn new(screen: Vector) -> HomeController {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(&frame);

        let frame = Rectangle::new((10.0, 70.0), (screen.x - 20.0, screen.y - 90.0));
        let line_color = Color::from_hex("#CCCCCC");
        let mut mesh = DrawShape::rectangle(&frame, None, Some(line_color), 1.0, 0.0);
        let shape = ShapeView::new(frame).with_mesh(&mut mesh);
        scene.views.push(Rc::new(RefCell::new(shape)));

        let frame = Rectangle::new((0.0, 0.0), (screen.x, 50.0));
        let navbar = NavBar::new(&frame);
        HomeController {
            scene,
            navbar,
            events: EventQueue::new(),
        }
    }
}

impl Controller for HomeController {

    fn view_will_load(&mut self) {
        self.navbar.color = Some(Color::RED);
        self.navbar.set_title("Home");

        let events = self.events.clone();
        let mut btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Back");
        btn.set_onclick(move |_action, _tk| {
            let mut notifier = Notifier::new();
            events.borrow().register_to(&mut notifier);
            let evt = Event::new(Action::Click(42));
            notifier.notify(evt);
        });
        self.navbar.set_left_button(btn);

        let events = self.events.clone();
        let mut btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Next");
        btn.set_onclick(move |_action, _tk| {
            let mut notifier = Notifier::new();
            events.borrow().register_to(&mut notifier);
            let evt = Event::new(Action::Click(43));
            notifier.notify(evt);
        });
        self.navbar.set_right_button(btn);

        self.navbar.layout_views();
    }

    fn update(&mut self, window: &mut Window) {
        let _ = self.scene.update(window);
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        let _ = self.scene.render(theme, window);
        let _ = self.navbar.render(theme, window);
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        self.navbar.scene.handle_mouse_at(pt)
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        self.navbar.scene.handle_mouse_down(pt, state);
        self.scene.handle_mouse_down(pt, state)
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        self.navbar.scene.handle_mouse_up(pt, state);
        self.scene.handle_mouse_up(pt, state)
    }

}

// impl<HomeController: FnMut(Event)> EventListener for HomeController {
//     fn on_event(&mut self, event: Event) {
//         eprintln!("event={:?}", event);
//         self(event);

//         match event.action {
//             Action::Click(_) => {

//                 // self.navbar.color = Some(Color::BLUE);
//             }
//             // _ => ()
//         }
//     }
// }
