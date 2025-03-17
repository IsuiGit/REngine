// SDL_INIT CONSTANTS START-------------------------------------------------------------
// SDL_INIT_AUDIO implies SDL_INIT_EVENTS
pub const SDL_INIT_AUDIO: u32 = 0x00000010;
// SDL_INIT_VIDEO implies SDL_INIT_EVENTS, should be initialized on the main thread
pub const SDL_INIT_VIDEO: u32 = 0x00000020;
// SDL_INIT_JOYSTICK implies SDL_INIT_EVENTS, should be initialized on the same thread
// as SDL_INIT_VIDEO on Windows if you don't set SDL_HINT_JOYSTICK_THREAD
pub const SDL_INIT_JOYSTICK: u32 = 0x00000200;
pub const SDL_INIT_HAPTIC: u32 = 0x00001000;
// SDL_INIT_GAMEPAD implies SDL_INIT_JOYSTICK
pub const SDL_INIT_GAMEPAD: u32 = 0x00002000;
pub const SDL_INIT_EVENTS: u32 = 0x00004000;
// SDL_INIT_SENSOR implies SDL_INIT_EVENTS
pub const SDL_INIT_SENSOR: u32 = 0x00008000;
// SDL_INIT_CAMERA implies SDL_INIT_EVENTS
pub const SDL_INIT_CAMERA: u32 = 0x00010000;
// SDL_INIT CONSTANTS END---------------------------------------------------------------
