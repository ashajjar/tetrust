pub const FPS: u64 = 25;

pub trait GameObject {
    fn draw(&self);
}

pub enum Collision {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
