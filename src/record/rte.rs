use serde::Deserialize;

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
    // pub fn draw<R: Renderer>(&self, renderer: &mut R, camera: &Camera) {
    //     let x: f64 = match self.pin_x.parse() {
    //         Ok(ok) => ok,
    //         Err(_) => match self.via_x.parse() {
    //             Ok(ok) => ok,
    //             Err(_) => return,
    //         },
    //     };
    //
    //     let y: f64 = match self.pin_y.parse() {
    //         Ok(ok) => ok,
    //         Err(_) => match self.via_y.parse() {
    //             Ok(ok) => ok,
    //             Err(_) => return,
    //         },
    //     };
    //
    //     Circle::new(
    //         Vec2::new(x, y),
    //         0.1
    //     ).draw(renderer, Color::rgb(0x7F, 0x7F, 0x7F), camera);
    //
    //
    //     let w = renderer.width() as i32;
    //     let h = renderer.height() as i32;
    //     let px = ((x - camera.p.x) * camera.scale).round() as i32 + w / 2;
    //     let py = ((y - camera.p.y) * camera.scale).round() as i32 + h / 2;
    //
    //     let mut px = px - (self.pin_number.len() as i32) * 4;
    //     let py = py - 8;
    //     for c in self.pin_number.chars() {
    //         renderer.char(px, py, c, Color::rgb(0xFF, 0xFF, 0xFF));
    //         px += 8;
    //     }
    // }
}
