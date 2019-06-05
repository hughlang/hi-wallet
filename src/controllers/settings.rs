use super::*;
use crate::application::*;
use crate::events::*;

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

#[allow(dead_code)]
pub struct SettingsController {
    frame: Rectangle,
    scene: Scene,
    // nav: Weak<RefCell<NavController>>,
    events: Rc<RefCell<EventQueue>>,
}

impl SettingsController {
    pub fn new(frame: Rectangle) -> SettingsController {
        let scene = Scene::new(&frame);
        let controller = SettingsController {
            frame,
            scene,
            events: EventQueue::new(),
        };
        controller
    }
}

impl Controller for SettingsController {

    fn view_will_load(&mut self) {
        let frame = Rectangle::new((10.0, 70.0), (self.frame.x() - 20.0, self.frame.y() - 90.0));
        let line_color = Color::from_hex("#FFD700");
        let mut mesh = DrawShape::rectangle(&frame, None, Some(line_color), 3.0, 0.0);
        let shape = ShapeView::new(frame).with_mesh(&mut mesh);
        self.scene.views.push(Rc::new(RefCell::new(shape)));


    }

    fn screen_title(&self) -> &str {
        "Settings"
    }

    fn left_nav_items(&self) -> Vec<NavItem> {
        let mut items: Vec<NavItem> = Vec::new();
        let btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Back");
        let item = NavItem::new(BACK_BUTTON, btn);
        items.push(item);
        items
    }

    // fn right_nav_items(&self) -> Vec<NavItem> {
    //     let mut items: Vec<NavItem> = Vec::new();
    //     let btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Next");
    //     let item = NavItem::new(NEXT_BUTTON, btn);
    //     items.push(item);
    //     items
    // }

    fn get_nav_target(&mut self, evt: NavEvent) -> Option<NavTarget> {
        // let controller = SettingsCont
        None

    }

    fn update(&mut self, ctx: &mut AppContext, window: &mut Window) {
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

