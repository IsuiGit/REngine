use libloading::{Library, Symbol};
use std::{env, io, error::Error, os::raw::c_void};

#[derive(Debug)]
pub struct Win{
    lib32: Library,
}

impl Win{
    pub fn new() -> Self{
        Self {
            lib32: load32().unwrap(),
        }
    }

    pub fn terminate(&self, pid: u32) -> Result<String, Box<dyn Error>> {
        unsafe {
            let open_process: Symbol<unsafe extern "system" fn(u32, bool, u32) -> *mut c_void> =
                self.lib32.get(b"OpenProcess").expect("Failed to load OpenProcess");
            let terminate_process: Symbol<unsafe extern "system" fn(*mut c_void, u32) -> bool> =
                self.lib32.get(b"TerminateProcess").expect("Failed to load TerminateProcess");
            let close_handle: Symbol<unsafe extern "system" fn(*mut c_void) -> bool> =
                self.lib32.get(b"CloseHandle").expect("Failed to load CloseHandle");
            let handle = open_process(1, false, pid);
            if handle.is_null() {
                return Err(Box::new(io::Error::last_os_error()));
            }
            let errno_t = terminate_process(handle, 0);
            if !(errno_t){
                return Err(Box::new(io::Error::last_os_error()));
            }
            let errno_c = close_handle(handle);
            if !(errno_c) {
                return Err(Box::new(io::Error::last_os_error()));
            }
            Ok(String::from(pid.to_string() + " terminate succesefully\n"))
        }
    }
}

fn load32() -> Result<Library, Box<dyn Error>> {
    unsafe {
        let path = env::var("windir").unwrap() + "\\System32\\kernel32.dll";
        let lib = libloading::Library::new(path)?;
        Ok(lib)
    }
}
