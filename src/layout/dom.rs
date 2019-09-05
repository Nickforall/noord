use std::collections::{HashMap, HashSet};

use html5ever::rcdom;
use std::rc::Rc;

type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ElementData {
  pub tag_name: String,
  pub attributes: AttrMap,
  pub is_debug_mode: bool,
}

impl ElementData {
  pub fn id(&self) -> Option<&String> {
    self.attributes.get(&String::from("id"))
  }

  pub fn classlist(&self) -> Option<HashSet<&str>> {
    match self.attributes.get(&String::from("class")) {
      Some(string) => return Some(string.split(' ').collect()),
      None => return None,
    }
  }
}

#[derive(Debug)]
pub struct Node {
  // data common to all nodes:
  pub children: Vec<Node>,

  // data specific to each node type:
  pub node_type: NodeType,

  pub is_debug_mode: bool,
}

impl Node {
  /// Used to identify the given element while debugging
  pub fn debug_identifier(&self) -> String {
    match &self.node_type {
      NodeType::Document() => "Node#Document".to_owned(),
      NodeType::Text(string) => format!("Node#Text({})", &string).to_owned(),
      NodeType::Element(data) => format!(
        "Node#Element#{} classlist={:?} id={}",
        data.tag_name,
        data.classlist().unwrap_or(HashSet::new()),
        data.id().unwrap_or(&"none".to_owned())
      )
      .to_owned(),
    }
  }
}

#[derive(Debug)]
pub enum NodeType {
  Document(),
  Text(String),
  Element(ElementData),
}

pub fn document(children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Document(),
    is_debug_mode: false,
  }
}

pub fn text(data: String) -> Node {
  Node {
    children: Vec::new(),
    node_type: NodeType::Text(data),
    is_debug_mode: false,
  }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  // used to print debug messages in the layout pipeline for processes affecting this element.
  let is_debug_mode = attrs.contains_key("__noord_debug");
  Node {
    children: children,
    node_type: NodeType::Element(ElementData {
      tag_name: name,
      attributes: attrs,
      is_debug_mode,
    }),
    is_debug_mode,
  }
}

pub fn serialize_rc_dom(root: Rc<rcdom::Node>) -> Node {
  let element = Rc::try_unwrap(Rc::clone(&root)).unwrap_err();

  match &element.data {
    rcdom::NodeData::Document => {
      return document(
        element
          .children
          .clone()
          .into_inner()
          .into_iter()
          .map(|child| serialize_rc_dom(child))
          .collect(),
      )
    }
    rcdom::NodeData::Element {
      name,
      attrs,
      template_contents: _,
      mathml_annotation_xml_integration_point: _,
    } => {
      let attributes = attrs.borrow().clone();
      let mut hashmap: AttrMap = HashMap::new();

      for attribute in attributes {
        let key = &*attribute.name.local;
        let value = String::from(attribute.value);
        hashmap.insert(String::from(key), value);
      }

      return elem(
        String::from(&*name.local),
        hashmap,
        element
          .children
          .clone()
          .into_inner()
          .into_iter()
          .map(|child| serialize_rc_dom(child))
          .collect(),
      );
    }
    rcdom::NodeData::Text { contents } => return text(String::from(contents.borrow().clone())),
    _ => panic!("Unrecognized NodeData value on RCDom node"),
  }
}
