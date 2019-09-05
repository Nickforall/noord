
use html5ever::driver::ParseOpts;
use html5ever::rcdom::RcDom;
use html5ever::rcdom::Node;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document};
use std::path::Path;

pub fn parse_html_doc(path_str: String) -> std::rc::Rc<Node> {
  let opts = ParseOpts {
      tree_builder: TreeBuilderOpts {
          drop_doctype: true,
          ..Default::default()
      },
      ..Default::default()
  };

  let dom = parse_document(RcDom::default(), opts)
      .from_utf8()
      .from_file(Path::new(&path_str))
      .unwrap();

  return dom.document;
}