use std::ops::Sub;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::frame::Frame;
use crate::game::{FPS, HEIGHT, WIDTH};
use crate::scene::Scene;
use crate::tile::Tile;

mod game;
mod scene;
mod frame;
mod tile;

fn main() {
    let mut scene = Scene::new();
    let main_frame = Frame::new(1, 1, WIDTH, HEIGHT);
    let next_block_frame = Frame::new(WIDTH + 3, 1, 30, 15);
    let stats_frame = Frame::new(WIDTH + 3, 17, 30, 44);

    let x = 2;
    let y = 2;

    let object_height = 2;
    let object_width = 4;

    let square = Tile::new(object_width, object_height, x, y, WIDTH, HEIGHT);
    let main_frame_ref = Arc::new(Mutex::new(main_frame));
    let next_block_ref = Arc::new(Mutex::new(next_block_frame));
    let stats_frame_ref = Arc::new(Mutex::new(stats_frame));
    let square_ref = Arc::new(Mutex::new(square));

    scene.add(main_frame_ref);
    scene.add(next_block_ref);
    scene.add(stats_frame_ref);
    scene.add(square_ref);

    loop {
        let start_time = Instant::now();
        scene.refresh();

        let end_time = Instant::now();
        let raw_fps = Duration::from_micros(1000000 / FPS);
        let elapsed_microseconds = end_time.duration_since(start_time);

        thread::sleep(raw_fps.sub(elapsed_microseconds));
    }
}
