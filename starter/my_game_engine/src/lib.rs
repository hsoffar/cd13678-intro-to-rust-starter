pub mod ffi;

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
        open_window("Rust Test Window - Game Loop");

        unsafe {
            while ffi::window_should_close() == 0 {
                ffi::clear_screen();
                ffi::update_game_window();
                std::thread::sleep(FRAME_DELAY);
            }
        }
    }

    #[test]
    #[ignore]
   
    fn test_sprite_rendering() {
        open_window("Rust Test Window - Sprite Rendering");

        unsafe {
            let sprite = ffi::create_sprite(0.0, 0.0, 50, 60, 255, 0, 0);

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
            let red = ffi::create_sprite(0.0, 0.0, 50, 60, 255, 0, 0);
            let green = ffi::create_sprite(0.0, 0.0, 50, 60, 0, 255, 0);

            let start = Instant::now();

            while ffi::window_should_close() == 0 {
                ffi::clear_screen();

                if start.elapsed() < Duration::from_secs(5) {
                    ffi::render_sprite(red);
                } else {
                    ffi::render_sprite(green);
                }

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
                    if !pressed[index] && ffi::get_key(window, keys[index]) == ffi::GLFW_PRESS {
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
            let sprite: *mut ffi::Sprite = ffi::create_sprite(0.0, 0.0, 50, 60, 0, 128, 255);

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
