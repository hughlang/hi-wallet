use super::*;
use crate::prelude::*;
use tweek::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

// #[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    input::{ButtonState, Key, MouseButton, MouseCursor},
    lifecycle::{Event, State, Window},
    Error, Result,
};

/// This is intended for passing around configuration and state information
/// throughout the controller/scene hierarchy.
pub struct AppContext {
    pub screen: Vector,
    pub event_bus: EventBus,
}

impl AppContext {
    pub fn new(screen: Vector) -> Self {
        AppContext {
            screen,
            event_bus: EventBus::default(),
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub struct Application {
    screen: Vector,
    theme: Theme,
    context: AppContext,
    tk_state: TKState,
    // nav_controller: NavController,
    front_controller: Option<Rc<RefCell<dyn Controller>>>,
}

impl Application {
    pub fn new(screen: Vector) -> Result<Application> {
        std::env::set_var("RUST_LOG", "main=trace,tweek=trace");

        #[cfg(not(target_arch = "wasm32"))]
        env_logger::builder().default_format_timestamp(false).default_format_module_path(false).init();
        #[cfg(not(target_arch = "wasm32"))]
        color_backtrace::install();

        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut nav = NavController::new(frame);

        let frame = Rectangle::new((0.0, 50.0), (screen.x, screen.y - 50.0));
        let home = HomeController::new(frame.clone());
        nav.push_controller(Rc::new(RefCell::new(home)));

        nav.view_will_load();

        let s = Application {
            screen,
            theme: ThemeManager::default_theme(),
            context: AppContext::new(screen),
            tk_state: TKState::new(),
            // nav_controller: nav,
            front_controller: Some(Rc::new(RefCell::new(nav))),
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
        if let Some(cell) = &mut self.front_controller {
            let mut controller = cell.borrow_mut();
            (&mut *controller).update(&mut self.context, window);

            // TODO: Read EventBus
        }

        // Dequeue events in EventBus and trigger controller lifecycle methods
        for event in self.context.event_bus.into_iter() {
            // Only listen for NavEvents
            if let Ok(evt) = event.downcast_ref::<NavEvent>() {
                /*
                For nav events, tell the nav controller to begin transition. It should have a
                "next_target" saved which identifies the next controller to transition in.
                 */
                if let Some(cell) = &mut self.front_controller {
                    cell.borrow_mut().view_will_transition(*evt);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Repaint the entire screen
        window.clear(Color::from_hex("#EEEEEE"))?;
        // Nav controller rendering: If top view controller is a navcontroller,
        // render its subviews
        if let Some(cell) = &mut self.front_controller {
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
                if let Some(cell) = &mut self.front_controller {
                    let hover = (cell.borrow_mut()).handle_mouse_at(pt);
                    if hover {
                        window.set_cursor(MouseCursor::Hand);
                    } else {
                        window.set_cursor(MouseCursor::Default);
                    }
                }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                if let Some(cell) = &mut self.front_controller {
                    (cell.borrow_mut()).handle_mouse_down(&window.mouse().pos(), &mut self.tk_state);
                }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Released) => {
                if let Some(cell) = &mut self.front_controller {
                    (cell.borrow_mut()).handle_mouse_up(&window.mouse().pos(), &mut self.tk_state);
                }
            }
            Event::MouseWheel(_xy) => {
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
            Event::Typed(_c) => {
                // self.scene.handle_key_press(*c, window);
            }
            _ => {}
        };
        Ok(())
    }
}
