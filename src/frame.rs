use std::io::Write;
use crate::game::{Collision, GameObject};

pub struct Frame {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Frame {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            x,
            y,
        }
    }
}

impl GameObject for Frame {
    fn draw(&self) {
        print!("\u{001b}[H\u{001b}[?25l");

        let top_left_corner = '\u{2554}';
        let top_right_corner = '\u{2557}';

        let bottom_left_corner = '\u{255A}';
        let bottom_right_corner = '\u{255D}';

        let horizontal_bar = '\u{2550}';
        let vertical_bar = '\u{2551}';

        let frame_color_index = 19;
        print!("\u{001b}[38;5;{}m", frame_color_index);
        print!("\u{001b}[{};{}H{}", self.y, self.x, top_left_corner);
        for col in self.x + 1..self.x + self.width {
            print!("\u{001b}[{};{}H{}", self.y, col, horizontal_bar);
        }
        print!("\u{001b}[{};{}H{}", self.y, self.x + self.width, top_right_corner);

        for row in self.y + 1..self.y + self.height - 1 {
            print!("\u{001b}[{};{}H{}", row, self.x, vertical_bar);
            print!("\u{001b}[{};{}H{}", row, self.x + self.width, vertical_bar);
        }

        print!("\u{001b}[{};{}H{}", self.y + self.height - 1, self.x, bottom_left_corner);
        for col in self.x + 1..self.x + self.width {
            print!("\u{001b}[{};{}H{}", self.y + self.height - 1, col, horizontal_bar);
        }
        print!("\u{001b}[{};{}H{}", self.y + self.height - 1, self.x + self.width, bottom_right_corner);
        print!("\u{001b}[0m");
        std::io::stdout().flush().unwrap()
    }

    fn change_position(&mut self) -> Option<Collision> {
        None
    }

    fn get_speed(&self) -> (i32, i32) {
        (0, 0)
    }

    fn on_collision(&mut self, _:  &Option<Collision>) {
        ()
    }
}
