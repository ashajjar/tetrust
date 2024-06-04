use std::sync::Arc;
use crate::game::GameObject;

pub struct Scene {
    objects: Vec<Arc<dyn GameObject>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: vec![]
        }
    }

    pub fn add<T>(&mut self, object: Arc<T>) where T: GameObject + 'static {
        self.objects.push(object);
    }

    pub fn refresh(&self) {
        self.reset();
        for object in &self.objects {
            object.draw();
        }
    }

    fn reset(&self) {
        println!("\u{001b}[2J\u{001b}[3J");
    }
}
