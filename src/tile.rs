use std::io::Write;
use crate::game::{GameObject, Collision};
use crate::game::Collision::{EAST, NORTH, SOUTH, WEST};

pub struct Tile {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    screen_width: i32,
    screen_height: i32,
    pub dy: i32,
    pub dx: i32,
    color_index: i32,
    content: String,
}

impl Tile {
    pub(crate) fn new(x: i32, y: i32, screen_width: i32, screen_height: i32, color_index: i32, content: String) -> Self {
        Self { width: content.lines().next().unwrap().len() as i32, height: content.lines().count() as i32, x, y, screen_width, screen_height, dy: 1, dx: 0, color_index, content }
    }
}

impl GameObject for Tile {
    fn draw(&self) {
        assert!(self.x < self.screen_width + 1, "X cannot be greater than screen width");
        assert!(self.y < self.screen_width + 1, "Y cannot be greater than screen height");

        for (i, line) in self.content.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == '0' { continue; }

                print!("\u{001b}[{};{}H\u{001b}[48;5;{}m{}", self.y + (i as i32), self.x + (j as i32), self.color_index, ' ');
            }
        }

        print!("\u{001b}[62;0H\u{001b}[48;5;{}m [x={},y={}]", self.color_index, self.x, self.y);

        print!("\u{001b}[0m");

        std::io::stdout().flush().unwrap()
    }

    /// Printing on the screen is based on index 1
    fn change_position(&mut self) -> Option<Collision> {
        let (dx, dy) = self.get_speed();
        if self.x + dx > self.screen_width - self.width {
            return Some(EAST);
        }

        // 2 = 1 counting for the frame border + 1 for the starting index of the line
        if self.x + dx < 2 {
            return Some(WEST);
        }

        if self.y + dy > self.screen_height - self.height {
            return Some(SOUTH);
        }

        if self.y + dy < 2 {
            return Some(NORTH);
        }

        self.x += dx;
        self.y += dy;
        None
    }

    fn get_speed(&self) -> (i32, i32) {
        (self.dx, self.dy)
    }

    fn on_collision(&mut self, collision: &Option<Collision>) {
        match collision {
            None => {}
            Some(collision) => {
                match collision {
                    SOUTH | NORTH => self.dy *= -1,
                    WEST | EAST => { self.dx *= -1 }
                }
            }
        }
    }
}
