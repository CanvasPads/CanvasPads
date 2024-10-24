use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub trait WindowHandle: HasWindowHandle + HasDisplayHandle {}

pub struct Application<'a> {
    window_handle: Box<dyn WindowHandle + 'a>,
}

impl<'a> Application<'a> {
    pub fn new() -> ApplicationBuilder<'a> {
        ApplicationBuilder::default()
    }
}

#[derive(Default)]
pub struct ApplicationBuilder<'a> {
    window_handle: Option<Box<dyn WindowHandle + 'a>>,
}

impl<'a> ApplicationBuilder<'a> {
    pub fn with_rwh<T: WindowHandle + 'a>(&mut self, handle: T) -> &mut Self {
        self.window_handle = Some(Box::new(handle));
        self
    }
}
