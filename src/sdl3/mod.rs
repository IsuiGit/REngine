mod sdl3_types;
pub mod sdl3_consts;
mod sdl3_structs;

use sdl3_types::*;
use sdl3_consts::*;
use sdl3_structs::*;

use libloading::{Library, Symbol};
use std::{path::Path, ffi::{c_void, CString, CStr}};

pub struct SDL3{
    lib: Library,
}

impl SDL3{
    pub fn new() -> Self {
        unsafe {
            let lib = Library::new(Path::new("src/sdl3/SDL3.dll")).expect("Failed to load SDL3.dll");
            SDL3 {lib}
        }
    }

    pub fn run(&mut self){
        self.sdl3_init(SDL_INIT_VIDEO);
        let window = self.sdl3_create_window("Test Window!", 800, 600, SDL_VOID);
        if window.is_null(){
            eprintln!("Failed to create window!");
            return;
        }
        self.sdl3_poll_event(16);
        self.sdl3_destroy_window(window);
        self.sdl3_quit();
    }

    pub fn sdl3_init(&mut self, flags: u32) {
        unsafe {
            let _sdl3_init: Symbol<SDL_Init> = self.lib.get(b"SDL_Init").expect("Failed to get symbol SDL_Init");
            if !_sdl3_init(flags) {
                panic!("SDL could not initialize! SDL_Error: {}", self.sdl3_get_error());
            }
        }
    }

    pub fn sdl3_quit(&mut self){
        unsafe {
            let _sdl3_quit: Symbol<SDL_Quit> = self.lib.get(b"SDL_Quit").expect("Failed to get symbol SDL_Quit");
            _sdl3_quit();
        }
    }

    pub fn sdl3_create_window(&mut self, title: &str, w:u32, h:u32, flags:u32) -> *mut c_void {
        unsafe{
            let title_ptr = CString::new(title).unwrap();
            let _sdl3_create_window: Symbol<SDL_CreateWindow> = self.lib.get(b"SDL_CreateWindow").expect("Failed to get symbol SDL_CreateWindow");
            _sdl3_create_window(title_ptr.as_ptr(), w, h, flags)
        }
    }

    pub fn sdl3_destroy_window(&mut self, window: *mut c_void){
        unsafe {
            let _sdl3_destroy_window: Symbol<SDL_DestroyWindow> = self.lib.get(b"SDL_DestroyWindow").expect("Failed to get symbol SDL_DestroyWindow");
            _sdl3_destroy_window(window);
        }
    }

    pub fn sdl3_poll_event(&mut self, ms: u32) {
        unsafe {
            let _sdl3_poll_event: Symbol<SDL_PollEvent> = self.lib.get(b"SDL_PollEvent").expect("Failed to get symbol SDL_PollEvent");
            let mut run = true;
            while run{
                let mut event: SDL_Event = SDL_Event{ type_: 0 };
                while _sdl3_poll_event(&mut event) {
                    match event.type_ {
                        SDL_EVENT_QUIT => {
                            run = false;
                        },
                        _ => {
                            println!("Unhandled event type: {}", event.type_);
                        }
                    }
                }
                if ms > 0{
                    let _sdl3_delay: Symbol<SDL_Delay> = self.lib.get(b"SDL_Delay").expect("Failed to get symbol SDL_Delay");
                    _sdl3_delay(ms);
                }
            }
        }
    }

    pub fn sdl3_get_error(&mut self) -> String {
        unsafe {
            let _sdl3_get_error: Symbol<SDL_GetError> = self.lib.get(b"SDL_GetError").expect("Failed to get symbol SDL_GetError");
            let c_err = CStr::from_ptr(_sdl3_get_error());
            c_err.to_string_lossy().into_owned()
        }
    }
}
