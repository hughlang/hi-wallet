// use tweek::prelude::*;
use hi_wallet::application::*;

use quicksilver::{
    geom::Vector,
    lifecycle::{run_with, Settings},
};

fn main() {
    let screen = Vector::new(400, 600);
    run_with("hi wallet!", screen, Settings::default(), || Application::new(screen));
}
