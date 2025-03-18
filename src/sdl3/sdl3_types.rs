use crate::sdl3::sdl3_structs::*;
use std::{ffi::c_void, os::raw::c_char};
// SDL3 types register
// SDL3_Init type
pub type SDL_Init = unsafe extern "C" fn(u32) -> bool;

// SDL3_Quit type
pub type SDL_Quit = unsafe extern "C" fn();

// SDL3_GetError type
pub type SDL_GetError = unsafe extern "C" fn() -> *const c_char;

// SDL3_CreateWindow type
pub type SDL_CreateWindow = unsafe extern "C" fn(*const i8, u32, u32, u32) -> *mut c_void; // Window pointer

// SDL3_DestroyWindow type
pub type SDL_DestroyWindow = unsafe extern "C" fn(*mut c_void);

// SDL3_Delay type
pub type SDL_Delay = unsafe extern "C" fn(u32);

// SDL3_PollEvent
pub type SDL_PollEvent = unsafe extern "C" fn(&mut SDL_Event) -> bool;
