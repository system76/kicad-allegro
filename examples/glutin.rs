mod support;

use glutin::GlContext;

use self::support::{Vertex, Triangle};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("A fantastic window!");
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    println!("Pixel format of the window's GL context: {:?}", gl_window.get_pixel_format());

    let gl = support::load(&gl_window.context());

    let mut triangles = vec![
        Triangle {
            a: Vertex {
                x: -0.25,
                y: -0.25,
                c: (1.0, 0.0, 0.0),
            },
            b: Vertex {
                x: 0.0,
                y: 0.25,
                c: (0.0, 1.0, 0.0),
            },
            c: Vertex {
                x: 0.25,
                y: -0.25,
                c: (0.0, 0.0, 1.0),
            },
        },
        Triangle {
            a: Vertex {
                x: -0.75,
                y: -0.75,
                c: (1.0, 0.0, 0.0),
            },
            b: Vertex {
                x: -0.5,
                y: -0.25,
                c: (0.0, 1.0, 0.0),
            },
            c: Vertex {
                x: -0.25,
                y: -0.75,
                c: (0.0, 0.0, 1.0),
            },
        },
    ];

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            //println!("{:?}", event);
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == glutin::ElementState::Pressed {
                            if input.virtual_keycode == Some(glutin::VirtualKeyCode::W) {
                            }
                        }
                    },
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => (),
                },
                _ => ()
            }
        });

        {
            let t = &mut triangles[0];
            t.a.x += 0.001;
            t.b.x += 0.001;
            t.c.x += 0.001;
        }

        gl.draw_frame([1.0, 0.5, 0.7, 1.0], &triangles);
        let _ = gl_window.swap_buffers();
    }
}
