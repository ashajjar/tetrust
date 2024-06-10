use std::ops::Sub;
use std::thread;
use std::time::{Duration, Instant};

use crate::frame::Frame;
use crate::game::{FPS, GameObject, HEIGHT, WIDTH};
use crate::tile::Tile;

mod game;
mod frame;
mod tile;

fn main() {
    let main_frame = Frame::new(1, 1, WIDTH, HEIGHT);
    let next_block_frame = Frame::new(WIDTH + 3, 1, 30, 15);
    let stats_frame = Frame::new(WIDTH + 3, 17, 30, 44);
    let mut current = Tile::generate_next(&main_frame);
    let mut next = Tile::generate_next(&next_block_frame);

    loop {
        let start_time = Instant::now();
        reset();

        next.x = 6;
        next.y = 3;

        main_frame.draw();
        next_block_frame.draw();
        stats_frame.draw();

        let collision = current.change_position();
        current.on_collision(&collision);
        current.draw();
        next.draw();

        if let Some(_) = collision {
            next.container = &main_frame;
            (next, current) = (Tile::generate_next(&next_block_frame), next);
            current.x = 30;
            current.y = 2;
        }

        let end_time = Instant::now();
        let raw_fps = Duration::from_micros(1000000 / FPS);
        let elapsed_microseconds = end_time.duration_since(start_time);

        thread::sleep(raw_fps.sub(elapsed_microseconds));
    }
}

fn reset() {
    println!("\u{001b}[2J\u{001b}[3J");
}
