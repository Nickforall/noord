#[derive(Copy, Clone, PartialEq)]
pub struct Color {
  pub g: u8,
  pub b: u8,
  pub r: u8,
  pub a: u8,
}

impl Color {
  pub fn new_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
    Color { r, g, b, a }
  }

  pub fn black() -> Self {
    Color::new_alpha(0, 0, 0, 255)
  }

  pub fn transparent() -> Self {
    Color::new_alpha(0, 0, 0, 0)
  }
}

use glium::uniforms::{AsUniformValue, UniformValue};

impl AsUniformValue for Color {
  fn as_uniform_value(&self) -> UniformValue {
    UniformValue::Vec4([
      self.r as f32 / 255.0,
      self.g as f32 / 255.0,
      self.b as f32 / 255.0,
      self.a as f32 / 255.0,
    ])
  }
}

impl std::fmt::Debug for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "rgba({}, {}, {}, {:?})",
      self.r,
      self.g,
      self.b,
      (self.a / 255)
    )
  }
}
