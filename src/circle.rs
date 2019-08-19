use orbclient::{Color, Renderer};
use std::cmp;

use crate::camera::Camera;
use crate::vec::Vec2;

pub struct Circle {
    pub p: Vec2,
    pub r: f64,
}

impl Circle {
    pub fn new(p: Vec2, r: f64) -> Self {
        Self { p, r }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut R, color: Color, camera: &Camera) {
        let w = renderer.width() as i32;
        let h = renderer.height() as i32;
        renderer.circle(
            ((self.p.x - camera.p.x) * camera.scale).round() as i32 + w / 2,
            ((self.p.y - camera.p.y) * camera.scale).round() as i32 + h / 2,
            -cmp::max(1, (self.r * camera.scale).round() as i32),
            color
        );
    }

    pub fn contains(&self, point: Vec2) -> Option<f64> {
        let d = point - self.p;
        let r = d.length();
        if r < self.r {
            Some(r)
        } else {
            None
        }
    }

    pub fn intersection(&self, other: &Self) -> Option<(Vec2, f64)> {
        let d = other.p - self.p;
        let r = d.length();

        if r < (self.r + other.r) {
            Some((
                d,
                r
            ))
        } else {
            None
        }
    }
}
