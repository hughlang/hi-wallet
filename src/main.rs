// use tweek::prelude::*;
use hi_wallet::application::*;

use tweek::quicksilver::{
    geom::Vector,
    lifecycle::{run_with, Settings},
};

fn main() {
    let screen = Vector::new(800, 600);
    run_with("Hi Wallet", screen, Settings::default(), || Application::new(screen));
}
