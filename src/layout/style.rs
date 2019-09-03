use std::collections::HashMap;

use super::dom::{Node, NodeType};

pub type StylePropertyMap = HashMap<String, crate::layout::css::Value>;

#[derive(Debug)]
pub struct StyledNode<'a> {
  values: StylePropertyMap,
  node: &'a Node,
  children: Vec<StyledNode<'a>>,
}

pub fn create_styletree<'a>(root: &'a Node, stylesheet: &super::css::Stylesheet) -> StyledNode<'a> {
  StyledNode {
    node: root,
    values: match root.node_type {
      NodeType::Element(ref data) => stylesheet.specified_values_for_element(data),
      _ => HashMap::new(),
    },
    children: root
      .children
      .iter()
      .map(|child| create_styletree(&child, stylesheet))
      .collect(),
  }
}
