use std::ops::Sub;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod game;
mod scene;
mod frame;
mod square;

use crate::game::{FPS};
use crate::game::Collision::{EAST, NORTH, SOUTH, WEST};
use crate::scene::Scene;
use crate::frame::Frame;
use crate::square::Square;

fn main() {
    let width = 80;
    let height = 60;
    let mut scene = Scene::new();
    let main_frame = Frame::new(1, 1, width, height);
    let next_block_frame = Frame::new(width + 3, 1, 30, 15);
    let stats_frame = Frame::new(width + 3, 17, 30, 44);

    let x = 2;
    let y = 2;

    let mut direction_x = 1;
    let mut direction_y = 1;
    let dx = 1;
    let dy = 1;

    let object_height = 2;
    let object_width = 4;

    let main_frame_ref = Arc::new(main_frame);
    let square = Square::new(object_width, object_height, x, y, Arc::clone(&main_frame_ref));

    let next_block_ref = Arc::new(next_block_frame);
    let stats_frame_ref = Arc::new(stats_frame);
    let square_ref = Arc::new(Mutex::new(square));

    scene.add(Arc::clone(&main_frame_ref));
    scene.add(next_block_ref);
    scene.add(stats_frame_ref);
    scene.add(Arc::clone(&square_ref));

    loop {
        let start_time = Instant::now();

        scene.refresh();

        match square_ref.lock().expect("Cannot find a square").change_position(dx * direction_x, dy * direction_y) {
            None => {}
            Some(touch) => {
                match touch {
                    SOUTH | NORTH => { direction_y = direction_y * -1 }
                    WEST | EAST => { direction_x = direction_x * -1 }
                };
            }
        }

        let end_time = Instant::now();
        let raw_fps = Duration::from_micros(1000000 / FPS);
        let elapsed_microseconds = end_time.duration_since(start_time);

        thread::sleep(raw_fps.sub(elapsed_microseconds));
    }
}
