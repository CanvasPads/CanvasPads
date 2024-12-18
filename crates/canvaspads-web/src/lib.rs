use std::{cell::RefCell, rc::Rc};
use canvaspads::app::{CanvasElementOptions, Instance, Surface};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

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
    let elm: Rc<RefCell<HtmlCanvasElement>> = Rc::new(RefCell::new(elm.dyn_into().unwrap()));
    let options = CanvasElementOptions {
        elm: elm.borrow().clone(),
        width: 640,
        height: 480,
        padding_top: 0,
        padding_bottom: 0,
        padding_left: 0,
        padding_right: 0,
    };
    let surface = Surface::from_canvas(options);
    let instance = match Instance::new(surface).await {
        Ok(i) => i,
        Err(..) => {
            alert("An error occured.");
            return;
        }
    };

    let closure = Closure::wrap(Box::new(|e: web_sys::TouchEvent| {
        //
    }) as Box<dyn FnMut(_)>);
    elm.borrow()
        .add_event_listener_with_callback("click", &closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    let _ = wrapper.append_child(&elm.borrow());
}

#[wasm_bindgen]
pub fn start() {
    wasm_bindgen_futures::spawn_local(run());
}
