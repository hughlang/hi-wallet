extern crate orbtk;
use orbtk::*;
use std::{cell::Cell, rc::Rc};

static WALLET_THEME_CSS: &'static str = include_str!("wallet-theme-ext.css");

#[derive(Default)]
struct MainViewState {
  counter: Cell<i32>,
}

impl MainViewState {}

impl State for MainViewState {
  fn update(&self, context: &mut Context<'_>) {
    if let Ok(button_count_label) = context.widget().borrow_mut_property::<Label>() {
      button_count_label.0 = format!("Button count: {}", self.counter.get());
    }
  }
}

fn _create_header(text: &str) -> Template {
  TextBlock::create()
    .with_property(Label::from(text))
    .with_property(Selector::from("textblock").with_class("h1"))
}

struct MainView;

impl Widget for MainView {
  fn create() -> Template {
    let state = Rc::new(MainViewState::default());
    // let clear_state = state.clone();
    let label = SharedProperty::new(Label::from("prototype"));

    Template::default()
      .as_parent_type(ParentType::Single)
      .with_state(state.clone())
      // Nav bar
      .with_child(build_navbar()) 
      .with_shared_property(label)
      .with_debug_name("MainView")
  }
}

// Create a row for buttons and information
fn build_navbar() -> Template {
  Row::create().with_property(Selector::from("row").with_class("full"))
      .with_child(Column::create()
        .with_child(Button::create()
          .with_property(Selector::from("button").with_class("primary"))
          .with_property(Label::from("Primary")),			
        ),
      )  
      .with_child(Column::create()
        .with_child(Button::create()
          .with_property(Selector::from("button").with_class("none"))
          .with_property(Label::from("Normal")),			
      ),
    )
}

fn main() {
  let mut application = Application::default();
  let theme = format!("{}{}", WALLET_THEME_CSS, DEFAULT_THEME_CSS);
  application
    .create_window()
    .with_bounds(Bounds::new(50, 50, 600, 400))
    .with_title("Hi Wallet :: wallet0")
    .with_theme(Theme::parse(&theme))
    .with_root(MainView::create())
    .with_debug_flag(true)
    .build();
  application.run();
}
