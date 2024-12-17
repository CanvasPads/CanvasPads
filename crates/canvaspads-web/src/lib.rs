use canvaspads::app::{CanvasElementOptions, Instance, Surface};
use wasm_bindgen::prelude::*;

static CPS_CANVAS_ID: &str = "cps_root";

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub async fn run() {
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
    if let Err(..) = Instance::new(surface).await {
        alert("An error occured.");
    };
}

#[wasm_bindgen]
pub fn start() {
    wasm_bindgen_futures::spawn_local(run());
}
