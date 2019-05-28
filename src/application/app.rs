// use tweek::prelude::*;

#[allow(unused_imports)]
use tweek::quicksilver::{
    geom::{Circle, Line, Rectangle, Scalar, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run_with, Asset, Event, Settings, State, Window},
    Error, Result,
};

#[allow(dead_code)]
#[allow(unused_variables)]
pub struct MainState {
    screen: Vector,
}

impl MainState {
    pub fn new(screen: Vector) -> Result<MainState> {
        let s = MainState {
            screen
        };
        Ok(s)
    }
}

impl State for MainState {
    // Initialize the struct
    fn new() -> Result<MainState> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Remove any lingering artifacts from the previous frame
        window.clear(Color::WHITE)?;
        // Draw a rectangle with a top-left corner at (100, 100) and a width and height of 32 with
        // a blue background
        // Draw a triangle with a red background, rotated by 45 degrees, and scaled down to half
        // its size
        window.draw_ex(
            &Triangle::new((500, 50), (450, 100), (650, 150)),
            Col(Color::RED),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            0,
        );
            // Only take up half the screen with the immi widgets
            // .rescale(0.5, 0.5, &Alignment::center())
            ;

        // We completed with no errors
        Ok(())
    }
}