use std::collections::HashMap;

use html5ever::rcdom;
use std::rc::Rc;

#[derive(Debug)]
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
      // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Document(),
    Text(String),
    Element(ElementData),
}

pub fn document(children: Vec<Node>) -> Node {
    Node { children: children, node_type: NodeType::Document() }
}

pub fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}

pub fn serialize_rc_dom(root: Rc<rcdom::Node>) -> Node {
  let element = Rc::try_unwrap(Rc::clone(&root)).unwrap_err();

  match &element.data {
    rcdom::NodeData::Document => {
      return document(element.children.clone().into_inner().into_iter().map(|child| serialize_rc_dom(child)).collect())
    },
    rcdom::NodeData::Element { name, attrs: _, template_contents: _, mathml_annotation_xml_integration_point: _ } => {
      return elem(String::from(&*name.local), HashMap::new(), element.children.clone().into_inner().into_iter().map(|child| serialize_rc_dom(child)).collect())
    },
    rcdom::NodeData::Text { contents } => {
      return text(String::from(contents.borrow().clone()))
    }
    _ => panic!("faka")
  }
}