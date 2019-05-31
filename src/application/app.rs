use super::*;
use crate::prelude::*;
use tweek::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

// #[allow(unused_imports)]
use quicksilver::{
    geom::Vector,
    graphics::Color,
    input::{ButtonState, Key, MouseButton, MouseCursor},
    lifecycle::{Event, State, Window},
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
    tk_state: TKState,
    main_controller: Option<Rc<RefCell<Controller>>>,
}

impl Application {
    pub fn new(screen: Vector) -> Result<Application> {
        std::env::set_var("RUST_LOG", "main=trace,tweek=debug");

        #[cfg(not(target_arch = "wasm32"))]
        env_logger::builder().default_format_timestamp(false).default_format_module_path(false).init();
        let mut home = HomeController::new(screen);
        home.view_will_load();

        let s = Application {
            screen,
            theme: ThemeManager::default_theme(),
            tk_state: TKState::new(),
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
        // Nav controller rendering: If top view controller is a navcontroller,
        // render its subviews
        if let Some(cell) = &mut self.main_controller {
            (cell.borrow_mut()).render(&mut self.theme, window);
        }
        Ok(())
    }

    #[allow(unused_assignments)]
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Focused => {
                log::debug!("size={:?} y={:?}", window.screen_size(), 0);
            }
            Event::MouseMoved(pt) => {
                if let Some(cell) = &mut self.main_controller {
                    let hover = (cell.borrow_mut()).handle_mouse_at(pt);
                    if hover {
                        window.set_cursor(MouseCursor::Hand);
                    } else {
                        window.set_cursor(MouseCursor::Default);
                    }
                }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                if let Some(cell) = &mut self.main_controller {
                    (cell.borrow_mut()).handle_mouse_down(&window.mouse().pos(), &mut self.tk_state);
                }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Released) => {
                if let Some(cell) = &mut self.main_controller {
                    (cell.borrow_mut()).handle_mouse_up(&window.mouse().pos(), &mut self.tk_state);
                }
            }
            Event::MouseWheel(xy) => {
                // self.scene.handle_mouse_scroll(xy, &mut self.tk_state);
            }
            Event::Key(key, ButtonState::Pressed) => match key {
                Key::Escape => {
                    window.close();
                }
                _ => {
                    // self.scene.handle_key_command(key, window);
                }
            },
            Event::Typed(c) => {
                // self.scene.handle_key_press(*c, window);
            }
            _ => {}
        };
        Ok(())
    }
}
