use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use tetrust::Getch;

use crate::command::Command;
use crate::frame::Frame;
use crate::game::{Collision, end_game, FPS, GameObject, HEIGHT, reset, WIDTH};
use crate::tile::Tile;

mod game;
mod frame;
mod tile;
mod command;

fn main() {
    let (sender, receiver) = channel::<Command>();

    let input = thread::spawn(move || {
        input_thread_handler(sender);
    });

    let game = thread::spawn(move || {
        let mut main_frame = Frame::new(1, 1, WIDTH, HEIGHT);
        let mut next_block_frame = Frame::new(WIDTH + 3, 1, 30, 15);
        let stats_frame = Frame::new(WIDTH + 3, 17, 30, 44);

        let main_frame_mutex = Mutex::new(&mut main_frame);
        let next_block_frame_mutex = Mutex::new(&mut next_block_frame);

        let (mut current, main_frame_mutex) = Tile::generate_next(&main_frame_mutex);
        let (mut next, mut next_block_frame_mutex) = Tile::generate_next(&next_block_frame_mutex);

        loop {
            let start_time = Instant::now();
            reset();

            next.x = 6;
            next.y = 3;

            match main_frame_mutex.try_lock() {
                Ok(frame) => { frame.draw() }
                Err(_) => {}
            };

            match next_block_frame_mutex.try_lock() {
                Ok(frame) => { frame.draw() }
                Err(_) => {}
            };
            stats_frame.draw();

            let collision = current.change_position();
            current.draw();
            next.draw();

            if let Some(collision) = collision {
                match collision {
                    Collision::SOUTH => {
                        next.container = freeze(current, &main_frame_mutex);
                        ((next, next_block_frame_mutex), current) = (Tile::generate_next(&next_block_frame_mutex), next);
                        current.x = 30;
                        current.y = 2;
                    }
                    _ => { continue }
                }
            }

            execute_command(&receiver, &mut current);
            let end_time = Instant::now();
            let raw_fps = Duration::from_micros(1000000 / FPS);
            let elapsed_microseconds = end_time.duration_since(start_time);

            let fps_adjustment = raw_fps
                .checked_sub(elapsed_microseconds)
                .or(Some(raw_fps))
                .unwrap();
            thread::sleep(fps_adjustment);
        }
    });

    input.join().expect("Failed to get input !");
    game.join().expect("Game panicked !");
}

fn freeze<'a>(tile: Tile, frame: &'a Mutex<&'a mut Frame>) -> &'a Mutex<&'a mut Frame> {
    match frame.try_lock() {
        Ok(mut container) => {
            container.freeze_tile(tile);
        }
        Err(_) => {}
    }
    frame
}

fn execute_command(receiver: &Receiver<Command>, current: &mut Tile) {
    match receiver.try_recv() {
        Ok(command) => {
            log(command.to_string().as_str(), 1);

            match command {
                Command::LEFT => {
                    current.x -= 3;
                }
                Command::RIGHT => {
                    current.x += 3;
                }
                Command::DOWN => {
                    current.y += 2;
                }
                Command::ROTATE => { current.bitmap; }
                Command::SMASH => {
                    current.dy = 10;
                }
                Command::NONE => {
                    current.dx = 0;
                }
                Command::EXIT => {
                    end_game();
                }
            }
        }
        Err(_) => {}
    };
}

fn input_thread_handler(sender: Sender<Command>) {
    loop {
        let char_reader = Getch::new();

        let mut byte_arr: [u8; 4] = [0, 0, 0, 0];
        let mut i = 0;
        loop {
            let c = char_reader.getch_raw().unwrap();
            if i == 0 && c != 27 {
                byte_arr[0] = c;
                break;
            }
            if c == 0 { break; }
            byte_arr[i] = c;
            i += 1;
            if i == 3 {
                break;
            }
        }

        let command = Command::from_bytes(byte_arr);
        sender.send(command).unwrap();
    }
}

fn log(message: &str, offset: i32) {
    let clear = String::from(' ').repeat(80);
    print!(
        "\u{001b}[{};{}H\u{001b}[48;5;16m {}",
        62 + offset,
        1,
        clear
    );
    print!(
        "\u{001b}[{};{}H\u{001b}[38;5;34m {}",
        62 + offset,
        1,
        message
    );
    println!("\u{001b}[0m");
}
