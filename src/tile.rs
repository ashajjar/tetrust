use std::io::Write;
use std::sync::Mutex;

use rand::Rng;

use crate::frame::Frame;
use crate::game::{Collision, GameObject};
use crate::game::Collision::{EAST, NORTH, SOUTH, WEST};

pub const TILE_WIDTH: usize = 12;
pub const TILE_HEIGHT: usize = 8;
pub const TILES_COUNT: usize = 7;

const SQUARE: [[u8; TILE_WIDTH]; TILE_HEIGHT] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const STICK: [[u8; TILE_WIDTH]; TILE_HEIGHT] = [
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
];

const THE_T: [[u8; TILE_WIDTH]; TILE_HEIGHT] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const LEFT_L: [[u8; TILE_WIDTH]; TILE_HEIGHT] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const RIGHT_L: [[u8; TILE_WIDTH]; TILE_HEIGHT] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const THE_S: [[u8; TILE_WIDTH]; TILE_HEIGHT] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const THE_Z: [[u8; TILE_WIDTH]; TILE_HEIGHT] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const TILES: [[[u8; TILE_WIDTH]; TILE_HEIGHT]; TILES_COUNT] = [THE_S, THE_T, RIGHT_L, STICK, LEFT_L, SQUARE, THE_Z];

pub struct Tile<'a> {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub dy: i32,
    pub dx: i32,
    pub(crate) bitmap: [[u8; TILE_WIDTH]; TILE_HEIGHT],
    pub(crate) color_index: i32,
    pub(crate) container: &'a Mutex<&'a mut Frame>,
}

impl<'a> Tile<'a> {
    pub fn calc_empty_space_bottom(&self) -> i32 {
        for (i, line) in self.bitmap.iter().enumerate().rev() {
            if line.iter().sum::<u8>() != 0 {
                return (TILE_HEIGHT - 1 - i) as i32;
            }
        }
        (TILE_HEIGHT - 1) as i32
    }

    pub fn calc_empty_space_left(&self) -> i32 {
        for column in 0..TILE_WIDTH {
            for line in self.bitmap {
                if line[column] != 0 { return column as i32; }
            }
        }
        0
    }

    pub fn calc_empty_space_right(&self) -> i32 {
        for column in (0..=11).rev() {
            for line in self.bitmap {
                if line[column] != 0 { return 11 - column as i32; }
            }
        }
        0
    }

    fn log(&self, message: &str, offset: i32) {
        let container = self.container.try_lock();
        let container = match container {
            Ok(frame) => { frame }
            Err(_) => { return }
        };
        let clear = String::from(' ').repeat(container.width as usize);
        print!(
            "\u{001b}[{};{}H\u{001b}[48;5;16m {}",
            container.height + offset,
            container.x,
            clear
        );
        print!(
            "\u{001b}[{};{}H\u{001b}[38;5;34m {}",
            container.height + offset,
            container.x,
            message
        );
        print!("\u{001b}[0m");
    }
}

impl<'a> Tile<'a> {
    pub(crate) fn generate_next(container: &'a Mutex<&'a mut Frame>) -> (Tile<'a>, &'a Mutex<&'a mut Frame>) {
        let color_index = rand::thread_rng().gen_range(15..232);
        let tile_index: usize = rand::thread_rng().gen_range(0..TILES_COUNT);
        (Self {
            x: 30,
            y: 2,
            container,
            color_index,
            dy: 1,
            dx: 0,
            bitmap: TILES[tile_index],
        }, container)
    }
}

impl GameObject for Tile<'_> {
    fn draw(&self) {
        let container = self.container.try_lock();
        let container = match container {
            Ok(frame) => { frame }
            Err(_) => { return }
        };
        assert!(self.x < container.width + 1, "X cannot be greater than screen width");
        assert!(self.y < container.height + 1, "Y cannot be greater than screen height");

        for (i, line) in self.bitmap.iter().enumerate() {
            for (j, bit) in line.iter().enumerate() {
                if *bit == 0 { continue; }
                print!(
                    "\u{001b}[{};{}H\u{001b}[48;5;{}m{}",
                    container.y + self.y + (i as i32),
                    container.x + self.x + (j as i32),
                    self.color_index,
                    ' '
                );
            }
        }

        let msg = format!(
            "[x={},y={}]",
            container.y + self.x,
            container.y + self.y
        );
        drop(container); // todo try to remove this line ;)
        self.log(&msg[0..], 1);
        print!("\u{001b}[0m");
        std::io::stdout().flush().unwrap();
    }

    /// Printing on the screen is based on index 1
    fn change_position(&mut self) -> Option<Collision> {
        let empty_space_bottom = self.calc_empty_space_bottom();
        let empty_space_left = self.calc_empty_space_left();
        let empty_space_right = self.calc_empty_space_right();

        let msg = format!(
            "[space_left={},space_right={},bottom={}]",
            empty_space_left,
            empty_space_right,
            empty_space_bottom,
        );
        self.log(&msg[0..], 2);

        let container = self.container.try_lock();
        let container = match container {
            Ok(frame) => { frame }
            Err(_) => { return None }
        };


        if self.y + self.dy > container.height - (9 - empty_space_bottom) {
            self.container.clear_poison();
            return Some(SOUTH);
        }

        if self.y + self.dy < 2 {
            self.container.clear_poison();
            return Some(NORTH);
        }

        if self.x + (TILE_WIDTH as i32) - empty_space_right > container.width {
            self.x = container.width + empty_space_right - (TILE_WIDTH as i32);
            self.change_position();
            self.container.clear_poison();
            return Some(EAST);
        }

        // 2 = 1 counting for the frame border + 1 for the starting index of the line
        if self.x < (0 - empty_space_left) {
            self.x = -empty_space_left + 1;
            self.change_position();
            self.container.clear_poison();
            return Some(WEST);
        }

        self.x += self.dx;
        self.y += self.dy;
        self.container.clear_poison();
        None
    }
}
