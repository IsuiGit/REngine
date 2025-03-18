mod cli;
mod machine;
mod core;
mod win;
mod sdl3;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = core::App::build(&args);
    match app {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        },
        Ok(app) => app.run(),
    }
}
