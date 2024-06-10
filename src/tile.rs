use std::io::Write;
use rand::Rng;
use crate::frame::Frame;
use crate::game::{GameObject, Collision};
use crate::game::Collision::{EAST, NORTH, SOUTH, WEST};

const MSB: u16 = 0b1000_0000_0000_0000;

const SQUARE: [u16; 8] = [
    0b0000_0000_0000_0000,
    0b0000_0000_0000_0000,
    0b0000_1111_1111_0000,
    0b0000_1111_1111_0000,
    0b0000_1111_1111_0000,
    0b0000_1111_1111_0000,
    0b0000_0000_0000_0000,
    0b0000_0000_0000_0000,
];

const STICK: [u16; 8] = [
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
];

const THE_T: [u16; 8] = [
    0b0000_0000_0000_0000,
    0b0000_0000_0000_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_1111_1111_1111,
    0b0000_1111_1111_1111,
    0b0000_0000_0000_0000,
    0b0000_0000_0000_0000,
];

const LEFT_L: [u16; 8] = [
    0b0000_0000_0000_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_1111_1111_0000,
    0b0000_1111_1111_0000,
    0b0000_0000_0000_0000,
];

const RIGHT_L: [u16; 8] = [
    0b0000_0000_0000_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_0000,
    0b0000_0000_1111_1111,
    0b0000_0000_1111_1111,
    0b0000_0000_0000_0000,
];

const THE_S: [u16; 8] = [
    0b0000_0000_0000_0000,
    0b0000_0000_0000_0000,
    0b0000_0000_1111_1111,
    0b0000_0000_1111_1111,
    0b0000_1111_1111_0000,
    0b0000_1111_1111_0000,
    0b0000_0000_0000_0000,
    0b0000_0000_0000_0000,
];

const THE_Z: [u16; 8] = [
    0b0000_0000_0000_0000,
    0b0000_0000_0000_0000,
    0b0000_1111_1111_0000,
    0b0000_1111_1111_0000,
    0b0000_0000_1111_1111,
    0b0000_0000_1111_1111,
    0b0000_0000_0000_0000,
    0b0000_0000_0000_0000,
];

const TILES: [[u16; 8]; 7] = [THE_S, THE_T, RIGHT_L, STICK, LEFT_L, SQUARE, THE_Z];

pub struct Tile<'a> {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub dy: i32,
    pub dx: i32,
    bitmap: [u16; 8],
    color_index: i32,
    pub(crate) container: &'a Frame,
}

impl<'a> Tile<'a> {
    pub(crate) fn generate_next(container: &'a Frame) -> Tile<'a> {
        let color_index = rand::thread_rng().gen_range(15..232);
        let tile_index: usize = rand::thread_rng().gen_range(0..7);
        Self {
            x: 30,
            y: 2,
            container,
            color_index,
            dy: 1,
            dx: 0,
            bitmap: TILES[tile_index],
        }
    }
}

impl GameObject for Tile<'_> {
    fn draw(&self) {
        assert!(self.x < self.container.width + 1, "X cannot be greater than screen width");
        assert!(self.y < self.container.height + 1, "Y cannot be greater than screen height");

        for (i, line) in self.bitmap.iter().enumerate() {
            let mut line = *line;
            for j in 0..15 {
                let msb = MSB & line;
                if msb != 0 {
                    print!(
                        "\u{001b}[{};{}H\u{001b}[48;5;{}m{}",
                        self.container.y + self.y + (i as i32),
                        self.container.x + self.x + j,
                        self.color_index,
                        ' '
                    );
                }

                line = line << 1;
            }
        }

        print!("\u{001b}[62;0H\u{001b}[48;5;{}m [x={},y={}]", self.color_index, self.container.y + self.x, self.container.y + self.y);
        print!("\u{001b}[0m");

        std::io::stdout().flush().unwrap()
    }

    /// Printing on the screen is based on index 1
    fn change_position(&mut self) -> Option<Collision> {
        if self.x + self.dx > self.container.width - 16 {
            return Some(EAST);
        }

        // 2 = 1 counting for the frame border + 1 for the starting index of the line
        if self.x + self.dx < 2 {
            return Some(WEST);
        }

        if self.y + self.dy > self.container.height - 8 {
            return Some(SOUTH);
        }

        if self.y + self.dy < 2 {
            return Some(NORTH);
        }

        self.x += self.dx;
        self.y += self.dy;
        None
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
