use azul::prelude::*;
use azul::widgets::svg::*;

const WHEEL_SVG: &str = include_str!("wheel.svg");
#[derive(Debug)]
struct Wheel<T: Layout> {
    elements: Vec<String>,
    svg: Option<(SvgCache<T>, Vec<SvgLayerResource>)>,
}
/*
impl<T: Layout> Layout for Wheel<T> {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        if let Some((svg_cache, svg_layers)) = self.svg {
            Svg::with_layers(svg_layers).render_svg(&info.window, &svg_cache)
        } else {
            Button::labeled("Load SVG").dom().with_callback(load_svg)
        }
    }
}
*/
