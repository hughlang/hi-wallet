use super::*;
use crate::application::*;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

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

pub struct HomeController<'a> {
    scene: Scene,
    nav: Weak<RefCell<&'a NavController>>,
    events: Rc<RefCell<EventQueue>>,
}

impl<'a> HomeController<'a> {
    pub fn new(screen: Vector, nav: Option<&NavController>) -> HomeController {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(&frame);

        let frame = Rectangle::new((10.0, 70.0), (screen.x - 20.0, screen.y - 90.0));
        let line_color = Color::from_hex("#CCCCCC");
        let mut mesh = DrawShape::rectangle(&frame, None, Some(line_color), 1.0, 0.0);
        let shape = ShapeView::new(frame).with_mesh(&mut mesh);
        scene.views.push(Rc::new(RefCell::new(shape)));

        // let frame = Rectangle::new((0.0, 0.0), (screen.x, 50.0));
        // let navbar = NavBar::new(&frame);

        let mut controller = HomeController {
            scene,
            nav: Weak::new(),
            events: EventQueue::new(),
        };
        if let Some(nav) = nav {
            let rc = Rc::new(RefCell::new(nav));
            controller.nav = Rc::downgrade(&rc);
        }
        controller
    }
}

impl<'a> Controller for HomeController<'a> {

    fn update(&mut self, window: &mut Window) {
        // let mut events = self.events.borrow_mut().queue();
        // (*events).clear();
        // for event in events.drain(..) {

        // }
        // *events;
        let _ = self.scene.update(window);
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        let _ = self.scene.render(theme, window);
        // let _ = self.navbar.render(theme, window);
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        self.scene.handle_mouse_at(pt)

    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        println!(">>> handle_mouse_down");
        if let Some(ref mut rc) = self.nav.upgrade() {
            let mut nav = rc.borrow_mut();
            (&mut *nav).notify("Booo");
            // rc.borrow_mut().notify("Mouse down");
        }
        self.scene.handle_mouse_down(pt, state)
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        self.scene.handle_mouse_up(pt, state)
    }

}

