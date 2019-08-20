use glutin::GlContext;
use std::{fs, io};
use std::collections::HashSet;
use std::io::Write;

use self::camera::Camera;
mod camera;

use self::record::{RteRecord, SymRecord};
mod record;

pub use self::support::{Triangle, Vertex};
mod support;

use self::vec::Vec2;
mod vec;

fn main() -> io::Result<()> {
    let board_path =
        //"/home/jeremy/Dropbox (System76)/TGL/AEP/BRD/609182_TGL_UP3_LPDDR4x_AEP_BRD_Rev0p9.brd"
        "/home/jeremy/Dropbox (System76)/TGL/RVP/BRD/609003_TGL_U_DDR4_SODIMM_RVP_BRD_REV0p8.brd"
    ;

    let mut rte_records = Vec::new();
    {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'!')
            .flexible(true)
            .from_path(format!("{}-rte.txt", board_path))?;

        for record_res in reader.records() {
            let record = record_res?;
            if record.get(0) == Some("S") {
                let rte_record: RteRecord = record.deserialize(None)?;
                //println!("{:?}", rte_record);
                rte_records.push(rte_record);
            } else {
                println!("Unimplemented: {:?}", record);
            }
        }
    }

    let mut sym_records = Vec::new();
    {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'!')
            .flexible(true)
            .from_path(format!("{}-sym.txt", board_path))?;

        for record_res in reader.records() {
            let record = //TODO record_res?;
            match record_res {
                Ok(ok) => ok,
                Err(_) => continue,
            };
            if record.get(0) == Some("S") {
                let sym_record: SymRecord = record.deserialize(None)?;
                //println!("{:?}", sym_record);
                sym_records.push(sym_record);
            } else {
                println!("Unimplemented: {:?}", record);
            }
        }
    }

    {
        let mut layers = Vec::new();
        let mut nets = Vec::new();
        let mut tracks = Vec::new();

        for rte in rte_records.iter() {
            if rte.class != "VIA CLASS" {
                continue;
            }

            if ! rte.net_name.starts_with("M_") {
                continue;
            }

            let x: f64 = match rte.via_x.parse() {
                Ok(ok) => ok,
                Err(_) => continue,
            };

            let y: f64 = match rte.via_y.parse() {
                Ok(ok) => ok,
                Err(_) => continue,
            };

            let i = match nets.iter().position(|x| x == &rte.net_name) {
                Some(some) => some,
                None => nets.len(),
            };
            if i == nets.len() {
                nets.push(rte.net_name.clone());
            }

            tracks.push(format!(
                "  (via (at {} {}) (size 0.4572) (drill 0.2032) (layers F.Cu B.Cu) (net {}))\n",
                x, -y,
                i
            ));
        }

        for sym in sym_records.iter() {
            if sym.class != "ETCH" {
                continue;
            }

            if ! sym.net_name.starts_with("M_") {
                continue;
            }

            match sym.graphic_data_name.as_str() {
                "LINE" => {
                    let x1: f64 = match sym.graphic_data_1.parse() {
                        Ok(ok) => ok,
                        Err(_) => continue,
                    };

                    let y1: f64 = match sym.graphic_data_2.parse() {
                        Ok(ok) => ok,
                        Err(_) => continue,
                    };

                    let x2: f64 = match sym.graphic_data_3.parse() {
                        Ok(ok) => ok,
                        Err(_) => continue,
                    };

                    let y2: f64 = match sym.graphic_data_4.parse() {
                        Ok(ok) => ok,
                        Err(_) => continue,
                    };

                    let t: f64 = sym.graphic_data_5.parse().unwrap_or(0.0);

                    let layer = match sym.subclass.as_str() {
                        "TOP" => format!("F.Cu"),
                        "BOTTOM" => format!("B.Cu"),
                        other => {
                            let layer = format!("{}.Cu", other);
                            if layers.iter().position(|x| x == &layer).is_none() {
                                layers.push(layer.clone());
                            }
                            layer
                        },
                    };

                    let i = match nets.iter().position(|x| x == &sym.net_name) {
                        Some(some) => some,
                        None => nets.len(),
                    };
                    if i == nets.len() {
                        nets.push(sym.net_name.clone());
                    }

                    tracks.push(format!(
                        //TODO: layer and net
                        "  (segment (start {} {}) (end {} {}) (width {}) (layer {}) (net {}))\n",
                        x1, -y1,
                        x2, -y2,
                        t,
                        layer,
                        i
                    ));
                },
                _ => {
                    println!("TODO: {:?}", sym);
                }
            }
        }

        let mut file = fs::File::create("export.kicad_pcb")?;
        file.write_all(format!(
r#"(kicad_pcb (version 20171130) (host pcbnew 5.1.4-e60b266~84~ubuntu19.04.1)

  (general
    (thickness 1.6)
    (drawings 0)
    (tracks {})
    (zones 0)
    (modules 0)
    (nets {})
  )

  (page A4)
  (layers
    (0 F.Cu signal)
"#,
            tracks.len(),
            nets.len(),
        ).as_bytes())?;

        layers.sort_by(|a, b| natord::compare(&a, &b));
        for (i, layer) in layers.iter().enumerate() {
            file.write_all(format!(
                "    ({} {} signal)",
                i + 1,
                layer
            ).as_bytes())?;
        }

        file.write_all(
r#"    (31 B.Cu signal)
    (32 B.Adhes user)
    (33 F.Adhes user)
    (34 B.Paste user)
    (35 F.Paste user)
    (36 B.SilkS user)
    (37 F.SilkS user)
    (38 B.Mask user)
    (39 F.Mask user)
    (40 Dwgs.User user)
    (41 Cmts.User user)
    (42 Eco1.User user)
    (43 Eco2.User user)
    (44 Edge.Cuts user)
    (45 Margin user)
    (46 B.CrtYd user)
    (47 F.CrtYd user)
    (48 B.Fab user)
    (49 F.Fab user)
  )

  (setup
    (last_trace_width 0.25)
    (trace_clearance 0.2)
    (zone_clearance 0.508)
    (zone_45_only no)
    (trace_min 0.2)
    (via_size 0.8)
    (via_drill 0.4)
    (via_min_size 0.4)
    (via_min_drill 0.3)
    (uvia_size 0.3)
    (uvia_drill 0.1)
    (uvias_allowed no)
    (uvia_min_size 0.2)
    (uvia_min_drill 0.1)
    (edge_width 0.05)
    (segment_width 0.2)
    (pcb_text_width 0.3)
    (pcb_text_size 1.5 1.5)
    (mod_edge_width 0.12)
    (mod_text_size 1 1)
    (mod_text_width 0.15)
    (pad_size 1.524 1.524)
    (pad_drill 0.762)
    (pad_to_mask_clearance 0.051)
    (solder_mask_min_width 0.25)
    (aux_axis_origin 0 0)
    (visible_elements FFFFFF7F)
    (pcbplotparams
      (layerselection 0x010fc_ffffffff)
      (usegerberextensions false)
      (usegerberattributes false)
      (usegerberadvancedattributes false)
      (creategerberjobfile false)
      (excludeedgelayer true)
      (linewidth 0.100000)
      (plotframeref false)
      (viasonmask false)
      (mode 1)
      (useauxorigin false)
      (hpglpennumber 1)
      (hpglpenspeed 20)
      (hpglpendiameter 15.000000)
      (psnegative false)
      (psa4output false)
      (plotreference true)
      (plotvalue true)
      (plotinvisibletext false)
      (padsonsilk false)
      (subtractmaskfromsilk false)
      (outputformat 1)
      (mirror false)
      (drillshape 1)
      (scaleselection 1)
      (outputdirectory ""))
  )

"#
        .as_bytes())?;

        for (i, net) in nets.iter().enumerate() {
            file.write_all(format!(
                "  (net {} \"{}\")\n",
                i,
                net,
            ).as_bytes())?;
        }

        file.write_all(
r#"
  (net_class Default "This is the default net class."
    (clearance 0.2)
    (trace_width 0.25)
    (via_dia 0.8)
    (via_drill 0.4)
    (uvia_dia 0.3)
    (uvia_drill 0.1)
  )

"#
        .as_bytes())?;

        for tracks in tracks.iter() {
            file.write_all(tracks.as_bytes())?;
        }
        file.write_all(b"\n)\n")?;
    }

    return Ok(());

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("KiCad Allegro");
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    println!("Pixel format of the window's GL context: {:?}", gl_window.get_pixel_format());

    let gl = support::load(&gl_window.context());

    let mut camera = Camera {
        p: Vec2::new(0.0, 0.0),
        scale: 1.0/128.0,
    };

    let mut w = 0.0;
    let mut h = 0.0;
    let mut keys = HashSet::new();
    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;
    let mut dragging_opt = None;
    let mut running = true;
    let mut triangles = Vec::new();
    while running {
        events_loop.poll_events(|event| {
            //println!("{:?}", event);
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CursorMoved { position, .. } => {
                        mouse_x = position.x / (w / 2.0) - 1.0;
                        mouse_y = - (position.y / (h / 2.0) - 1.0);
                    },
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(virtual_keycode) = input.virtual_keycode {
                            if input.state == glutin::ElementState::Pressed {
                                keys.insert(virtual_keycode);

                                if virtual_keycode == glutin::VirtualKeyCode::R {
                                    camera.p = Vec2::new(0.0, 0.0);
                                    camera.scale = 1.0 / 128.0;
                                }
                            } else {
                                keys.remove(&virtual_keycode);
                            }
                        }
                    },
                    glutin::WindowEvent::MouseInput { state, button, .. } => {
                        dragging_opt = if state == glutin::ElementState::Pressed &&  button == glutin::MouseButton::Left {
                            Some(camera.translate(mouse_x, mouse_y))
                        } else {
                            None
                        };
                    },
                    glutin::WindowEvent::MouseWheel { delta, .. } => {
                        let dy = match delta {
                            glutin::MouseScrollDelta::LineDelta(_x, y) => (y as f64) * 64.0,
                            glutin::MouseScrollDelta::PixelDelta(position) => position.y,
                        };
                        let target = camera.translate(mouse_x, mouse_y);
                        camera.scale *= 1.0 + dy / 144.0;
                        camera.target(mouse_x, mouse_y, target);
                    },
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        println!("Resized to {}, {}", logical_size.width, logical_size.height);
                        w = logical_size.width;
                        h = logical_size.height;
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => (),
                },
                _ => ()
            }
        });

        if keys.contains(&glutin::VirtualKeyCode::W) {
            camera.p.y += 10.0;
        }

        if keys.contains(&glutin::VirtualKeyCode::S) {
            camera.p.y -= 10.0;
        }

        if keys.contains(&glutin::VirtualKeyCode::A) {
            camera.p.x -= 10.0;
        }

        if keys.contains(&glutin::VirtualKeyCode::D) {
            camera.p.x += 10.0;
        }

        if keys.contains(&glutin::VirtualKeyCode::Q) {
            camera.scale *= 1.0 - 10.0 / 144.0;
        }

        if keys.contains(&glutin::VirtualKeyCode::E) {
            camera.scale *= 1.0 + 10.0 / 144.0;
        }

        if let Some(dragging) = dragging_opt {
            camera.target(mouse_x, mouse_y, dragging);
        }

        triangles.clear();
        for rte_record in rte_records.iter() {
            rte_record.triangles(&mut triangles, &camera);
        }
        for sym_record in sym_records.iter() {
            sym_record.triangles(&mut triangles, &camera);
        }

        gl.draw_frame([0.0, 0.0, 0.0, 1.0], &triangles);
        let _ = gl_window.swap_buffers();
    }

    Ok(())
}
