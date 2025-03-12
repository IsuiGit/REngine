mod cli;
mod machine;
mod system;
mod win;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = system::App::build(&args);
    match app{
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        },
        Ok(app) => app.run(),
    }
}
