use wasm_bindgen::JsValue;

pub fn is_svg_tag(tag: &str) -> bool {
    match tag.to_lowercase().as_str() {
        "animate" => true,
        "animatemotion" => true,
        "animatetransform" => true,
        "circle" => true,
        "clippath" => true,
        "defs" => true,
        "desc" => true,
        "discard" => true,
        "ellipse" => true,
        "feblend" => true,
        "fecolormatrix" => true,
        "fecomponenttransfer" => true,
        "fecomposite" => true,
        "feconvolvematrix" => true,
        "fediffuselighting" => true,
        "fedisplacementmap" => true,
        "fedistantlight" => true,
        "fedropshadow" => true,
        "feflood" => true,
        "fefunca" => true,
        "fefuncb" => true,
        "fefuncg" => true,
        "fefuncr" => true,
        "fegaussianblur" => true,
        "feimage" => true,
        "femerge" => true,
        "femergenode" => true,
        "femorphology" => true,
        "feoffset" => true,
        "fepointlight" => true,
        "fespecularlighting" => true,
        "fespotlight" => true,
        "fetile" => true,
        "feturbulence" => true,
        "filter" => true,
        "foreignobject" => true,
        "g" => true,
        "line" => true,
        "lineargradient" => true,
        "marker" => true,
        "mask" => true,
        "metadata" => true,
        "mpath" => true,
        "path" => true,
        "pattern" => true,
        "polygon" => true,
        "polyline" => true,
        "radialgradient" => true,
        "rect" => true,
        "set" => true,
        "stop" => true,
        "svg" => true,
        "switch" => true,
        "symbol" => true,
        "text" => true,
        "textpath" => true,
        "title" => true,
        "tspan" => true,
        "unknown" => true,
        "use" => true,
        "view" => true,
        _ => false,
    }
}


pub fn get_oninput_value(event: &JsValue) -> String {
    let event: web_sys::Event = From::from(event.clone());
    let target: web_sys::EventTarget = event
        .target()
        .expect("target failed");
    let target: JsValue = From::from(target);
    let target: web_sys::HtmlInputElement = From::from(target);
    let value = target.value();
    value
}