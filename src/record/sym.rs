use orbclient::{Color, Renderer};
use serde::Deserialize;
use std::f64::consts::PI;

use crate::{Camera, Line, Triangle, Vertex};
use crate::vec::Vec2;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SymRecord {
    record_kind: String,
    sym_type: String,
    sym_name: String,
    refdes_sort: String,
    refdes: String,
    sym_x: String,
    sym_y: String,
    sym_rotate: String,
    sym_mirror: String,
    net_name_sort: String,
    net_name: String,
    class: String,
    subclass: String,
    record_tag: String,
    graphic_data_name: String,
    graphic_data_number: String,
    graphic_data_1: String,
    graphic_data_2: String,
    graphic_data_3: String,
    graphic_data_4: String,
    graphic_data_5: String,
    graphic_data_6: String,
    graphic_data_7: String,
    graphic_data_8: String,
    graphic_data_9: String,
    graphic_data_10: String,
    comp_device_type: String,
    comp_package: String,
    comp_part_number: String,
    comp_value: String,
    value: String,
}

impl SymRecord {
    pub fn draw<R: Renderer>(&self, renderer: &mut R, camera: &Camera) {
        if self.class != "ETCH" {
            return;
        }

        match self.graphic_data_name.as_str() {
            "LINE" => {
                let ax: f64 = match self.graphic_data_1.parse() {
                    Ok(ok) => ok,
                    Err(_) => return,
                };

                let ay: f64 = match self.graphic_data_2.parse() {
                    Ok(ok) => ok,
                    Err(_) => return,
                };

                let bx: f64 = match self.graphic_data_3.parse() {
                    Ok(ok) => ok,
                    Err(_) => return,
                };

                let by: f64 = match self.graphic_data_4.parse() {
                    Ok(ok) => ok,
                    Err(_) => return,
                };

                let t_opt: Option<f64> = match self.graphic_data_5.parse() {
                    Ok(ok) => Some(ok),
                    Err(_) => None,
                };

                Line::new(
                    Vec2::new(ax, ay),
                    Vec2::new(bx, by),
                    t_opt
                ).draw(renderer, Color::rgb(0x7F, 0x7F, 0x7F), camera);
            },
            _ => (),
        }
    }

    pub fn triangles(&self, triangles: &mut Vec<Triangle>, camera: &Camera) {
        if self.class != "ETCH" {
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

                let t: f64 = self.graphic_data_5.parse().unwrap_or(0.0);

                let thickness = t.max(0.00001);
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
