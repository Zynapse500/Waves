#[macro_use]
extern crate glium;

use glium::glutin;
use glium::Surface;
use glium::index::{NoIndices, PrimitiveType};


use std::time::{Duration, Instant};

mod vertex;

use vertex::Vertex;

fn main() {
    println!("Hello, world!");

    let (mut events_loop, display) = create_window();


    let canvas = create_canvas(&display);
    let indices = NoIndices(PrimitiveType::TrianglesList);

    let program = create_program(&display);

    let (width, height) = {
        let (w, h) = display.gl_window().window().get_inner_size().unwrap();
        (w as f32, h as f32)
    };

    let start_time = Instant::now();

    let mut left_mouse_down = false;
    let mut right_mouse_down = false;
    let mut mouse_position = [0.0, 0.0];


    let mut centers: Vec<[f32; 2]> = vec![
        [width / 2.0, height / 2.0],
        [width / 4.0, height / 2.0],
    ];

    let mut wavelength = 100.0 as f32;
    let mut frequency = 0.25 as f32;

    let mut color = 0.5;

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::Closed => running = false,
                        glutin::WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(key) = input.virtual_keycode {
                                match input.state {
                                    glutin::ElementState::Pressed => {
                                        match key {
                                            glutin::VirtualKeyCode::Escape => running = false,

                                            glutin::VirtualKeyCode::Up => wavelength *= 1.25,
                                            glutin::VirtualKeyCode::Down => wavelength /= 1.25,

                                            glutin::VirtualKeyCode::Right => frequency *= 1.25,
                                            glutin::VirtualKeyCode::Left => frequency /= 1.25,

                                            _ => ()
                                        }
                                    }

                                    _ => ()
                                }
                            }
                        }

                        glutin::WindowEvent::MouseInput { button, state, .. } => {
                            match state {
                                glutin::ElementState::Pressed => {
                                    match button {
                                        glutin::MouseButton::Left => left_mouse_down = true,
                                        glutin::MouseButton::Right => right_mouse_down = true,

                                        _ => ()
                                    }
                                }

                                glutin::ElementState::Released => {
                                    match button {
                                        glutin::MouseButton::Left => left_mouse_down = false,
                                        glutin::MouseButton::Right => right_mouse_down = false,

                                        _ => ()
                                    }
                                }
                            }
                        }

                        glutin::WindowEvent::CursorMoved { position, .. } => {
                            mouse_position = [position.0 as f32, position.1 as f32];
                        }

                        _ => ()
                    }
                }

                _ => ()
            }
        });


        if left_mouse_down {
            centers[0] = mouse_position;
        }



        let mut frame = display.draw();

        frame.clear_color(0.2, 0.0, 0.0, 1.0);

        let left = 0.0 as f32;
        let right = width;
        let top = 0.0 as f32;
        let bottom = height;

        let time = as_secs(Instant::now() - start_time) as f32;


        /*

        let blend = glium::Blend {
            color: glium::BlendingFunction::Addition {
                source: glium::LinearBlendingFactor::ConstantColor,
                destination: glium::LinearBlendingFactor::OneMinusConstantColor,
            },
            alpha: glium::BlendingFunction::Max,
            constant_value: (color, color, color, color),
        };
*/
        let draw_parameters = glium::DrawParameters {
            ..Default::default()
        };

        if right_mouse_down {
            color = mouse_position[1] as f32 / height
        }

        let uniforms = uniform!(
            left: left,
            right: right,
            top: top,
            bottom: bottom,

            center0: centers[0],
            center1: centers[1],

            color: color,

            wavelength: wavelength,
            time: time,
            frequency: frequency
        );

        frame.draw(&canvas, &indices, &program, &uniforms, &draw_parameters).unwrap();


        frame.finish().unwrap();
        display.swap_buffers().unwrap();
    }
}


fn create_window() -> (glutin::EventsLoop, glium::Display) {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Waves")
        .with_fullscreen(Some(events_loop.get_primary_monitor()))
        .with_dimensions(512, 512);

    let context = glutin::ContextBuilder::new()
        .with_multisampling(8)
        .with_srgb(false)
        .with_vsync(false);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    (events_loop, display)
}


fn create_canvas(display: &glium::Display) -> glium::VertexBuffer<Vertex> {
    let data = &[
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [1.0, -1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [-1.0, 1.0] },
        Vertex { position: [-1.0, -1.0] },
    ];

    glium::VertexBuffer::new(display, data).unwrap()
}


fn create_program(display: &glium::Display) -> glium::Program {
    use std::str::from_utf8;
    let vertex = from_utf8(include_bytes!("shaders/shader.vert")).unwrap();
    let fragment = from_utf8(include_bytes!("shaders/shader.frag")).unwrap();

    glium::Program::from_source(display, vertex, fragment, None).unwrap()
}


fn as_secs(duration: Duration) -> f64 {
    duration.as_secs() as f64 + 1e-9 * duration.subsec_nanos() as f64
}
