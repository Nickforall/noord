pub mod css;
pub mod dom;
pub mod html;
pub mod parser;
pub mod style;

pub fn layout_pipeline() {
  let rcdom = html::parse_html_doc();
  let dom = dom::serialize_rc_dom(rcdom);
  let stylesheet = css::parse(String::from(include_str!("../../support/style.css")));
  let style_tree = style::create_styletree(&dom, &stylesheet);

  println!("{:#?}", style_tree);
}
