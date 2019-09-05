#[macro_use]
extern crate glium;
extern crate getopts;
extern crate html5ever;

mod gfx;
mod layout;

use glium::glutin;
use glium::Surface;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut opts = getopts::Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");
    opts.optopt("c", "css", "CSS stylesheet", "FILENAME");

    let matches = opts.parse(std::env::args().skip(1)).unwrap();
    let html_path = matches
        .opt_str("h")
        .unwrap_or(String::from("support/dev.html"));
    let css_path = matches
        .opt_str("c")
        .unwrap_or(String::from("support/style.css"));

    // 1. The **winit::EventsLoop** for handling events.
    let mut events_loop = glutin::EventsLoop::new();
    // 2. Parameters for building the Window.
    let wb = glutin::WindowBuilder::new();
    // 3. Parameters for building the OpenGL context.
    let cb = glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let rcdom = layout::html::parse_html_doc(html_path);
    let dom = layout::dom::serialize_rc_dom(rcdom);

    let mut css_buffer = String::new();
    File::open(css_path)
        .unwrap()
        .read_to_string(&mut css_buffer)
        .unwrap();

    let stylesheet = layout::css::parse(String::from(include_str!("../support/style.css")));
    let style_tree = layout::style::create_styletree(&dom, &stylesheet);

    let mut closed = false;
    let mut should_redraw = true;

    while !closed {
        if should_redraw {
            let mut target = display.draw();
            target.clear_color(1.0, 1.0, 1.0, 1.0);

            let display_dimensions = display.get_framebuffer_dimensions();
            let window_dimensions = layout::geometry::Dimensions::new(layout::geometry::Rect {
                x: 0.0,
                y: 0.0,
                width: display_dimensions.0 as f32,
                height: display_dimensions.1 as f32,
            });
            let geom_tree = layout::geometry::layout_geometry_tree(&style_tree, window_dimensions);

            // println!("{:#?}", geom_tree);

            let list = gfx::display_list::build_display_list(&geom_tree);
            gfx::render_list(&list, display.clone(), &mut target);

            should_redraw = false;

            target.finish().unwrap();
        }

        // listing the events produced by application and waiting to be received
        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                glutin::WindowEvent::Resized(_) => should_redraw = true,
                _ => (),
            },
            _ => (),
        });
    }
}
