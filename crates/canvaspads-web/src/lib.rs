use canvaspads::app::{Application, CanvasElementOptions, Surface};
use wasm_bindgen::prelude::*;

static CPS_CANVAS_ID: &str = "cps_root";

#[wasm_bindgen]
pub fn start() {
    let wrapper = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(CPS_CANVAS_ID)
        .unwrap();
    let elm = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap();
    let _ = wrapper.append_child(&elm);
    let options = CanvasElementOptions {
        elm: elm.dyn_into().unwrap(),
        width: 640,
        height: 480,
        padding_top: 0,
        padding_bottom: 0,
        padding_left: 0,
        padding_right: 0,
    };
    let surface = Surface::from_canvas(options);
    Application::new(surface);
}
