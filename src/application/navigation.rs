use super::*;
use crate::utils::*;

use std::cell::RefCell;
use std::rc::Rc;

use quicksilver::{
    geom::{Rectangle},
    graphics::{Background::Col, Color},
    lifecycle::Window
};
use stretch::{
    geometry::*,
    node::Node,
    result::Layout,
    style::*
};

use tweek::gui::*;

pub struct NavController {
    // navbar: NavBar,
    pub controller_stack: Vec<Rc<RefCell<Controller>>>,
    pub modal_controller: Option<Rc<RefCell<Controller>>>,
    pub show_nav: bool,
}

impl NavController {
    pub fn new() -> Self {
        // let navbar = NavBar::new(Vector::new())
        NavController { controller_stack: Vec::new(), modal_controller: None, show_nav: true }
    }
}

/// This is a simple nav bar that supports a left button, right button and title label in the middle.
/// It does not yet support multiple buttons in the left and right side. And nor does it support
/// toolbar-style nav bars which have collections of buttons (like in Material Design)
pub struct NavBar {
    pub frame: Rectangle,
    pub color: Option<Color>,
    pub title_label: Option<Label>,
    pub left_button: Option<Button>,
    pub right_button: Option<Button>,
    layout: Option<Layout>,
}

impl NavBar {
    pub fn new(frame: &Rectangle) -> Self {
        NavBar {
            frame: frame.clone(),
            color: None,
            title_label: None,
            left_button: None,
            right_button: None,
            layout: None
            }
    }

    pub fn set_title(&mut self, title: &str) {
        // Create the label without position and size information. This gets done later.
        let label = Label::new(&self.frame, title);
        self.title_label = Some(label);
    }

    pub fn set_left_button(&mut self, button: Button) {
        self.left_button = Some(button);
    }

    pub fn set_right_button(&mut self, button: Button) {
        self.right_button = Some(button);
    }

    /// This layout defines a % split of 20-60-20 for the 3 sections. Each section has children nodes and
    /// only one node leaf is defined in each. Others could be added later.
    /// See: https://vislyhq.github.io/stretch/docs/rust/
    pub fn layout_views(&mut self) {

        let cell_padding = Rect {
            start: Dimension::Points(5.0),
            end: Dimension::Points(5.0),
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

        if let Some(button) = &self.left_button {
            let size = button.get_content_size();
            let node_size = Size { width: size.x, height: size.y };
            let leaf = Node::new_leaf(Style::default(), Box::new(move |_| Ok(node_size)));
            node.children()[0].add_child(&leaf);
        }

        if let Some(button) = &self.right_button {
            let size = button.get_content_size();
            let node_size = Size { width: size.x, height: size.y };
            let leaf = Node::new_leaf(Style::default(), Box::new(move |_| Ok(node_size)));
            node.children()[2].add_child(&leaf);
        }

        if let Some(title) = &self.title_label {
            let size = title.get_content_size();
            let node_size = Size { width: size.x, height: size.y };
            let center = Node::new_leaf(Style::default(), Box::new(move |_| Ok(node_size)));
            node.children()[1].add_child(&center);
        }

        let layout = node.compute_layout(Size::undefined()).unwrap();
        eprintln!("{:#?}", layout);
        // let loc = node.children()[0].location;
        let loc = layout.children[0].children[0].location;
        eprintln!("location={:?}", loc);

        let pos = self.absolute_position(&layout, vec![0, 0]);
        eprintln!("Abs position={:?}", pos);

        let mut solver = LayoutSolver {};
        let node_layout = solver.absolute_layout(&layout);

        self.layout = Some(layout);
    }

    fn absolute_position(&self, layout: &Layout, path: Vec<usize>) -> Point<f32> {
        let mut position = Point { x: 0.0, y: 0.0 };
        let mut current = layout.clone();

        for i in path {
            if i < current.children.len() {
                current = current.children[i].clone();
                let location = current.location;
                position = Point { x: position.x + location.x, y: position.y + location.y };
            } else {
                return position;
            }
        }
        position
    }
}



impl Container for NavBar {
    fn render_views(&mut self, theme: &mut Theme, window: &mut Window) {
        if let Some(color) = &mut self.color {
            window.draw(&self.frame, Col(*color));
        }

        if let Some(title) = &mut self.title_label {
            let _ = title.render(theme, window);
        }
        if let Some(button) = &mut self.left_button {
            let _ = button.render(theme, window);
        }
        if let Some(button) = &mut self.right_button {
            let _ = button.render(theme, window);
        }
    }

}

/*
Layout {
    order: 0,
    size: Size {
        width: 400.0,
        height: 50.0,
    },
    location: Point {
        x: 0.0,
        y: 0.0,
    },
    children: [
        Layout {
            order: 0,
            size: Size {
                width: 80.0,
                height: 50.0,
            },
            location: Point {
                x: 0.0,
                y: 0.0,
            },
            children: [
                Layout {
                    order: 0,
                    size: Size {
                        width: 40.0,
                        height: 50.0,
                    },
                    location: Point {
                        x: 0.0,
                        y: 0.0,
                    },
                    children: [],
                },
            ],
        },
        Layout {
            order: 1,
            size: Size {
                width: 240.0,
                height: 50.0,
            },
            location: Point {
                x: 80.0,
                y: 0.0,
            },
            children: [
                Layout {
                    order: 0,
                    size: Size {
                        width: 0.0,
                        height: 50.0,
                    },
                    location: Point {
                        x: 120.0,
                        y: 0.0,
                    },
                    children: [],
                },
            ],
        },
        Layout {
            order: 2,
            size: Size {
                width: 80.0,
                height: 50.0,
            },
            location: Point {
                x: 320.0,
                y: 0.0,
            },
            children: [
                Layout {
                    order: 0,
                    size: Size {
                        width: 40.0,
                        height: 50.0,
                    },
                    location: Point {
                        x: 40.0,
                        y: 0.0,
                    },
                    children: [],
                },
            ],
        },
    ],
}
 */