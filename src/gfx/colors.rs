pub struct Color {
  pub r: i16,
  pub g: i16,
  pub b: i16,
  pub a: i16,
}

impl Color {
  pub fn new(r: i16, g: i16, b: i16) -> Self {
    Self::new_alpha(r, g, b, 255)
  }

  pub fn new_alpha(r: i16, g: i16, b: i16, a: i16) -> Self {
    Color {
      r, g, b, a
    }
  }

  pub fn black() -> Self {
    Color::new_alpha(0, 0, 0, 255)
  }

  pub fn red() -> Self {
    Color::new_alpha(255, 0, 0, 255)
  }

  pub fn green() -> Self {
    Color::new_alpha(0, 255, 0, 255)
  }

  pub fn blue() -> Self {
    Color::new_alpha(0, 0, 255, 255)
  }
}

use glium::uniforms::{
  AsUniformValue,
  UniformValue
};

impl AsUniformValue for Color {
  fn as_uniform_value(&self) -> UniformValue {
    UniformValue::Vec4([self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0])
  }
} 