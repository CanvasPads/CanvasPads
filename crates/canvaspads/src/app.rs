use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use web_sys::HtmlCanvasElement;

pub trait WindowHandle: HasWindowHandle + HasDisplayHandle {}

/// An interface for an application.
///
/// # Examples
///
/// ```rust,ignore
/// use canvaspads::app::Application;
///
/// // A window should implements both `HasWindowHandle` and `HasDisplayHandle`
/// let app = Application::builder().with_rwh(window.raw_window_handle()).build();
/// ```
pub struct Application<'a> {
    surface: Surface<'a>,
}

pub struct Window<'a> {
    handle: Box<dyn WindowHandle + 'a>,
}

pub struct CanvasElement {
    pub elm: HtmlCanvasElement,
    pub width: i32,
    pub height: i32,
    pub padding: (i32, i32, i32, i32),
}

impl CanvasElement {
    pub fn from_options(options: CanvasElementOptions) -> Self {
        CanvasElement {
            elm: options.elm,
            width: options.width,
            height: options.height,
            padding: (
                options.padding_top,
                options.padding_bottom,
                options.padding_left,
                options.padding_right,
            ),
        }
    }
}

pub enum Surface<'a> {
    Window(Window<'a>),
    CanvasElement(CanvasElement),
}

pub struct CanvasElementOptions {
    pub elm: HtmlCanvasElement,
    pub width: i32,
    pub height: i32,
    pub padding_top: i32,
    pub padding_bottom: i32,
    pub padding_left: i32,
    pub padding_right: i32,
}

impl<'a> Surface<'a> {
    pub fn from_rwh() -> Self {
        unimplemented!()
    }
    pub fn from_canvas(options: CanvasElementOptions) -> Self {
        let elm = CanvasElement::from_options(options);
        Surface::CanvasElement(elm)
    }
}

impl<'a> Application<'a> {
    /// Create new application
    pub fn new(surface: Surface<'a>) -> Self {
        Application { surface }
    }
}
