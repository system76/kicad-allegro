use orbclient::{Color, Renderer};
use std::f64::consts::PI;

use crate::camera::Camera;
use crate::vec::Vec2;

pub struct Line {
    pub a: Vec2,
    pub b: Vec2,
    pub t_opt: Option<f64>,
}

impl Line {
    pub fn new(a: Vec2, b: Vec2, t_opt: Option<f64>) -> Self {
        Self { a, b, t_opt }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut R, color: Color, camera: &Camera) {
        let w = renderer.width() as i32;
        let h = renderer.height() as i32;

        renderer.line(
            ((self.a.x - camera.p.x) * camera.scale).round() as i32 + w / 2,
            ((self.a.y - camera.p.y) * camera.scale).round() as i32 + h / 2,
            ((self.b.x - camera.p.x) * camera.scale).round() as i32 + w / 2,
            ((self.b.y - camera.p.y) * camera.scale).round() as i32 + h / 2,
            color
        );

        // if let Some(thickness) = self.t_opt {
        //     let angle = (self.b.y - self.a.y).atan2(self.b.x - self.a.x);
        //
        //     let ax = self.a.x + thickness*(angle+PI/2.0).cos();
        //     let ay = self.a.y + thickness*(angle+PI/2.0).sin();
        //     let bx = self.a.x + thickness*(angle-PI/2.0).cos();
        //     let by = self.a.y + thickness*(angle-PI/2.0).sin();
        //     let cx = self.b.x + thickness*(angle-PI/2.0).cos();
        //     let cy = self.b.y + thickness*(angle-PI/2.0).sin();
        //     let dx = self.b.x + thickness*(angle+PI/2.0).cos();
        //     let dy = self.b.y + thickness*(angle+PI/2.0).sin();
        //
        //     renderer.line(
        //         ((ax - camera.p.x) * camera.scale).round() as i32 + w / 2,
        //         ((ay - camera.p.y) * camera.scale).round() as i32 + h / 2,
        //         ((bx - camera.p.x) * camera.scale).round() as i32 + w / 2,
        //         ((by - camera.p.y) * camera.scale).round() as i32 + h / 2,
        //         color
        //     );
        //
        //     renderer.line(
        //         ((bx - camera.p.x) * camera.scale).round() as i32 + w / 2,
        //         ((by - camera.p.y) * camera.scale).round() as i32 + h / 2,
        //         ((cx - camera.p.x) * camera.scale).round() as i32 + w / 2,
        //         ((cy - camera.p.y) * camera.scale).round() as i32 + h / 2,
        //         color
        //     );
        //
        //     renderer.line(
        //         ((cx - camera.p.x) * camera.scale).round() as i32 + w / 2,
        //         ((cy - camera.p.y) * camera.scale).round() as i32 + h / 2,
        //         ((dx - camera.p.x) * camera.scale).round() as i32 + w / 2,
        //         ((dy - camera.p.y) * camera.scale).round() as i32 + h / 2,
        //         color
        //     );
        //
        //     renderer.line(
        //         ((dx - camera.p.x) * camera.scale).round() as i32 + w / 2,
        //         ((dy - camera.p.y) * camera.scale).round() as i32 + h / 2,
        //         ((ax - camera.p.x) * camera.scale).round() as i32 + w / 2,
        //         ((ay - camera.p.y) * camera.scale).round() as i32 + h / 2,
        //         color
        //     );
        // }
    }
}
