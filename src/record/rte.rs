use serde::Deserialize;

use crate::{Camera, Triangle};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct RteRecord {
    pub record_kind: String,
    pub net_name: String,
    pub pin_number_sort: String,
    pub class: String,
    pub refdes: String,
    pub sym_type: String,
    pub sym_name: String,
    pub sym_x: String,
    pub sym_y: String,
    pub sym_rotate: String,
    pub sym_mirror: String,
    pub pin_name: String,
    pub pin_number: String,
    pub pin_x: String,
    pub pin_y: String,
    pub pin_edited: String,
    pub pad_stack_name: String,
    pub via_x: String,
    pub via_y: String,
    pub via_mirror: String,
    pub pin_rotation: String,
    pub test_point: String,
    pub net_probe_number: String,
}

impl RteRecord {
    pub fn triangles(&self, triangles: &mut Vec<Triangle>, camera: &Camera) {
        let x: f64 = match self.pin_x.parse() {
            Ok(ok) => ok,
            Err(_) => match self.via_x.parse() {
                Ok(ok) => ok,
                Err(_) => return,
            },
        };

        let y: f64 = match self.pin_y.parse() {
            Ok(ok) => ok,
            Err(_) => match self.via_y.parse() {
                Ok(ok) => ok,
                Err(_) => return,
            },
        };

        // b a
        // c d

        let size = 0.1;
        let ax = x + size;
        let ay = y - size;
        let bx = x - size;
        let by = y - size;
        let cx = x - size;
        let cy = y + size;
        let dx = x + size;
        let dy = y + size;

        let color = (1.0, 1.0, 1.0);
        triangles.push(Triangle {
            a: camera.vertex(ax, ay, color),
            b: camera.vertex(bx, by, color),
            c: camera.vertex(cx, cy, color),
        });
        triangles.push(Triangle {
            a: camera.vertex(ax, ay, color),
            b: camera.vertex(cx, cy, color),
            c: camera.vertex(dx, dy, color),
        });
    }
}
