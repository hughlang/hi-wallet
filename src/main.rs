// use tweek::prelude::*;
use hi_wallet::application::*;

use tweek::quicksilver::{
    geom::Vector,
    lifecycle::{run_with, Settings},
};

fn main() {
    std::env::set_var("RUST_LOG", "main=trace,tweek=debug");

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::builder().default_format_timestamp(false).default_format_module_path(false).init();

    let screen = Vector::new(800, 600);
    run_with("Hi Wallet", screen, Settings::default(), || MainState::new(screen));
}
