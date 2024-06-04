use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::frame::Frame;
use crate::game::{GameObject, Collision, FramedObject, MovingObject};
use crate::game::Collision::{EAST, NORTH, SOUTH, WEST};

pub struct Square {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    frame: Arc<Frame>,
    pub dy: i32,
    pub dx: i32,
}

impl Square {
    pub(crate) fn new(width: i32, height: i32, x: i32, y: i32, frame: Arc<Frame>) -> Self {
        Self { width, height, x, y, frame, dy: 1, dx: 1 }
    }
}

impl FramedObject for Square {
    fn get_frame(&self) -> Arc<Frame> {
        Arc::clone(&self.frame)
    }
}

impl MovingObject<Square> for Square {
    /// Printing on the screen is based on index 1
    fn change_position(&mut self) -> Option<Collision> {
        let (dx, dy) = self.get_speed();
        let frame = self.get_frame();
        if self.x + dx > frame.width - self.width {
            return Some(EAST);
        }

        // 2 = 1 counting for the frame border + 1 for the starting index of the line
        if self.x + dx < 2 {
            return Some(WEST);
        }

        if self.y + dy > frame.height - self.height {
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
}

impl GameObject for Mutex<Square> {
    fn draw(&self) {
        let mutable = self.lock().unwrap();
        assert!(mutable.x < mutable.frame.width + 1, "X cannot be greater than screen width");
        assert!(mutable.y < mutable.frame.height + 1, "Y cannot be greater than screen height");

        let color_index = 166;
        let object_line = String::from(' ').repeat(mutable.width as usize);

        for i in 0..mutable.height {
            print!("\u{001b}[{};{}H\u{001b}[48;5;{}m{}", mutable.y + i, mutable.x, color_index, object_line);
        }
        print!("\u{001b}[62;0H\u{001b}[48;5;{}m [x={},y={}]", color_index, mutable.x, mutable.y);

        print!("\u{001b}[0m");

        std::io::stdout().flush().unwrap()
    }
}
