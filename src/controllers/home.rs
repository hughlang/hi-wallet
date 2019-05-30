use crate::application::*;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Circle, Line, Rectangle, Scalar, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{run_with, Asset, Event, Settings, State, Window},
    Error, Result,
};

use tweek::{
    gui::{Button, Scene, ShapeView, TKDisplayable, Theme},
    shared::DrawShape,
};

pub struct HomeController {
    scene: Scene,
    navbar: NavBar,
}

impl HomeController {
    pub fn new(screen: Vector) -> Self {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(&frame);

        let mut xpos = 10.0;
        let ypos = 10.0;

        let frame = Rectangle::new((10.0, 10.0), (screen.x - 20.0, screen.y - 20.0));
        let line_color = Color::from_hex("#333333");
        let mut mesh = DrawShape::rectangle(&frame, None, Some(line_color), 1.0, 0.0);
        let shape = ShapeView::new(frame).with_mesh(&mut mesh);
        scene.views.push(Rc::new(RefCell::new(shape)));

        let frame = Rectangle::new((0.0, 0.0), (screen.x, 50.0));
        let navbar = NavBar::new(&frame);
        Self { scene, navbar }
    }
}

impl Controller for HomeController {

    fn view_will_load(&mut self) {
        self.navbar.color = Some(Color::RED);
        self.navbar.set_title("Home");
        let btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Back");
        self.navbar.set_left_button(btn);
        let btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Next");
        self.navbar.set_right_button(btn);
        self.navbar.layout_views();
    }

    fn update(&mut self, window: &mut Window) {
        let _ = self.scene.update(window);
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        self.navbar.render_views(theme, window);
        let _ = self.scene.render(theme, window);
        self.navbar.render_views(theme, window);
    }

}
