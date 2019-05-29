use super::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct NavController {
    pub controller_stack: Vec<Rc<RefCell<Controller>>>,
    pub modal_controller: Option<Rc<RefCell<Controller>>>,
}

impl NavController {
    pub fn new() -> Self {
        NavController {
            controller_stack: Vec::new(),
            modal_controller: None,
        }
    }
}