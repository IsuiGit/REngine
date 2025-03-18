use crate::{
    cli::Cli,
    sdl3::SDL3,
};

use std::{
    sync::Arc,
    thread,
    time::Duration,
    sync::atomic::{AtomicBool, Ordering},
    ffi::{CStr, CString}
};

pub struct App{
    exe_type: bool,
}

impl App{
    pub fn build(args: &[String]) -> Result<App, &'static str> {
        if args.len() < 2{
            return Ok(App {exe_type: true});
        }
        if args[1] == "--cli"{
            return Ok(App {exe_type: false});
        }
        Err("To much args. Calling REngine.exe supports only --cli argument.")
    }

    pub fn run(&self) {
        if !(self.exe_type){
            let mut _sys = Cli::init().unwrap();
            Cli::run(_sys);
        }
        let mut _sdl3 = SDL3::new();
        _sdl3.run();
    }
}

pub fn run_mut_task<F>(mut task: F, run: Arc<AtomicBool>)
where
    F: FnMut() + Send + 'static,
{
    thread::spawn(move || {
        while run.load(Ordering::SeqCst) {
            task();
            thread::sleep(Duration::from_secs(1));
        }
        println!("\nsystem::run_mut_task ended succesefully\n");
    });
}
