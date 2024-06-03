use std::io::Write;
use std::ops::Sub;
use std::thread;
use std::time::{Duration, Instant};

use crate::Touch::{EAST, NORTH, SOUTH, WEST};

const FPS: u64 = 25;

fn main() {
    let width = 80;
    let height = 60;
    let main_frame = Frame::new(1, 1, width, height);
    let next_block_frame = Frame::new(width + 3, 1, 30, 15);
    let stats_frame = Frame::new(width + 3, 17, 30, 44);
    let mut small_block = Frame::new(4, 4, 7, 3);

    let x = 2;
    let y = 2;

    let mut direction_x = 1;
    let mut direction_y = 1;
    let dx = 1;
    let dy = 1;

    let object_height = 2;
    let object_width = 4;

    let mut square = Square::new(object_width, object_height, x, y, &main_frame);

    loop {
        let start_time = Instant::now();

        reset_frame();
        for i in (2..width - 1).step_by(8) {
            for j in (2..height).step_by(3) {
                small_block.x = i;
                small_block.y = j;
                small_block.draw();
                println!("\u{001b}[{};{}H{}", j, i, i);
            }
        }

        main_frame.draw();
        next_block_frame.draw();
        stats_frame.draw();
        square.show();

        match square.change_position(dx * direction_x, dy * direction_y) {
            None => {}
            Some(touch) => {
                match touch {
                    SOUTH | NORTH => { direction_y = direction_y * -1 }
                    WEST | EAST => { direction_x = direction_x * -1 }
                };
            }
        }

        let end_time = Instant::now();
        let raw_fps = Duration::from_micros(1000000 / FPS);
        let elapsed_microseconds = end_time.duration_since(start_time);

        thread::sleep(raw_fps.sub(elapsed_microseconds));
    }
}

///Resets the screen, erasing the frame to prepare for the next frame
fn reset_frame() {
    println!("\u{001b}[2J\u{001b}[3J");
}


struct Frame {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Frame {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            x,
            y,
        }
    }

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

    // fn draw_box_at(&self, x: i32, y: i32, width: i32, height: i32) {
    //     assert!(x < self.width + 1, "X cannot be greater than screen width");
    //     assert!(y < self.height + 1, "Y cannot be greater than screen height");
    //
    //     let color_index = 166;
    //     let object_line = String::from(' ').repeat(width as usize);
    //
    //     for i in 0..height {
    //         print!("\u{001b}[{};{}H\u{001b}[48;5;{}m{}", y + i, x, color_index, object_line);
    //     }
    //     print!("\u{001b}[0m");
    //     std::io::stdout().flush().unwrap()
    // }
    // fn draw_circle_at(&self, x: i32, y: i32, radius: i32) {
    //     assert!(x < self.width + 1, "X cannot be greater than screen width");
    //     assert!(y < self.height + 1, "Y cannot be greater than screen height");
    //
    //     let color_index = 76;
    //
    //     for angle in 0..360 {
    //         let angle = angle as f32;
    //         let xi = radius as f32 * f32::cos(angle * PI / 180f32);
    //         let yi = radius as f32 * f32::sin(angle * PI / 180f32) / 2f32;
    //         print!("\u{001b}[{};{}H\u{001b}[48;5;{}m{}", y + yi as i32, x + xi as i32, color_index, 'x');
    //     }
    //
    //     print!("\u{001b}[0m");
    //     std::io::stdout().flush().unwrap()
    // }
}

struct Square<'a> {
    width: i32,
    height: i32,
    pub x: i32,
    pub y: i32,
    frame: &'a Frame,
}

impl<'a> Square<'a> {
    fn new(width: i32, height: i32, x: i32, y: i32, screen: &'a Frame) -> Self {
        Self { width, height, x, y, frame: screen }
    }
}

// impl<'a> Display for Square<'a> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         assert!(self.x < self.screen.width + 1, "X cannot be greater than screen width");
//         assert!(self.y < self.screen.height + 1, "Y cannot be greater than screen height");
//
//         let color_index = 166;
//         let object_line = String::from(' ').repeat(self.width as usize);
//
//         for i in 0..self.height {
//             let object = format!("\u{001b}[{};{}H\u{001b}[48;5;{}m{}", self.y + i, self.x, color_index, object_line);
//             f.write_str(&object)?;
//         }
//         f.write_str("\u{001b}[0m")?;
//         std::io::stdout().flush().unwrap();
//         Ok(())
//     }
// }

enum Touch {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl<'a> Square<'a> {
    /// Printing on the screen is based on index 1
    fn change_position(&mut self, dx: i32, dy: i32) -> Option<Touch> {
        if self.x + dx > self.frame.width - self.width {
            return Some(EAST);
        }

        // 2 = 1 counting for the frame border + 1 for the starting index of the line
        if self.x + dx < 2 {
            return Some(WEST);
        }

        if self.y + dy > self.frame.height - self.height {
            return Some(SOUTH);
        }

        if self.y + dy < 2 {
            return Some(NORTH);
        }

        self.x += dx;
        self.y += dy;
        None
    }
    fn show(&self) {
        assert!(self.x < self.frame.width + 1, "X cannot be greater than screen width");
        assert!(self.y < self.frame.height + 1, "Y cannot be greater than screen height");

        let color_index = 166;
        let object_line = String::from(' ').repeat(self.width as usize);

        for i in 0..self.height {
            print!("\u{001b}[{};{}H\u{001b}[48;5;{}m{}", self.y + i, self.x, color_index, object_line);
        }
        print!("\u{001b}[62;0H\u{001b}[48;5;{}m [x={},y={}]", color_index, self.x, self.y);

        print!("\u{001b}[0m");

        std::io::stdout().flush().unwrap()
    }
}
