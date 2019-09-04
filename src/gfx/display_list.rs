use super::colors::Color;
use crate::layout::css::Value;
use crate::layout::geometry::*;

pub type DisplayList = Vec<DisplayListCommand>;

#[derive(Debug, Copy, Clone)]
pub enum DisplayListCommand {
  Rect(Color, Rect),
}

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
  let mut list = Vec::new();
  render_layout_box(&mut list, layout_root);
  return list;
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
  render_background(list, layout_box);

  for child in &layout_box.children {
    render_layout_box(list, child);
  }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
  if let Some(node) = layout_box.get_style_node_option() {
    let color = match node.lookup(
      "background-color",
      "background",
      &Value::ColorValue(Color::transparent()),
    ) {
      Value::ColorValue(color) => color,
      _ => Color::transparent(),
    };

    // TODO: fix transparency issues?
    if color.a < 255 {
      return;
    }

    list.push(DisplayListCommand::Rect(
      color,
      layout_box.dimensions.padding_box(),
    ));
  }
}
