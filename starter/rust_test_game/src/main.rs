use std::sync::mpsc;
use serde::Deserialize;
use std::thread;
use my_game_engine::*;

pub enum NetworkCommand {
    FetchSprite,
    Quit,
}

#[derive(Debug, Deserialize)]
struct SpriteData {
    x: f32,
    y: f32,
    width: i32,
    height: i32,
    r: i32,
    g: i32,
    b: i32,
}




fn get_sprite_data() -> Result<SpriteData, String> {
     let response_result = reqwest::blocking::get(
        "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler"
    ); 

    println!("{:?}", response_result);
    
    let response = match response_result {
        Ok(response) => response,
        Err(error) => return Err(error.to_string()),
    };

    println!("{:?}", response);

    let sprite_data;

    match response.json::<SpriteData>() {
        Ok(data) => sprite_data = data,
        Err(error) => return Err(error.to_string()),
    };


    println!("{:?}", sprite_data);

    Ok(sprite_data)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel::<SpriteData>();
    let (command_sender, command_receiver) = mpsc::channel::<NetworkCommand>();


    let thread1_th = thread::spawn(move || {
        thread1(command_receiver, sender);
    });
    let thread2_th = thread::spawn(move || {
        thread2(command_sender, receiver);
    });

    // thread2 owns the window; it sends Quit on close, which ends thread1.
    thread2_th.join().unwrap();
    thread1_th.join().unwrap();
  

    Ok(())
}




fn thread1(commands: mpsc::Receiver<NetworkCommand>, sender: mpsc::Sender<SpriteData>) {

    while let Ok(command) = commands.recv() {
        match command {
            NetworkCommand::FetchSprite => match get_sprite_data() {
                Ok(sprite_data) => {
                    let _ = sender.send(sprite_data);
                }
                Err(error) => eprintln!("{error}"),
            },
            NetworkCommand::Quit => break,
        }
    }
}

fn thread2(commands: mpsc::Sender<NetworkCommand>, receiver: mpsc::Receiver<SpriteData>) {
    start_window!("Test game");

    let mut sprites: Vec<*mut my_game_engine::ffi::Sprite> = Vec::new();
    let mut space_was_down = false;

    unsafe {
        while my_game_engine::ffi::window_should_close() == 0 {
            // edge detection: one request per press, not one per frame
            let space_is_down = on_key_press!(my_game_engine::ffi::GLFW_KEY_SPACE);
            if space_is_down && !space_was_down {
                let _ = commands.send(NetworkCommand::FetchSprite);
            }
            space_was_down = space_is_down;

            // non-blocking drain: pick up new sprites without stalling the frame
            while let Ok(d) = receiver.try_recv() {
                sprites.push(spawn_sprite!(d.x, d.y, d.width, d.height, d.r, d.g, d.b));
            }

            my_game_engine::ffi::clear_screen();
            for s in &sprites {
                my_game_engine::ffi::render_sprite(*s);
            }
            my_game_engine::ffi::update_game_window();
            std::thread::sleep(my_game_engine::FRAME_DELAY);
        }
    }

    // window closed: stop the worker so main can join it
    let _ = commands.send(NetworkCommand::Quit);
}