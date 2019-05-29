use super::*;

use std::cell::RefCell;
use std::rc::Rc;


use quicksilver::{
    geom::{Vector}
};
use tweek::{
    gui::{Button, Label}
};

pub struct NavController {
    // navbar: NavBar,
    pub controller_stack: Vec<Rc<RefCell<Controller>>>,
    pub modal_controller: Option<Rc<RefCell<Controller>>>,
    pub show_nav: bool,
}

impl NavController {
    pub fn new() -> Self {
        // let navbar = NavBar::new(Vector::new())
        NavController {
            controller_stack: Vec::new(),
            modal_controller: None,
            show_nav: true,
        }
    }
}

pub struct NavBar {
    pub size: Vector,
    pub title_label: Option<Label>,
    pub left_button: Option<Button>,
    pub right_button: Option<Button>,
}

impl NavBar {
    pub fn new(size: Vector) -> Self {
        NavBar {
            size,
            title_label: None,
            left_button: None,
            right_button: None,
        }
    }
}