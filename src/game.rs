pub const FPS: u64 = 12;
pub const WIDTH: i32 = 80;
pub const HEIGHT: i32 = 60;
pub trait GameObject {
    fn draw(&self);

    fn change_position(&mut self) -> Option<Collision>;

    fn on_collision(&mut self, collision: &Option<Collision>);
}

pub enum Collision {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
