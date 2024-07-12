use std::process::exit;

pub const FPS: u64 = 12;
pub const WIDTH: i32 = 80;
pub const HEIGHT: i32 = 60;

pub trait GameObject {
    fn draw(&self);

    fn change_position(&mut self) -> Option<Collision>;
}

pub enum Collision {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

pub fn end_game() {
    print!("\u{001b}[2J\u{001b}[H\u{001b}[?25h");
    exit(0)
}

pub fn reset() {
    println!("\u{001b}[2J\u{001b}[3J");
}
