use std::collections::HashMap;
use std::ops::Sub;
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;

use crate::frame::Frame;
use crate::game::{FPS, GameObject, HEIGHT, WIDTH};
use crate::tile::Tile;

mod game;
mod scene;
mod frame;
mod tile;

fn main() {
    let mut current = generate_next();
    let mut next = generate_next();
    loop {
        let start_time = Instant::now();
        reset();
        draw_skeleton();

        let collision = current.change_position();
        current.on_collision(&collision);
        current.draw();

        if let Some(_) = collision {
            (next, current) = (generate_next(), next);
        }

        let end_time = Instant::now();
        let raw_fps = Duration::from_micros(1000000 / FPS);
        let elapsed_microseconds = end_time.duration_since(start_time);

        thread::sleep(raw_fps.sub(elapsed_microseconds));
    }
}

/**
    let content = String::from("\
111111
111111
111111");

    let content = String::from("\
111
111
111
111111
111111");


    let content = String::from("\
111
111
1111111
1111111
111
111");

 **/
fn generate_next() -> Tile {
    let x = 30;
    let y = 2;

    let square = String::from("\
11111111
11111111
11111111
11111111
");
    let l = String::from("\
1111
1111
1111
1111
11111111
11111111
");

    let reverse_l = String::from("\
00001111
00001111
00001111
00001111
11111111
11111111
");

    let stick = String::from("\
1111
1111
1111
1111
1111
1111
1111
1111
");

    let t = String::from("\
00001111
00001111
111111111111
111111111111
");

    let dog_left = String::from("\
11111111
11111111
000011111111
000011111111
");

    let dog_right = String::from("\
000011111111
000011111111
111111110000
111111110000
");

    let tiles = vec![square, stick, l, reverse_l, t, dog_left, dog_right];

    let color_index = rand::thread_rng().gen_range(15..232);
    let tile_index = rand::thread_rng().gen_range(0..7);
    Tile::new(x, y, WIDTH, HEIGHT, color_index, tiles[tile_index].clone())
}

fn reset() {
    println!("\u{001b}[2J\u{001b}[3J");
}

fn draw_skeleton() {
    let main_frame = Frame::new(1, 1, WIDTH, HEIGHT);
    let next_block_frame = Frame::new(WIDTH + 3, 1, 30, 15);
    let stats_frame = Frame::new(WIDTH + 3, 17, 30, 44);

    main_frame.draw();
    next_block_frame.draw();
    stats_frame.draw();
}
