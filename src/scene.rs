use std::sync::{Arc, Mutex};
use crate::game::GameObject;

pub struct Scene {
    objects: Vec<Arc<Mutex<dyn GameObject>>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: vec![],
        }
    }

    pub fn add(&mut self, object: Arc<Mutex<dyn GameObject>>) {
        self.objects.push(object);
    }

    pub fn refresh(&self) {
        self.reset();
        for object in &self.objects {
            let mut object = object.lock().unwrap();
            let collision = object.change_position();
            object.on_collision(&collision);
            object.draw();
        }
    }

    fn reset(&self) {
        println!("\u{001b}[2J\u{001b}[3J");
    }
}
