//! The Geometry Layout tree is one of the last steps in the layout pipelines

use crate::layout::css::{Unit, Value};
use crate::layout::style::DisplayStyle;
use crate::layout::style::StyledNode;

#[derive(Default, Debug, Copy, Clone)]
pub struct Dimensions {
  // Position of the content area relative to the document origin:
  content: Rect,

  // Surrounding edges:
  padding: EdgeSizes,
  border: EdgeSizes,
  margin: EdgeSizes,
}

impl Dimensions {
  pub fn new(rect: Rect) -> Self {
    Self {
      content: rect,
      padding: Default::default(),
      border: Default::default(),
      margin: Default::default(),
    }
  }

  // The area covered by the content area plus its padding.
  pub fn padding_box(self) -> Rect {
    self.content.expanded_by(self.padding)
  }
  // The area covered by the content area plus padding and borders.
  pub fn border_box(self) -> Rect {
    self.padding_box().expanded_by(self.border)
  }
  // The area covered by the content area plus padding, borders, and margin.
  pub fn margin_box(self) -> Rect {
    self.border_box().expanded_by(self.margin)
  }
}

#[derive(Default, Copy, Clone)]
pub struct Rect {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

impl Rect {
  fn expanded_by(self, edge: EdgeSizes) -> Rect {
    Rect {
      x: self.x - edge.left,
      y: self.y - edge.top,
      width: self.width + edge.left + edge.right,
      height: self.height + edge.top + edge.bottom,
    }
  }
}

impl std::fmt::Debug for Rect {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Rect {{ x: {}, y: {} }} {} x {}",
      self.x, self.y, self.width, self.height
    )
  }
}

struct SimpleDimensions {
  pub width: f32,
  pub height: f32,
}

impl SimpleDimensions {
  pub fn new(width: f32, height: f32) -> Self {
    SimpleDimensions { width, height }
  }

  pub fn from_dimension(rect: Rect) -> Self {
    SimpleDimensions {
      width: rect.width,
      height: rect.height,
    }
  }
}

#[derive(Default, Copy, Clone)]
pub struct EdgeSizes {
  left: f32,
  right: f32,
  top: f32,
  bottom: f32,
}

impl std::fmt::Debug for EdgeSizes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "EdgeSizes {{ left: {}, right: {}, top: {}, bottom: {} }} ",
      self.left, self.right, self.top, self.bottom
    )
  }
}

#[derive(Debug, Clone)]
pub enum BoxType<'a> {
  BlockNode(&'a StyledNode<'a>),
  InlineNode(&'a StyledNode<'a>),
  AnonymousBlock,
}

#[derive(Clone, Debug)]
pub struct LayoutBox<'a> {
  pub dimensions: Dimensions,
  box_type: BoxType<'a>,
  pub children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
  pub fn new(box_type: BoxType<'a>) -> Self {
    Self {
      dimensions: Default::default(),
      box_type,
      children: Vec::new(),
    }
  }

  pub fn get_style_node(&self) -> &'a StyledNode<'a> {
    match self.get_style_node_option() {
      Some(node) => node,
      None => panic!("This layout box (likely anonymous) has no style node"),
    }
  }

  pub fn get_style_node_option(&self) -> Option<&'a StyledNode<'a>> {
    match &self.box_type {
      BoxType::BlockNode(node) => Some(node),
      BoxType::InlineNode(node) => Some(node),
      BoxType::AnonymousBlock => None,
    }
  }

  fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
    match self.box_type {
      BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
      BoxType::BlockNode(_) => {
        // If we've just generated an anonymous block box, keep using it.
        // Otherwise, create a new one.
        match self.children.last() {
          Some(&LayoutBox {
            box_type: BoxType::AnonymousBlock,
            ..
          }) => {}
          _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
        }
        self.children.last_mut().unwrap()
      }
    }
  }

  // Lay out a box and its descendants.
  fn layout(&mut self, containing_block: Dimensions, original_containing_block: &SimpleDimensions) {
    match self.box_type {
      BoxType::BlockNode(_) => self.layout_block(containing_block, original_containing_block),
      BoxType::InlineNode(_) => {}  // TODO
      BoxType::AnonymousBlock => {} // TODO
    }
  }

  fn layout_block(
    &mut self,
    containing_block: Dimensions,
    original_containing_block: &SimpleDimensions,
  ) {
    // Child width can depend on parent width, so we need to calculate
    // this box's width before laying out its children.
    self.calculate_block_width(containing_block, original_containing_block);

    // Determine where the box is located within its container.
    self.calculate_block_position(containing_block, original_containing_block);

    // Recursively lay out the children of this box.
    self.layout_block_children(original_containing_block);

    // Parent height can depend on child height, so `calculate_height`
    // must be called *after* the children are laid out.
    self.calculate_block_height(original_containing_block);
  }

  /// Calculate the width of a block-level non-replaced element in normal flow.
  ///
  /// http://www.w3.org/TR/CSS2/visudet.html#blockwidth
  ///
  /// Sets the horizontal margin/padding/border dimensions, and the `width`.
  fn calculate_block_width(
    &mut self,
    containing_block: Dimensions,
    original_containing_block: &SimpleDimensions,
  ) {
    use Unit::Px;
    use Value::*;

    let style = self.get_style_node();

    let reference_containing_width = original_containing_block.width;

    // `width` has initial value `auto`.
    let auto = Value::Keyword("auto".to_string());
    let mut width = style.value("width").unwrap_or(auto.clone());

    // margin, border, and padding have initial value 0.
    let zero = Value::Length(0.0, Unit::Px);

    let mut margin_left = style.lookup("margin-left", "margin", &zero);
    let mut margin_right = style.lookup("margin-right", "margin", &zero);

    let border_left = style.lookup("border-left-width", "border-width", &zero);
    let border_right = style.lookup("border-right-width", "border-width", &zero);

    let padding_left = style.lookup("padding-left", "padding", &zero);
    let padding_right = style.lookup("padding-right", "padding", &zero);

    let total: f32 = [
      &margin_left,
      &margin_right,
      &border_left,
      &border_right,
      &padding_left,
      &padding_right,
      &width,
    ]
    .iter()
    .map(|v| v.to_px(reference_containing_width))
    .sum();

    // If width is not auto and the total is wider than the container, treat auto margins as 0.
    if width != auto && total > containing_block.content.width {
      if margin_left == auto {
        margin_left = Value::Length(0.0, Unit::Px);
      }
      if margin_right == auto {
        margin_right = Value::Length(0.0, Unit::Px);
      }
    }

    // Adjust used values so that the above sum equals `containing_block.width`.
    // Each arm of the `match` should increase the total width by exactly `underflow`,
    // and afterward all values should be absolute lengths in px.
    let underflow = original_containing_block.width - total;

    match (width == auto, margin_left == auto, margin_right == auto) {
      // If the values are overconstrained, calculate margin_right.
      (false, false, false) => {
        margin_right = Length(
          margin_right.to_px(reference_containing_width) + underflow,
          Px,
        );
      }

      // If exactly one size is auto, its used value follows from the equality.
      (false, false, true) => {
        margin_right = Length(underflow, Px);
      }
      (false, true, false) => {
        margin_left = Length(underflow, Px);
      }

      // If width is set to auto, any other auto values become 0.
      (true, _, _) => {
        if margin_left == auto {
          margin_left = Length(0.0, Px);
        }
        if margin_right == auto {
          margin_right = Length(0.0, Px);
        }

        if underflow >= 0.0 {
          // Expand width to fill the underflow.
          width = Length(underflow, Px);
        } else {
          // Width can't be negative. Adjust the right margin instead.
          width = Length(0.0, Px);
          margin_right = Length(
            margin_right.to_px(reference_containing_width) + underflow,
            Px,
          );
        }
      }

      // If margin-left and margin-right are both auto, their used values are equal.
      (false, true, true) => {
        margin_left = Length(underflow / 2.0, Px);
        margin_right = Length(underflow / 2.0, Px);
      }
    }

    let d = &mut self.dimensions;
    d.content.width = width.to_px(reference_containing_width);

    d.padding.left = padding_left.to_px(reference_containing_width);
    d.padding.right = padding_right.to_px(reference_containing_width);

    d.border.left = border_left.to_px(reference_containing_width);
    d.border.right = border_right.to_px(reference_containing_width);

    d.margin.left = margin_left.to_px(reference_containing_width);
    d.margin.right = margin_right.to_px(reference_containing_width);
  }

  /// Finish calculating the block's edge sizes, and position it within its containing block.
  ///
  /// http://www.w3.org/TR/CSS2/visudet.html#normal-block
  ///
  /// Sets the vertical margin/padding/border dimensions, and the `x`, `y` values.
  fn calculate_block_position(
    &mut self,
    containing_block: Dimensions,
    original_containing_block: &SimpleDimensions,
  ) {
    use Unit::Px;
    use Value::*;
    let style = self.get_style_node();
    let d = &mut self.dimensions;

    // margin, border, and padding have initial value 0.
    let zero = Length(0.0, Px);

    let reference_containing_height = original_containing_block.height;

    // If margin-top or margin-bottom is `auto`, the used value is zero.
    d.margin.top = style
      .lookup("margin-top", "margin", &zero)
      .to_px(reference_containing_height);
    d.margin.bottom = style
      .lookup("margin-bottom", "margin", &zero)
      .to_px(reference_containing_height);

    d.border.top = style
      .lookup("border-top-width", "border-width", &zero)
      .to_px(reference_containing_height);
    d.border.bottom = style
      .lookup("border-bottom-width", "border-width", &zero)
      .to_px(reference_containing_height);

    d.padding.top = style
      .lookup("padding-top", "padding", &zero)
      .to_px(reference_containing_height);
    d.padding.bottom = style
      .lookup("padding-bottom", "padding", &zero)
      .to_px(reference_containing_height);

    d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;

    // Position the box below all the previous boxes in the container.
    d.content.y = containing_block.content.height
      + containing_block.content.y
      + d.margin.top
      + d.border.top
      + d.padding.top;
  }

  /// Lay out the block's children within its content area.
  ///
  /// Sets `self.dimensions.height` to the total content height.
  fn layout_block_children(&mut self, _: &SimpleDimensions) {
    let d = &mut self.dimensions;
    let original_container = SimpleDimensions::from_dimension(d.content.clone());
    for child in &mut self.children {
      child.layout(*d, &original_container);

      // Increment the height so each child is laid out below the previous one.
      d.content.height = d.content.height + child.dimensions.padding_box().height;
    }
  }

  /// Height of a block-level non-replaced element in normal flow with overflow visible.
  fn calculate_block_height(&mut self, original_container: &SimpleDimensions) {
    // If the height is set to an explicit length, use that exact length.
    // Otherwise, just keep the value set by `layout_block_children`.
    if let Some(value) = self.get_style_node().value("height") {
      self.dimensions.content.height = value.to_px(original_container.height);
    }
  }
}

// Build the tree of LayoutBoxes, but don't perform any layout calculations yet.
pub fn build_geometry_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
  // Create the root box.
  let mut root = LayoutBox::new(match style_node.display() {
    DisplayStyle::Block => BoxType::BlockNode(style_node),
    DisplayStyle::Inline => BoxType::InlineNode(style_node),
    DisplayStyle::None => panic!("Root node has display: none."),
  });

  // Create the descendant boxes.
  for child in &style_node.children {
    match child.display() {
      DisplayStyle::Block => root.children.push(build_geometry_tree(child)),
      DisplayStyle::Inline => root
        .get_inline_container()
        .children
        .push(build_geometry_tree(child)),
      DisplayStyle::None => {} // Skip nodes with `display: none;`
    }
  }
  return root;
}

pub fn layout_geometry_tree<'a>(
  node: &'a StyledNode<'a>,
  mut containing_block: Dimensions,
) -> LayoutBox<'a> {
  // The layout algorithm expects the container height to start at 0.
  let original_container = SimpleDimensions::from_dimension(containing_block.content.clone());
  containing_block.content.height = 0.0;

  let mut root_box = build_geometry_tree(node);
  root_box.layout(containing_block, &original_container);
  root_box.dimensions.content.height = original_container.height;
  return root_box;
}
