use std::borrow::Cow;
use std::fmt::Display;

use crate::app::CanvasElement;
use crate::render::driver::GraphicsDriver;

const SRC: &str = r###"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
"###;

pub struct WgpuDriver<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    width: u32,
    height: u32,
    render_pipeline: wgpu::RenderPipeline,
}

pub enum WgpuDriverError {
    CreateSurfaceError,
    AdapterNotFound,
    CreateDeviceError,
}

impl Display for WgpuDriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WgpuDriverError::CreateSurfaceError => {
                write!(f, "Failed to create surface")
            }
            WgpuDriverError::AdapterNotFound => {
                write!(f, "Failed to find an appropriate adapter")
            }
            WgpuDriverError::CreateDeviceError => {
                write!(f, "Failed to create device")
            }
        }
    }
}

pub type WgpuDriverResult<T> = Result<T, WgpuDriverError>;

impl<'a> WgpuDriver<'a> {
    async fn init_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface<'a>,
    ) -> WgpuDriverResult<wgpu::Adapter> {
        match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
        {
            Some(a) => Ok(a),
            None => Err(WgpuDriverError::AdapterNotFound),
        }
    }

    async fn init(
        instance: wgpu::Instance,
        surface: wgpu::Surface<'a>,
        width: u32,
        height: u32,
    ) -> WgpuDriverResult<Self> {
        let adapter = Self::init_adapter(&instance, &surface).await?;

        let (device, queue) = match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
        {
            Ok(r) => r,
            Err(..) => return Err(WgpuDriverError::CreateDeviceError),
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(SRC)),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(swapchain_format.into())],
            }),
            multiview: None,
            cache: None,
        });

        let config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &config);

        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&render_pipeline);
            rpass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(WgpuDriver {
            surface,
            device,
            queue,
            config,
            width,
            height,
            render_pipeline,
        })
    }

    fn reconfigure_surface_size(&mut self) {
        self.config.width = self.width;
        self.config.height = self.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub async fn from_canvas(canvas: CanvasElement) -> WgpuDriverResult<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface_target = wgpu::SurfaceTarget::Canvas(canvas.elm);
        let surface = match instance.create_surface(surface_target) {
            Ok(s) => s,
            Err(..) => return Err(WgpuDriverError::CreateSurfaceError),
        };

        Self::init(instance, surface, canvas.width, canvas.height).await
    }
}

impl<'a> GraphicsDriver for WgpuDriver<'a> {
    fn resize_surface(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.reconfigure_surface_size();
    }
}
