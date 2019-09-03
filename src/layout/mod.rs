pub mod css;
pub mod dom;
pub mod html;
pub mod parser;
pub mod style;

pub fn layout_pipeline() {
  let rcdom = html::parse_html_doc();
  let noord_dom = dom::serialize_rc_dom(rcdom);
  let stylesheet = css::parse(String::from(include_str!("../../support/style.css")));

  println!("{:#?}", stylesheet)
}
