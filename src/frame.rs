use std::io::Write;
use crate::game::{Collision, GameObject};
use crate::tile::{Tile, TILE_HEIGHT, TILE_WIDTH};

pub struct Frame {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) frame_color_index: i32,
    data: Vec<Vec<i32>>,
}

impl Frame {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut data = vec![];

        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(0);
            }
            data.push(row)
        }

        Self {
            width,
            height,
            x,
            y,
            frame_color_index: 19,
            data,
        }
    }

    pub fn freeze_tile(&mut self, tile: Tile) {
        self.frame_color_index = tile.color_index;
        for i in 0..TILE_HEIGHT {
            for j in 0..TILE_WIDTH {
                let i1 = i + tile.y as usize;
                let i2 = j + tile.x as usize;
                let i3 = tile.bitmap[i][j] as i32;
                print!("\u{001b}[{};{}H\u{001b}[38;5;34m {} {} {} ....\n", 63, 1, i1, i2, i3);
                if i1 >= self.height as usize || i2 >= self.width as usize {
                    break;
                }

                self.data[i1][i2] = i3;
            }
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

        print!("\u{001b}[38;5;{}m", self.frame_color_index);
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

        for col in 2..self.width {
            for row in 2..self.height {
                if self.data[row as usize][col as usize] == 0 { continue; }
                print!("\u{001b}[{};{}H{}", row+1, col, '#');
            }
        }
        print!("\u{001b}[{};{}H{}", self.y + self.height - 1, self.x + self.width, bottom_right_corner);
        print!("\u{001b}[0m");
        std::io::stdout().flush().unwrap()
    }

    fn change_position(&mut self) -> Option<Collision> {
        None
    }
}
