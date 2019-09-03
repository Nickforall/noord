pub mod utils;
pub mod vertex;
pub mod colors;

pub use vertex::Vertex;

use crate::glium::Surface;
use utils::gl_to_pos;
use colors::Color;

static FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    uniform vec4 in_color;
    out vec4 color;

    void main() {
        color = in_color;
    }
"#;

static VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

fn opaque_shader(display: glium::Display) -> glium::Program {
  let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();
  program
}

pub fn draw_rect(display: glium::Display, target: &mut glium::Frame, x: i32, y: i32, w: i32, h: i32, color: Option<Color>) {
    let color = color.unwrap_or(Color::black());
    let program = opaque_shader(display.clone());

    let dimensions = display.get_framebuffer_dimensions();

    let vertex1 = Vertex { position: gl_to_pos((x, y), dimensions) };
    let vertex2 = Vertex { position: gl_to_pos((x, y + h), dimensions) };
    let vertex3 = Vertex { position: gl_to_pos((x + w, y + h), dimensions) };
    let vertex4 = Vertex { position: gl_to_pos((x + w, y), dimensions) };

    let shape = vec![vertex1, vertex2, vertex3, vertex4];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let uniforms = uniform! {
      in_color: color
    };

    target.draw(&vertex_buffer, &indices, &program, &uniforms,
        &Default::default()).unwrap();
}
