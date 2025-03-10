use crate::machine;
use crate::system;
use std::{
    error::Error,
    io,
    process,
    thread,
    time::Duration,
    sync::{Arc, Mutex},
    sync::atomic::{AtomicBool, Ordering}
};

#[derive(Debug)]
pub struct Cli;


impl Cli{
    pub fn init() -> Result<machine::SystemInfo, Box<dyn Error>> {
        Ok(machine::SystemInfo::default())
    }

    pub fn cli(sys: machine::SystemInfo){
        // Описать в доукментации эту часть
        let mut _sys = Arc::new(Mutex::new(sys));
        let run = Arc::new(AtomicBool::new(true));
        let task = {
            let _sys = Arc::clone(&_sys);
            move || {
                let mut sys = _sys.lock().unwrap();
                sys.update();
            }
        };
        system::run_mut_task(task, Arc::clone(&run));
        // Это и так понятно
        println!("\nsystem::run_mut_task started succesefully");
        println!("\nCommands to execute:\n");
        println!("################################");
        println!("# sys - show system info       #");
        println!("# mem - show memory info       #");
        println!("# processes - system processes #");
        println!("# exit - to exit REngine CLI   #");
        println!("################################\n");
        println!("REngine (c) IsuiGit\n");
        loop{
            let mut com = String::new();
            let _ = io::stdin().read_line(&mut com);
            match com.trim_end() {
                "exit" => {
                    run.store(false, Ordering::SeqCst); // Это тоже описать в доке
                    thread::sleep(Duration::from_secs(1));
                    process::exit(0);
                },
                "sys" => println!("{}\n", _sys.lock().unwrap().info_as_str().unwrap()),
                "mem" => println!("{:?}\n", _sys.lock().unwrap().mem_as_vec().unwrap()),
                "processes" => println!("{:?}\n", _sys.lock().unwrap().processes_as_vec().unwrap()),
                _ => println!("missmatch: {}", com),
            }
        }
    }
}
