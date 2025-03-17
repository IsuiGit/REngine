use crate::{machine, core, win};
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

    pub fn run(sys: machine::SystemInfo){
        let mut _sys = Arc::new(Mutex::new(sys));
        let run = Arc::new(AtomicBool::new(true));
        let task = {
            let _sys = Arc::clone(&_sys);
            move || {
                let mut sys = _sys.lock().unwrap();
                sys.update();
            }
        };
        core::run_mut_task(task, Arc::clone(&run));
        println!("\nsystem::run_mut_task started succesefully");
        println!("\nCommands to execute:\n");
        println!("#####################################################");
        println!("# sys - show system info                            #");
        println!("# mem - show memory info                            #");
        println!("# processes - system processes                      #");
        println!("# pretty processes - system processes as table view #");
        println!("# kill - process terminate by PID                   #");
        println!("# exit - to exit REngine CLI                        #");
        println!("#####################################################\n");
        println!("REngine (c) IsuiGit\n");
        loop{
            let mut com = String::new();
            let _ = io::stdin().read_line(&mut com);
            match com.trim_end() {
                "exit" => {
                    run.store(false, Ordering::SeqCst);
                    thread::sleep(Duration::from_secs(2));
                    process::exit(0);
                },
                "sys" => println!("{}\n", _sys.lock().unwrap().info_as_str().unwrap()),
                "mem" => println!("{:?}\n", _sys.lock().unwrap().mem_as_vec().unwrap()),
                "processes" => println!("{:?}\n", _sys.lock().unwrap().processes_as_vec().unwrap()),
                "pretty processes" => {
                    println!("\nPretty processes table:\n");
                    _sys.lock().unwrap().pretty_processes();
                },
                "kill" => {
                    if _sys.lock().unwrap().name().unwrap() == "Windows"{
                        let mut pid_s = String::new();
                        let _ = io::stdin().read_line(&mut pid_s);
                        let pid: u32 = match pid_s.trim().parse() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("Error: '{}' is not a valid u32\n", pid_s.trim());
                                continue;
                            },
                        };
                        let win = win::Win::new();
                        let errno = win.terminate(pid).unwrap();
                        println!("{}", errno);
                    }
                    else{
                        println!("This option is not available on your platform.\n")
                    }
                }
                _ => println!("missmatch: {}", com),
            }
        }
    }
}
