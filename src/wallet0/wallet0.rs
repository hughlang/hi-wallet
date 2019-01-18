extern crate orbtk;
use orbtk::*;
use std::{cell::Cell, rc::Rc};

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

fn create_header(text: &str) -> Template {
  TextBlock::create()
    .with_property(Label::from(text))
    .with_property(Selector::from("textblock").with_class("h1"))
}

fn make_row() -> Template {
  Row::create().with_property(Selector::from("row").with_class("space"))
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
      .with_child(
        make_row().with_child(build_column1())
          .with_child(Column::create()
            .with_child(Container::create().with_child(create_header("Text")))
            .with_child(
              Container::create().with_child(
                TextBlock::create()
                  .with_shared_property(label.clone())
                  .with_property(Selector::from("textblock").with_class("fheight")),
              ),
            ),
        ),
			)
      .with_shared_property(label)
      .with_debug_name("MainView")
  }
}

// Create a row for buttons and information
fn _build_navbar() -> Template {
  Row::create().with_property(Selector::from("row").with_class("space"))	
}

fn build_column1() -> Template {
  let column = Column::create()
    .with_child(Container::create().with_child(create_header("Buttons")))
    .with_child(
      Container::create().with_child(
        Button::create()
          .with_property(Selector::from("button").with_class("primary"))
          .with_property(Label::from("Primary")),
      ),
    )
    .with_child(
      Container::create()
        .with_child(ToggleButton::create().with_property(Label::from("ToggleButton"))),
    )
    .with_child(
      Container::create().with_child(CheckBox::create().with_property(Label::from("CheckBox"))),
    )
    .with_child(Container::create().with_child(Switch::create()));
  column
}

fn main() {
  let mut application = Application::default();
  // let theme = format!("{}{}", LIGHT_THEME_EXTENSION, LIGHT_THEME_EXTENSION_CSS);
  application
    .create_window()
    .with_bounds(Bounds::new(50, 50, 600, 400))
    .with_title("Hi Wallet :: wallet0")
    // .with_theme(Theme::parse(&theme))
    .with_root(MainView::create())
    .with_debug_flag(true)
    .build();
  application.run();
}
