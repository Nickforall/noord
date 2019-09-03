pub mod html;
pub mod style;

pub mod dom;

pub fn layout_pipeline() {
  let rcdom = html::parse_html_doc();
  let noord_dom = dom::serialize_rc_dom(rcdom);

  println!("{:#?}", noord_dom)
}