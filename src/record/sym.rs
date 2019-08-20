use serde::Deserialize;
use std::f64::consts::PI;

use crate::{Camera, Triangle};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SymRecord {
    pub record_kind: String,
    pub sym_type: String,
    pub sym_name: String,
    pub refdes_sort: String,
    pub refdes: String,
    pub sym_x: String,
    pub sym_y: String,
    pub sym_rotate: String,
    pub sym_mirror: String,
    pub net_name_sort: String,
    pub net_name: String,
    pub class: String,
    pub subclass: String,
    pub record_tag: String,
    pub graphic_data_name: String,
    pub graphic_data_number: String,
    pub graphic_data_1: String,
    pub graphic_data_2: String,
    pub graphic_data_3: String,
    pub graphic_data_4: String,
    pub graphic_data_5: String,
    pub graphic_data_6: String,
    pub graphic_data_7: String,
    pub graphic_data_8: String,
    pub graphic_data_9: String,
    pub graphic_data_10: String,
    pub comp_device_type: String,
    pub comp_package: String,
    pub comp_part_number: String,
    pub comp_value: String,
    pub value: String,
}

impl SymRecord {
    pub fn triangles(&self, triangles: &mut Vec<Triangle>, camera: &Camera) {
        if self.class != "ETCH" {
            return;
        }

        if ! self.net_name.starts_with("M_") {
            return;
        }

        match self.graphic_data_name.as_str() {
            "LINE" => {
                let x1: f64 = match self.graphic_data_1.parse() {
                    Ok(ok) => ok,
                    Err(_) => return,
                };

                let y1: f64 = match self.graphic_data_2.parse() {
                    Ok(ok) => ok,
                    Err(_) => return,
                };

                let x2: f64 = match self.graphic_data_3.parse() {
                    Ok(ok) => ok,
                    Err(_) => return,
                };

                let y2: f64 = match self.graphic_data_4.parse() {
                    Ok(ok) => ok,
                    Err(_) => return,
                };

                let w: f64 = self.graphic_data_5.parse().unwrap_or(0.0);

                let thickness = w.max(0.00001) / 2.0;
                let angle = (y2 - y1).atan2(x2 - x1);
                let ax = x1 + thickness*(angle+PI/2.0).cos();
                let ay = y1 + thickness*(angle+PI/2.0).sin();
                let bx = x1 + thickness*(angle-PI/2.0).cos();
                let by = y1 + thickness*(angle-PI/2.0).sin();
                let cx = x2 + thickness*(angle-PI/2.0).cos();
                let cy = y2 + thickness*(angle-PI/2.0).sin();
                let dx = x2 + thickness*(angle+PI/2.0).cos();
                let dy = y2 + thickness*(angle+PI/2.0).sin();

                // b a
                // c d

                let color = (0.5, 0.5, 0.5);
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
            },
            _ => (),
        }
    }
}
