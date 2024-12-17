use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use web_sys::HtmlCanvasElement;

use crate::render::driver::{
    wgpu::{WgpuDriver, WgpuDriverError},
    GraphicsDriver,
};

pub trait WindowHandle: HasWindowHandle + HasDisplayHandle {}

pub enum InstanceError {
    WgpuDriverError(WgpuDriverError),
}

pub type InstanceResult<T> = Result<T, InstanceError>;

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
pub struct Instance<'a> {
    driver: Box<dyn GraphicsDriver + 'a>,
}

pub struct Window<'a> {
    handle: Box<dyn WindowHandle + 'a>,
}

pub struct CanvasElement {
    pub elm: HtmlCanvasElement,
    pub width: u32,
    pub height: u32,
    pub padding: (u32, u32, u32, u32),
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
    pub width: u32,
    pub height: u32,
    pub padding_top: u32,
    pub padding_bottom: u32,
    pub padding_left: u32,
    pub padding_right: u32,
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

impl<'a> Instance<'a> {
    /// Create new instance
    pub async fn new(surface: Surface<'a>) -> InstanceResult<Self> {
        let driver: Box<dyn GraphicsDriver> = match surface {
            Surface::CanvasElement(elm) => {
                let driver = match WgpuDriver::from_canvas(elm).await {
                    Ok(d) => d,
                    Err(err) => return Err(InstanceError::WgpuDriverError(err)),
                };
                Box::new(driver)
            }
            _ => {
                unimplemented!()
            }
        };
        Ok(Instance { driver })
    }

    pub async fn run() -> InstanceResult<()> {
        Ok(())
    }
}
