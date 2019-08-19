use crate::Vertex;
use crate::vec::Vec2;

pub struct Camera {
    pub p: Vec2,
    pub scale: f64,
}

impl Camera {
    pub fn translate(&self, x: f64, y: f64) -> Vec2 {
        Vec2::new(
            self.p.x + x / self.scale,
            self.p.y + y / self.scale
        )
    }

    pub fn target(&mut self, x: f64, y: f64, target: Vec2) {
        self.p.x = target.x - x / self.scale;
        self.p.y = target.y - y / self.scale;
    }

    pub fn vertex(&self, x: f64, y: f64, c: (f32, f32, f32)) -> Vertex {
        Vertex {
            x: ((x - self.p.x) * self.scale) as f32,
            y: ((y - self.p.y) * self.scale) as f32,
            c,
        }
    }
}
