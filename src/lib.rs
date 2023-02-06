use std::ptr;
const NULL: *const u8 = ptr::null();

#[repr(u32)]
#[allow(dead_code)]
enum SDLInitFlags {
    Audio = 0x00000010,
    Video = 0x00000020,
}

#[repr(u32)]
#[allow(dead_code)]
enum SDLWindowFlags {
    WindowShown = 0x00000004,
}

#[derive(Default, Debug)]
#[repr(C)]
struct SdlEvent {
    event_type: u32,
    padding: [u8; 3],
}

#[repr(u32)]
#[allow(dead_code)]
enum SdlEventType {
    SdlQuit = 0x100,
}

#[link(name = "SDL2")]
#[allow(dead_code)]
extern "C" {
    fn SDL_Init(flags: u32) -> i32;
    fn SDL_CreateWindow(title: *const u8, x: i32, y: i32, w: i32, h: i32, flags: u32) -> *const u8;
    fn SDL_GetWindowSurface(sdl_window: *const u8) -> *const u8;
    fn SDL_FillRect(sdl_surface: *const u8, sdl_rect: *const u8, color: u32) -> i32;
    fn SDL_UpdateWindowSurface(sdl_window: *const u8);
    fn SDL_DestroyWindow(sdl_window: *const u8);
    fn SDL_PollEvent(event: &mut SdlEvent) -> i32;
    fn SDL_Quit();
    fn SDL_GetError() -> *const u8;
}

pub struct Window {
    sdl_window: *const u8,
    sdl_surface: *const u8,
}

pub fn create_window() -> Result<Window, &'static str> {
    let sdl_init = unsafe { SDL_Init(SDLInitFlags::Video as u32) };
    if sdl_init < 0 {
        return Err("Cannot initialize video subsystem");
    }

    let sdl_window = unsafe {
        SDL_CreateWindow(
            "Hello from rust".as_ptr(),
            0,
            0,
            600,
            400,
            SDLWindowFlags::WindowShown as u32,
        )
    };
    if sdl_window.is_null() {
        return Err("Cannot create window");
    }

    let sdl_surface = unsafe { SDL_GetWindowSurface(sdl_window) };
    if sdl_surface.is_null() {
        return Err("Cannot create drawing surface");
    }

    unsafe {
        SDL_FillRect(sdl_surface, NULL, 0xFFFFFFFF);
        SDL_UpdateWindowSurface(sdl_window);
    }

    Ok(Window {
        sdl_window,
        sdl_surface,
    })
}

pub enum WindowEvents {
    Quit,
}

pub fn poll_event() -> Option<WindowEvents> {
    let mut event = SdlEvent::default();
    unsafe {
        SDL_PollEvent(&mut event);
    }

    println!("{:?}", event);

    None
}

pub fn close_window(window: Window) {
    unsafe {
        SDL_DestroyWindow(window.sdl_window);
        SDL_Quit();
    }
}
