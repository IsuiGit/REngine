use sysinfo::{System, Pid, Process};
use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone)]
pub struct SystemInfo{
    name: String,
    kernel: String,
    version: String,
    host: String,
    total: u64,
    used: u64,
    processes: Vec<(u32, String)>,
}

impl Default for SystemInfo{
    fn default() -> SystemInfo{
        let sys = System::new_all();
        SystemInfo{
            name: System::name().unwrap(),
            kernel: System::kernel_version().unwrap(),
            version: System::os_version().unwrap(),
            host: System::host_name().unwrap(),
            total: sys.total_memory(),
            used: sys.used_memory(),
            processes: as_vec(sys.processes()).unwrap(),
        }
    }
}

impl SystemInfo{
    pub fn info_as_str(&self) -> Result<String, Box<dyn Error>> {
        let iterable = vec![
            self.name.clone(),
            self.kernel.clone(),
            self.version.clone(),
            self.host.clone()
        ];
        Ok(iterable.join(", "))
    }

    pub fn mem_as_vec(&self) -> Result<Vec<u64>, Box<dyn Error>> {
        Ok(vec![self.total, self.used])
    }

    pub fn processes_as_vec(&self) -> Result<Vec<(u32, String)>, Box<dyn Error>> {
        Ok(self.processes.clone())
    }

    pub fn pretty_processes(&self){
        let mut pprocesses = self.processes.clone();
        pprocesses.sort_by(|a, b| a.1.cmp(&b.1));
        for index in 1..pprocesses.len() {
            println!("pid: {}\t\t\tname: {}", pprocesses[index].0.to_string(), pprocesses[index].1.clone());
        }
        println!("\n");
    }

    pub fn name(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.name.clone())
    }

    pub fn update(&mut self){
        let sys = System::new_all();
        self.name = System::name().unwrap();
        self.kernel = System::kernel_version().unwrap();
        self.version = System::os_version().unwrap();
        self.host = System::host_name().unwrap();
        self.total = sys.total_memory();
        self.used = sys.used_memory();
        self.processes = as_vec(sys.processes()).unwrap();
    }
}

fn as_vec(processes: &HashMap<Pid, Process>) -> Result<Vec<(u32, String)>, Box<dyn Error>>{
    let mut vec: Vec<(u32, String)> = Vec::new();
    for (pid, process) in processes{
        vec.push((pid.as_u32(),process.name().to_str().expect("Err").to_string()))
    }
    Ok(vec)
}
