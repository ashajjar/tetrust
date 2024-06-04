use std::sync::{Arc};
use crate::frame::Frame;

pub const FPS: u64 = 12;

pub trait GameObject {
    fn draw(&self);
}

pub trait FramedObject {
    fn get_frame(&self) -> Arc<Frame>;
}

pub trait MovingObject<T> where T: FramedObject {
    fn change_position(&mut self) -> Option<Collision>;

    fn get_speed(&self) -> (i32, i32);
}

pub enum Collision {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
