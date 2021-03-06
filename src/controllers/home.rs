use super::*;
use crate::application::*;
use crate::events::*;

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

#[allow(dead_code)]
pub struct HomeController {
    frame: Rectangle,
    scene: Scene,
    events: Rc<RefCell<EventQueue>>,
}

impl HomeController {
    pub fn new(frame: Rectangle) -> HomeController {
        let scene = Scene::new(&frame);

        let controller = HomeController {
            frame,
            scene,
            events: EventQueue::new(),
        };

        // if let Some(nav) = nav {
        //     let rc = Rc::new(RefCell::new(nav));
        //     controller.nav = Rc::downgrade(&rc);
        // }
        controller
    }
}

impl Controller for HomeController {

    fn view_will_load(&mut self) {
        let frame = Rectangle::new((10.0, 70.0), (self.frame.width() - 20.0, self.frame.height() - 90.0));
        let line_color = Color::from_hex("#333333");
        let mut mesh = DrawShape::rectangle(&frame, None, Some(line_color), 3.0, 0.0);
        let shape = ShapeView::new(frame).with_mesh(&mut mesh);
        self.scene.views.push(Rc::new(RefCell::new(shape)));
    }

    fn screen_title(&self) -> &str {
        "Home"
    }

    fn left_nav_items(&self) -> Vec<NavItem> {
        let mut items: Vec<NavItem> = Vec::new();
        let btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Back");
        let item = NavItem::new(BACK_BUTTON, btn);
        items.push(item);
        items
    }

    fn right_nav_items(&self) -> Vec<NavItem> {
        let mut items: Vec<NavItem> = Vec::new();
        let btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Next");
        let item = NavItem::new(NEXT_BUTTON, btn);
        items.push(item);
        items
    }

    fn nav_target_for_event(&mut self, event: &NavEvent, _ctx: &mut AppContext) -> Option<NavTarget> {
        match event {
            NavEvent::Next => {
                let controller = SettingsController::new(self.frame.clone());
                let target = NavTarget {
                    nav_event: event.clone(),
                    controller: Rc::new(RefCell::new(controller))
                };
                return Some(target);
            }
            _ => ()
        }
        None
    }

    fn update(&mut self, ctx: &mut AppContext, window: &mut Window) {
        // This is just placeholder code for future consideration of what kinds of events
        // might get queued within this controller.
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
                // _ => {}
            }
        }
        if let Some(evt) = nav_event {
            ctx.event_bus.register_event(evt);
        }

        let _ = self.scene.update(window, Vector::ZERO);

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
        // if let Some(ref mut rc) = self.nav.upgrade() {
        //     let mut nav = rc.borrow_mut();
        //     (&mut *nav).notify("Booo");
        //     // rc.borrow_mut().notify("Mouse down");
        // }
        self.scene.handle_mouse_down(pt, state)
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        self.scene.handle_mouse_up(pt, state)
    }

}

