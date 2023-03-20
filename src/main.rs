pub mod config;
pub mod core;

use config::Config;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    let config = &Config::new(filename);

    let mut app = core::app::App::new(config);

    if let Err(err) = app.run() {
        println!("Error: {}", err);
    };
}
