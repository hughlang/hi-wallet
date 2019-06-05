/// navbar
use super::*;
use crate::controllers::*;
use crate::utils::*;

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

/// This is a simple nav bar that supports a left button, right button and title label in the middle.
/// It does not yet support multiple buttons in the left and right side. And nor does it support
/// toolbar-style nav bars which have collections of buttons (like in Material Design)
pub struct NavBar {
    pub frame: Rectangle,
    pub scene: Scene,
    pub color: Option<Color>,
    title_ptr: Option<usize>,
    left_btn_id: Option<usize>, // todo: make these a vec of usize IDs to allow multiple buttons
    right_btn_id: Option<usize>,
    layout: Option<Layout>,
}

impl NavBar {
    pub fn new(frame: &Rectangle) -> Self {
        let scene = Scene::new(frame);

        NavBar {
            frame: frame.clone(),
            scene,
            color: None,
            title_ptr: None,
            left_btn_id: None,
            right_btn_id: None,
            layout: None
        }
    }

    pub fn set_title(&mut self, title: &str) {
        let label = Label::new(&self.frame, title);
        if let Some(idx) = &self.title_ptr {
            self.scene.views[*idx] = Rc::new(RefCell::new(label));
        } else {
            self.scene.views.push(Rc::new(RefCell::new(label)));
            self.title_ptr = Some(self.scene.views.len() - 1);
        }
    }

    /// TODO: make it Option<Button> where None means remove the current button
    pub fn set_left_button(&mut self, button: Button) {
        if let Some(idx) = &self.left_btn_id {
            self.scene.controls[*idx] = Rc::new(RefCell::new(button));
        } else {
            self.scene.controls.push(Rc::new(RefCell::new(button)));
            self.left_btn_id = Some(self.scene.controls.len() - 1);
        }
    }

    /// TODO: make it Option<Button> where None means remove the current button
    pub fn set_right_button(&mut self, button: Button) {
        if let Some(idx) = &self.right_btn_id {
            self.scene.controls[*idx] = Rc::new(RefCell::new(button));
        } else {
            self.scene.controls.push(Rc::new(RefCell::new(button)));
            self.right_btn_id = Some(self.scene.controls.len() - 1);
        }
    }

    /// This layout defines a % split of 20-60-20 for the 3 sections. Each section has children nodes and
    /// only one node leaf is defined in each. Others could be added later.
    /// See: https://vislyhq.github.io/stretch/docs/rust/
    pub fn layout_views(&mut self) {

        let cell_padding = Rect {
            start: Dimension::Points(8.0),
            end: Dimension::Points(8.0),
            top: Dimension::Points(5.0),
            bottom: Dimension::Points(5.0),
            ..Default::default()
        };

        let node = Node::new(
            Style {
                size: Size { width: Dimension::Points(self.frame.size.x), height: Dimension::Points(50.0) },
                ..Default::default()
            },
            vec![
                &Node::new(
                    Style {
                        size: Size { width: Dimension::Percent(0.2), height: Dimension::Auto },
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        padding: cell_padding,
                        ..Default::default()
                    },
                    vec![],
                ),
                &Node::new(
                    Style {
                        size: Size { width: Dimension::Percent(0.6), height: Dimension::Auto },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: cell_padding,
                        ..Default::default()
                    },
                    vec![],
                ),
                &Node::new(
                    Style {
                        size: Size { width: Dimension::Percent(0.2), height: Dimension::Auto },
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::Center,
                        padding: cell_padding,
                        ..Default::default()
                    },
                    vec![],
                ),
            ],
        );

        if let Some(idx) = &self.left_btn_id {
            let cell = &mut self.scene.controls[*idx];
            let size = (cell.borrow()).get_content_size();
            let node_size = Size { width: size.x, height: size.y };
            let leaf = Node::new_leaf(Style::default(), Box::new(move |_| Ok(node_size)));
            node.children()[0].add_child(&leaf);
        }
        if let Some(idx) = &self.right_btn_id {
            let cell = &mut self.scene.controls[*idx];
            let size = (cell.borrow()).get_content_size();
            let node_size = Size { width: size.x, height: size.y };
            let leaf = Node::new_leaf(Style::default(), Box::new(move |_| Ok(node_size)));
            node.children()[2].add_child(&leaf);
        }
        if let Some(idx) = &self.title_ptr {
            let cell = &mut self.scene.views[*idx];
            let size = (cell.borrow()).get_content_size();
            let node_size = Size { width: size.x, height: size.y };
            let leaf = Node::new_leaf(Style::default(), Box::new(move |_| Ok(node_size)));
            node.children()[1].add_child(&leaf);
        }

        let layout = node.compute_layout(Size::undefined()).unwrap();

        let solver = LayoutSolver {};
        let abs_layout = solver.absolute_layout(&layout);
        eprintln!("node_layout={:#?}", abs_layout);

        if let Some(idx) = &self.left_btn_id {
            let item = &abs_layout.children[0].children[0];
            eprintln!("{} left={:?}", idx, item.location);
            let cell = &mut self.scene.controls[*idx];
            (cell.borrow_mut()).set_origin(&Vector::new(item.location.x, item.location.y));
        }
        if let Some(idx) = &self.right_btn_id {
            let item = &abs_layout.children[2].children[0];
            eprintln!("{} right={:?}", idx, item.location);
            let cell = &mut self.scene.controls[*idx];
            (cell.borrow_mut()).set_origin(&Vector::new(item.location.x, item.location.y));
        }

        self.layout = Some(layout);
    }

    /// First renders the background and then the scene content
    pub fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        if let Some(color) = &self.color {
            window.draw(&self.frame, Col(*color));
        }
        let _ = self.scene.render(theme, window);
    }
}

pub struct NavItem {
    /// A custom id value used to uniquely identify the button click
    pub tag: u32,
    /// A simple button which has been pre-sized to fit within the narrow space of navbar items
    pub button: Button,
    /// A tuple of pixel padding values in the format: left, top, right, bottom
    pub padding: (f32, f32, f32, f32),
}

impl NavItem {
    pub fn new(tag: u32, button: Button) -> Self {
        NavItem {
            tag,
            button,
            padding: (0.0, 0.0, 0.0, 0.0)
        }
    }
}