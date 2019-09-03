#[macro_use]
extern crate glium;
extern crate html5ever;

mod gfx;
mod layout;

use glium::glutin;
use glium::Surface;

fn main() {

    layout::layout_pipeline();

    // 1. The **winit::EventsLoop** for handling events.
    let mut events_loop = glutin::EventsLoop::new();
    // 2. Parameters for building the Window.
    let wb = glutin::WindowBuilder::new();
    // 3. Parameters for building the OpenGL context.
    let cb = glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut closed = false;
    while !closed {    
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        gfx::draw_rect(display.clone(), &mut target, 10, 10, 200, 200, None);
        gfx::draw_rect(display.clone(), &mut target, 400, 400, 200, 200, Some(gfx::colors::Color::blue()));

        target.finish().unwrap();

        // listing the events produced by application and waiting to be received
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

