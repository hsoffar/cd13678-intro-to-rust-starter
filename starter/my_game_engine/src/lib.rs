pub mod ffi;
 use std::ffi::CString;

    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;
fn open_window(title: &str) {
    let title = CString::new(title).unwrap();
    unsafe {
        ffi::create_game_window(title.as_ptr(), WIDTH, HEIGHT);
    }
}



macro_rules! start_window_and_game_loop {
    ($title:expr) => {
        open_window($title);

        unsafe {
            while ffi::window_should_close() == 0 {
                ffi::clear_screen();
                ffi::update_game_window();
                std::thread::sleep(FRAME_DELAY);
            }
        }
    };
}


macro_rules! on_key_press {
    ($key:expr) => {

        unsafe {
        let window: *mut ffi::GLFWwindow = ffi::get_window();
        ffi::get_key(window, $key) == ffi::GLFW_PRESS 
        }
    };
}

macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $width:expr, $height:expr, $r:expr, $g:expr, $b:expr) => {{
        unsafe {
            ffi::create_sprite(
                $x,
                $y,
                $width,
                $height,
                $r,
                $g,
                $b,
            )
        }
    }};
}


macro_rules! change_sprite_color {
    ($sprite:expr, $r:expr, $g:expr, $b:expr) => {{
   unsafe {
            (*$sprite).color[0] = $r;
            (*$sprite).color[1] = $g;
            (*$sprite).color[2] = $b;
        }
    }};
}



#[cfg(test)]
mod tests {
    use super::ffi;
    use std::ffi::CString;
    use std::time::{Duration, Instant};

    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    const FRAME_DELAY: Duration = Duration::from_millis(16);



fn open_window(title: &str) {
    let title = CString::new(title).unwrap();
    unsafe {
        ffi::create_game_window(title.as_ptr(), WIDTH, HEIGHT);
    }
}



    #[test]
    #[ignore]
     
    fn test_simple_game_loop() {
       start_window_and_game_loop!("Rust Test Window - Game Loop");
    }

    #[test]
    #[ignore]
   
    fn test_sprite_rendering() {
        open_window("Rust Test Window - Sprite Rendering");

        unsafe {
            let sprite = spawn_sprite!(0.0, 0.0, 50, 60, 255, 0, 0);

            while ffi::window_should_close() == 0 {
                ffi::clear_screen();
                ffi::render_sprite(sprite);
                ffi::update_game_window();
                std::thread::sleep(FRAME_DELAY);
            }
        }
    }

    #[test]
    #[ignore]
    
    fn test_screen_clearing() {
        open_window("Rust Test Window - Screen Clearing");

        unsafe {
            let sprit_ = spawn_sprite!(0.0, 0.0, 50, 60, 255, 0, 0);

            let start = Instant::now();

            while ffi::window_should_close() == 0 {
                ffi::clear_screen();

                if start.elapsed() < Duration::from_secs(5) {
                    change_sprite_color!(sprit_,255,0,0);
                } else {
                    change_sprite_color!(sprit_,0,0,255);
                }

                ffi::render_sprite(sprit_);
                ffi::update_game_window();
                std::thread::sleep(FRAME_DELAY);
            }
        }
    }

    #[test]
    #[ignore]
    
    fn test_key_presses() {
        open_window("Rust Test Window - Key Presses");

        unsafe {
            let window: *mut ffi::GLFWwindow = ffi::get_window();

            let keys: [i32; 5] = [
                ffi::GLFW_KEY_SPACE,
                ffi::GLFW_KEY_UP,
                ffi::GLFW_KEY_DOWN,
                ffi::GLFW_KEY_LEFT,
                ffi::GLFW_KEY_RIGHT,
            ];





 
            let mut pressed: [bool; 5] = [false; 5];

            while ffi::window_should_close() == 0 && !pressed.iter().all(|state| *state) {
                for index in 0..keys.len() {
                    if !pressed[index] && on_key_press!(keys[index]) {
                        pressed[index] = true;
                    }
                }

          
                ffi::update_game_window();
                std::thread::sleep(FRAME_DELAY);
            }
        }
    }

    #[test]
    #[ignore]
    
    fn test_sprite_position_update() {
        open_window("Rust Test Window - Sprite Position");

        unsafe {
            let sprite: *mut ffi::Sprite = spawn_sprite!(0.0, 0.0, 50, 60, 0, 128, 255);

            let mut x: f32 = 0.0f32;

            while ffi::window_should_close() == 0 {
                x +=1.0;

                ffi::update_sprite_position(sprite, x, 270.0);

                ffi::clear_screen();
                ffi::render_sprite(sprite);
                ffi::update_game_window();
                std::thread::sleep(FRAME_DELAY);
            }
        }
    }
}
