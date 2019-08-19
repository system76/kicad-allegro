use glutin::GlContext;
use orbclient::{
    Color, EventOption, Renderer, Window, WindowFlag,
    ButtonEvent, MouseEvent,
};
use std::io;
use std::collections::HashSet;

use self::camera::Camera;
mod camera;

use self::circle::Circle;
mod circle;

use self::line::Line;
mod line;

use self::record::{RteRecord, SymRecord};
mod record;

pub use self::support::{Triangle, Vertex};
mod support;

use self::vec::Vec2;
mod vec;

fn main() -> io::Result<()> {
    let mut rte_records = Vec::new();
    {
        let path =
            //"/home/jeremy/Dropbox (System76)/TGL/AEP/BRD/609182_TGL_UP3_LPDDR4x_AEP_BRD_Rev0p9.brd-rte.txt"
            "/home/jeremy/Dropbox (System76)/TGL/RVP/BRD/609003_TGL_U_DDR4_SODIMM_RVP_BRD_REV0p8.brd-rte.txt"
        ;
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'!')
            .flexible(true)
            .from_path(path)?;

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
        let path =
            //"/home/jeremy/Dropbox (System76)/TGL/AEP/BRD/609182_TGL_UP3_LPDDR4x_AEP_BRD_Rev0p9.brd-rte.txt"
            "/home/jeremy/Dropbox (System76)/TGL/RVP/BRD/609003_TGL_U_DDR4_SODIMM_RVP_BRD_REV0p8.brd-sym.txt"
        ;
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'!')
            .flexible(true)
            .from_path(path)?;

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

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("A fantastic window!");
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
                        if input.state == glutin::ElementState::Pressed {
                            if input.virtual_keycode == Some(glutin::VirtualKeyCode::R) {
                                camera.p = Vec2::new(0.0, 0.0);
                                camera.scale = 1.0 / 128.0;
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
                        println!("Scroll {} at {}, {}", dy, mouse_x, mouse_y);
                        let target = camera.translate(mouse_x, mouse_y);
                        camera.scale *= 1.0 + dy / 144.0;
                        camera.target(mouse_x, mouse_y, target);
                    },
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
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

        if let Some(dragging) = dragging_opt {
            camera.target(mouse_x, mouse_y, dragging);
        }

        triangles.clear();
        for sym_record in sym_records.iter() {
            sym_record.triangles(&mut triangles, &camera);
        }

        gl.draw_frame([0.0, 0.0, 0.0, 1.0], &triangles);
        let _ = gl_window.swap_buffers();
    }


    /*
    let mut window = Window::new_flags(
        -1, -1,
        1024, 768,
        "kicad-allegro",
        &[WindowFlag::Async, WindowFlag::Resizable]
    ).ok_or(io::Error::new(
        io::ErrorKind::Other,
        "failed to open orbclient window"
    ))?;

    let mut camera = Camera {
        p: Vec2::new(0.0, 0.0),
        scale: 1.0,
    };
    let mut dragging_opt = None;
    let mut last_button = ButtonEvent {
        left: false,
        middle: false,
        right: false
    };
    let mut last_mouse = MouseEvent {
        x: 0,
        y: 0,
    };
    let mut keys = HashSet::with_capacity(256);
    'running: loop {
        window.set(Color::rgb(0x00, 0x00, 0x00));
        for sym_record in sym_records.iter() {
            sym_record.draw(&mut window, &camera);
        }
        // for rte_record in rte_records.iter() {
        //     rte_record.draw(&mut window, &camera);
        // }
        window.sync();

        let mut saw_event = true;
        while saw_event {
            saw_event = false;
            for event in window.events() {
                saw_event = true;
                match event.to_option() {
                    EventOption::Button(button) => {
                        dragging_opt = if button.left {
                            Some(camera.translate(&window, last_mouse.x, last_mouse.y))
                        } else {
                            None
                        };

                        last_button = button;
                    },
                    EventOption::Key(key) => {
                        if key.pressed {
                            keys.insert(key.scancode);

                            if key.scancode == orbclient::K_R {
                                camera.p = Vec2::new(0.0, 0.0);
                                camera.scale = 1.0;
                            }
                        } else {
                            keys.remove(&key.scancode);
                        }
                    },
                    EventOption::Mouse(mouse) => {
                        last_mouse = mouse;
                    },
                    EventOption::Scroll(scroll) => {
                        let target = camera.translate(&window, last_mouse.x, last_mouse.y);
                        camera.scale *= 1.0 + (scroll.y as f64 * 64.0 / 144.0);
                        camera.target(&window, last_mouse.x, last_mouse.y, target);
                    },
                    EventOption::Quit(_quit) => break 'running,
                    unknown => println!("{:?}", unknown),
                }
            }
        }

        if keys.contains(&orbclient::K_W) {
            camera.p.y -= 10.0;
        }

        if keys.contains(&orbclient::K_S) {
            camera.p.y += 10.0;
        }

        if keys.contains(&orbclient::K_A) {
            camera.p.x -= 10.0;
        }

        if keys.contains(&orbclient::K_D) {
            camera.p.x += 10.0;
        }

        if keys.contains(&orbclient::K_Q) {
            camera.scale *= 1.0 - 1.0 / 144.0;
        }

        if keys.contains(&orbclient::K_E) {
            camera.scale *= 1.0 + 1.0 / 144.0;
        }

        if let Some(dragging) = dragging_opt {
            camera.target(&window, last_mouse.x, last_mouse.y, dragging);
        }
    }
    */

    Ok(())
}
