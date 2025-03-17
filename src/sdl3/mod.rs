mod sdl3_types;
mod sdl3_consts;
use sdl3_types::*;
use sdl3_consts::*;

use libloading::{Library, Symbol};
use std::path::Path;

pub struct SDL3{
    lib: Library,
}

impl Default for SDL3{
    fn default() -> SDL3{
        SDL3{
            lib: load_sdl3().unwrap(),
        }
    }
}

impl SDL3{
    pub fn sdl3_init(&mut self) -> bool{
        unsafe {
            let _sdl3_init: Symbol<SDL_Init> = self.lib.get(b"SDL_Init").expect("Failed to get symbol SDL_Init");
            _sdl3_init(SDL_INIT_VIDEO)
        }
    }

    pub fn sdl3_quit(&mut self){
        unsafe {
            let _sdl3_quit: Symbol<SDL_Quit> = self.lib.get(b"SDL_Quit").expect("Failed to get symbol SDL_Quit");
            _sdl3_quit();
        }
    }

    pub fn sdl3_get_error(&mut self) -> *const i8{
        unsafe {
            let _sdl3_get_error: Symbol<SDL_GetError> = self.lib.get(b"SDL_GetError").expect("Failed to get symbol SDL_GetError");
            _sdl3_get_error()
        }
    }
}

fn load_sdl3() -> Result<Library, libloading::Error>{
    unsafe{
        let lib = libloading::Library::new(Path::new("src/sdl3/SDL3.dll"))?;
        Ok(lib)
    }
}
