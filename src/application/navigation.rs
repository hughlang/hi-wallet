use super::*;

use std::cell::RefCell;
use std::rc::Rc;

use quicksilver::lifecycle::Window;
use stretch::{geometry::Size, node::Node, result::Layout, style::*};
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
    pub size: (f32, f32),
    pub title_label: Option<Label>,
    pub left_button: Option<Button>,
    pub right_button: Option<Button>,
    layout: Option<Layout>,
}

impl NavBar {
    pub fn new(size: (f32, f32)) -> Self {
        NavBar {
            size,
            title_label: None,
            left_button: None,
            right_button: None,
            layout: None
            }
    }

    pub fn layout_views(&mut self) {
        let node = Node::new(
            Style {
                size: Size { width: Dimension::Points(self.size.0), height: Dimension::Points(self.size.1) },
                ..Default::default()
            },
            vec![
                &Node::new(
                    Style {
                        size: Size { width: Dimension::Percent(0.2), height: Dimension::Auto },
                        justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                    vec![],
                ),
                &Node::new(
                    Style {
                        size: Size { width: Dimension::Percent(0.6), height: Dimension::Auto },
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    vec![],
                ),
                &Node::new(
                    Style {
                        size: Size { width: Dimension::Percent(0.2), height: Dimension::Auto },
                        justify_content: JustifyContent::FlexEnd,
                        ..Default::default()
                    },
                    vec![],
                ),
            ],
        );
        let left = Node::new_leaf(Style::default(), Box::new(|_| Ok(Size { width: 24.0, height: 24.0 })));
        node.children()[0].add_child(&left);

        // let center = Node::new_leaf(Style::default(), Box::new(|_| Ok(Size { width: 24.0, height: 24.0 })));
        // node.children()[2].add_child(&center);

        let right = Node::new_leaf(Style::default(), Box::new(|_| Ok(Size { width: 24.0, height: 24.0 })));
        node.children()[2].add_child(&right);

        let layout = node.compute_layout(Size::undefined()).unwrap();
        eprintln!("{:#?}", layout);

        self.layout = Some(layout);
    }
}

impl Container for NavBar {
    fn render_views(&mut self, theme: &mut Theme, window: &mut Window) {
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
