use super::*;
use crate::prelude::*;
use tweek::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Circle, Line, Rectangle, Scalar, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run_with, Asset, Event, Settings, State, Window},
    Error, Result,
};

/// This is intended for passing around configuration and state information
/// throughout the controller/scene hierarchy.
pub struct AppState {
    pub screen: (f32, f32),
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub struct Application {
    screen: Vector,
    theme: Theme,
    main_controller: Option<Rc<RefCell<Controller>>>,
}

impl Application {
    pub fn new(screen: Vector) -> Result<Application> {
        std::env::set_var("RUST_LOG", "main=trace,tweek=debug");

        #[cfg(not(target_arch = "wasm32"))]
        env_logger::builder().default_format_timestamp(false).default_format_module_path(false).init();
        let home = HomeController::new(screen);

        let s = Application {
            screen,
            theme: ThemeManager::default_theme(),
            main_controller: Some(Rc::new(RefCell::new(home))),
        };
        Ok(s)
    }
}

impl State for Application {
    // Initialize the struct
    fn new() -> Result<Application> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if let Some(cell) = &mut self.main_controller {
            let mut controller = cell.borrow_mut();
            (&mut *controller).update(window);
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Repaint the entire screen
        window.clear(Color::WHITE)?;

        if let Some(cell) = &mut self.main_controller {
            let mut controller = cell.borrow_mut();
            (&mut *controller).render(&mut self.theme, window);
        }
        Ok(())
    }
}

