pub mod wgpu;

pub trait GraphicsDriver {
    fn resize_surface(&mut self, width: u32, height: u32);
}
