use serde::Deserialize;

use crate::{Camera, Triangle};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct RteRecord {
    record_kind: String,
    net_name: String,
    pin_number_sort: String,
    class: String,
    refdes: String,
    sym_type: String,
    sym_name: String,
    sym_x: String,
    sym_y: String,
    sym_rotate: String,
    sym_mirror: String,
    pin_name: String,
    pin_number: String,
    pin_x: String,
    pin_y: String,
    pin_edited: String,
    pad_stack_name: String,
    via_x: String,
    via_y: String,
    via_mirror: String,
    pin_rotation: String,
    test_point: String,
    net_probe_number: String,
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
