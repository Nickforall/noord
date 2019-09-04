use std::collections::HashMap;

use super::css::Value;
use super::dom::{Node, NodeType};

pub type StylePropertyMap = HashMap<String, Value>;

#[derive(Clone, Debug)]
pub struct StyledNode<'a> {
  pub values: StylePropertyMap,
  pub node: &'a Node,
  pub children: Vec<StyledNode<'a>>,
}

pub enum DisplayStyle {
  Inline,
  Block,
  None,
}

impl StyledNode<'_> {
  // Return the specified value of a property if it exists, otherwise `None`.
  pub fn value(&self, name: &str) -> Option<Value> {
    self.values.get(name).map(|v| v.clone())
  }

  // The value of the `display` property (defaults to inline).
  pub fn display(&self) -> DisplayStyle {
    match self.value("display") {
      Some(Value::Keyword(s)) => match &*s {
        "block" => DisplayStyle::Block,
        "none" => DisplayStyle::None,
        _ => DisplayStyle::Inline,
      },
      _ => DisplayStyle::Inline,
    }
  }

  pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
    self
      .value(name)
      .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
  }
}

pub fn create_styletree<'a>(root: &'a Node, stylesheet: &super::css::Stylesheet) -> StyledNode<'a> {
  let mut root = root;
  if let NodeType::Document() = root.node_type {
    root = &root.children.first().unwrap().children.last().unwrap();
  }


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
