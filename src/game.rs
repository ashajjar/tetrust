pub const FPS: u64 = 12;

pub trait GameObject {
    fn draw(&self);

    fn change_position(&mut self) -> Option<Collision>;

    fn get_speed(&self) -> (i32, i32);

    fn on_collision(&mut self, collision: Option<Collision>);
}

pub enum Collision {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
