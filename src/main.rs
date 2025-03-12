mod cli;
mod machine;
mod system;
mod win;

use cli::Cli;

fn main() {
    let mut _sys = Cli::init().unwrap();
    Cli::cli(_sys);
}
