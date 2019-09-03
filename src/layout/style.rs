use std::collections::HashMap;

use super::dom::Node;

type StylePropertyMap = HashMap<String, String>;

pub struct StyledNode<'a> {
  values: StylePropertyMap,
  node: &'a Node,
  children: Vec<StyledNode<'a>>,
}

pub fn style_tree<'a>(root: &'a Node) -> StyledNode<'a> {
  StyledNode {
      node: root,
      values: HashMap::new(),
      children: root.children.iter().map(|child| style_tree(&child)).collect(),
  }
}
