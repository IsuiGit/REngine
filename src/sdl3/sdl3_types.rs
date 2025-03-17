use std::ffi::c_void;
// SDL3 types register
// SDL3_Init type
pub type SDL_Init = unsafe extern "C" fn(u32) -> bool;

// SDL3_Quit type
pub type SDL_Quit = unsafe extern "C" fn() -> ();

// SDL3_GetError type
pub type SDL_GetError = unsafe extern "C" fn() -> *const i8;

// SDL3_CreateWindow
pub type SDL_CreateWindow = unsafe extern "C" fn(
    title: *const i8,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    flags: u32,
) -> *mut c_void; // Window pointer
