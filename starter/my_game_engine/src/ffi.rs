use std::ffi::{c_char, c_int, c_float};


pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

#[repr(C)]
pub struct GLFWwindow {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Sprite {
    pub width: c_int,
    pub height: c_int,
    pub color: [c_int; 3],
    pub x: c_float,
    pub y: c_float,
}

unsafe extern "C" {
 //void create_game_window(const char *title, int width, int height);




    pub unsafe fn create_game_window(
        title: *const c_char,
        width: c_int,
        height: c_int
    );

//Sprite* create_sprite(float x, float y, int width, int height, int r, int g, int b);

    pub unsafe fn create_sprite(
        x:      c_float,
        y:      c_float,
        width:  c_int,
        height: c_int,
        r:      c_int,
        g:      c_int,
        b:      c_int
    ) -> *mut Sprite;

    
    pub unsafe fn render_sprite(
        sprite: *mut Sprite
    );

    pub unsafe fn update_sprite_position(
        sprite: *mut Sprite,
        x: c_float,
        y: c_float,
    );

    pub unsafe fn update_game_window();

    pub unsafe fn clear_screen();

    pub unsafe fn window_should_close() -> c_int;

    pub unsafe fn get_key(
        window: *mut GLFWwindow,
        key: c_int,
    ) -> c_int;

    pub unsafe fn get_window() -> *mut GLFWwindow;


}