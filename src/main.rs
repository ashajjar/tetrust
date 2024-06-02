use std::io::Write;
use std::ops::Sub;
use std::thread;
use std::time::{Duration, Instant};

const FPS: u64 = 25;

fn main() {
    let width = 80;
    let height = 60;
    let screen = Screen::new(
        width,
        height,
    );

    let mut x = 0;
    let mut y = 0;

    let mut direction_x = 1;
    let mut direction_y = 1;
    let mut dx = 1;
    let mut dy = 1;

    let object_height = 2;
    let object_width = 4;
    loop {
        let start_time = Instant::now();
        screen.show();
        screen.draw_box_at(x, y, object_width, object_height);

        if x > width - object_width {
            direction_x = -1;
        }

        if x < 2 {
            direction_x = 1;
        }

        if y > height - object_height {
            direction_y = -1;
        }

        if y < object_height {
            direction_y = 1;
        }

        x += dx * direction_x;
        y += dy * direction_y;

        // println!("x={}, y={}", x, y);

        let end_time = Instant::now();
        let raw_fps = Duration::from_micros(1000000 / FPS);
        let elapsed_microseconds = end_time.duration_since(start_time);

        thread::sleep(raw_fps.sub(elapsed_microseconds));
    }
}


struct Screen {
    content: Vec<String>,
    width: i32,
    height: i32,
}

impl Screen {
    fn new(width: i32, height: i32) -> Self {
        let initial_row = String::from(' ');
        let initial_row = initial_row.repeat(width as usize);
        Self {
            content: vec![initial_row; height as usize],
            width,
            height,
        }
    }

    fn show(&self) {
        let out: String = self.content.join("\n\u{001b}[48;5;25m");
        print!("\u{001b}[2J\u{001b}[3J\u{001b}[H\u{001b}[?25l\u{001b}[48;5;25m");
        print!("{}", out);
        println!("\u{001b}[0m")
    }

    fn draw_box_at(&self, x: i32, y: i32, width: i32, height: i32) {
        assert!(x < self.width + 1, "X cannot be greater than screen width");
        assert!(y < self.height + 1, "Y cannot be greater than screen height");

        let color_index = 166;
        let object_line = String::from(' ').repeat(width as usize);

        for i in 0..height {
            print!("\u{001b}[{};{}H\u{001b}[48;5;{}m{}", y + i, x, color_index, object_line);
        }
        print!("\u{001b}[0m");
        std::io::stdout().flush().unwrap()
    }
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
