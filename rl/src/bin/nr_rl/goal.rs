use rand::Rng;

#[derive(Debug)]
pub struct Goal {
    x: f32,
    y: f32,
}

impl Goal {
    pub fn in_region(x_bounds: (f32, f32), y_bounds: (f32, f32)) -> Goal {
        let mut rng = rand::thread_rng();
        let x = x_bounds.0 + (x_bounds.1 - x_bounds.0) * rng.gen::<f32>();
        let y = y_bounds.0 + (y_bounds.1 - y_bounds.0) * rng.gen::<f32>();
        Goal { x: x, y: y }
    }

    pub fn coordinates(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
