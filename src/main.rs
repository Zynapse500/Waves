#[macro_use]
extern crate glium;

use glium::glutin;


mod vertex;


fn main() {
    println!("Hello, world!");

    let (mut events_loop, display) = create_window();


    let vertex_buffer = create_quad(&display);


    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent {event, ..} => {
                    match event {
                        glutin::WindowEvent::Closed => running = false,

                        _ => ()
                    }
                }

                _ => ()
            }
        });


        display.swap_buffers();
    }
}



fn create_window() -> (glutin::EventsLoop, glium::Display) {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Waves")
        .with_dimensions(512, 512);

    let context = glutin::ContextBuilder::new()
        .with_multisampling(8)
        .with_vsync(true);

    let mut display = glium::Display::new(window, context, &events_loop).unwrap();

    (events_loop, display)
}


fn create_quad(display: &glium::Display, x: f32, y: f32, w: f32, h: f32) -> glium::VertexBuffer<> {

}
