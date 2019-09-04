pub mod css;
pub mod dom;
pub mod geometry;
pub mod html;
pub mod style;

// pub fn layout_pipeline<'a>() -> &'a geometry::LayoutBox<'a> {
//   let rcdom = html::parse_html_doc();
//   let dom = dom::serialize_rc_dom(rcdom);
//   let stylesheet = css::parse(String::from(include_str!("../../support/style.css")));
//   let style_tree = style::create_styletree(&dom, &stylesheet);

//   let window_dimensions = geometry::Dimensions::new(geometry::Rect { x: 0.0, y: 0.0, width: 1000.0, height: 1000.0});
//   let geometry_tree = geometry::layout_geometry_tree(&style_tree, window_dimensions);

//   println!("{:#?}", geometry_tree);

//   return &geometry_tree;
// }
