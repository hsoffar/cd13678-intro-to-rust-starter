use std::sync::mpsc;
use serde::Deserialize;
use std::thread;
use my_game_engine::*;
use std::time::Duration;

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

    let mut sprite_data;

    match response.json::<SpriteData>() {
        Ok(data) => sprite_data = data,
        Err(error) => return Err(error.to_string()),
    };


    println!("{:?}", sprite_data);

    Ok(sprite_data)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel::<SpriteData>();


    let thread1_th = thread::spawn(move || {
        thread1(sender);
    });
    let thread2_th = thread::spawn(move || {
        thread2(receiver);
    });

    thread1_th.join().unwrap();
    thread2_th.join().unwrap();
  

    Ok(())
}




fn thread1(sender: mpsc::Sender<SpriteData>) {
    
    while(true) {  
        let sprite_data: SpriteData = get_sprite_data().unwrap();

        sender.send(sprite_data).unwrap();
        std::thread::sleep(Duration::from_secs(5) );
    }
}

fn thread2(receiver: mpsc::Receiver<SpriteData>) {
    start_window!("Test game");

    let mut sprites: Vec<*mut my_game_engine::ffi::Sprite> = Vec::new();

    unsafe {
        while my_game_engine::ffi::window_should_close() == 0 {
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
}