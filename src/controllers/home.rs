use crate::application::*;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused_imports)]
use tweek::quicksilver::{
    geom::{Circle, Line, Rectangle, Scalar, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{run_with, Asset, Event, Settings, State, Window},
    Error, Result,
};

use tweek::{
    quicksilver_ui::{Scene, ShapeView, Theme, TKDisplayable, TKResponder},
    shared::{DrawShape},
};

pub struct HomeController {
    scene: Scene,
}

impl HomeController {
    pub fn new(screen: Vector) -> Self {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(&frame);

        let mut xpos = 100.0;
        let ypos = 100.0;

        let frame = Rectangle::new((xpos, ypos), (120.0, 60.0));
        let fill_color = Color::from_hex("#CCCCCC");
        let line_color = Color::from_hex("#333333");
        let mut rounded = DrawShape::rectangle(&frame, Some(fill_color), Some(line_color), 2.0, 3.0);
        let shape = ShapeView::new(frame).with_mesh(&mut rounded);
        scene.views.push(Rc::new(RefCell::new(shape)));

        xpos += 180.0;
        let frame = Rectangle::new((xpos, ypos), (180.0, 60.0));
        let line_color = Color::from_hex("#333333");
        let mut mesh = DrawShape::rectangle(&frame, None, Some(line_color), 2.0, 0.0);
        let shape = ShapeView::new(frame).with_mesh(&mut mesh);
        scene.views.push(Rc::new(RefCell::new(shape)));

        Self {
            scene
        }
    }
}

impl Controller for HomeController {

    fn update(&mut self, _window: &mut Window) {
        let _ = self.scene.update();
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        let _ = self.scene.render(theme, window);
    }

    fn view_will_load(&mut self) {

    }

}