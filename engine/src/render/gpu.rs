use web_sys::HtmlCanvasElement;
use wgpu::CurrentSurfaceTexture;

pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    /// sRGB view of the surface, so shader output is gamma-correct.
    pub view_format: wgpu::TextureFormat,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
}

impl Gpu {
    pub async fn new(canvas: HtmlCanvasElement) -> Result<Self, String> {
        let size = (canvas.width().max(1), canvas.height().max(1));
        let instance = create_instance().await;
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
            .map_err(|e| e.to_string())?;
        let adapter = request_adapter(&instance, &surface).await?;
        let (device, queue) = request_device(&adapter).await?;
        let (config, view_format) = surface_config(&surface, &adapter, size)?;
        surface.configure(&device, &config);

        Ok(Self {
            device,
            queue,
            view_format,
            surface,
            config,
        })
    }

    pub fn size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let max = self.device.limits().max_texture_dimension_2d;
        let size = (width.clamp(1, max), height.clamp(1, max));
        if size != self.size() {
            (self.config.width, self.config.height) = size;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn acquire_frame(&self) -> Option<(wgpu::SurfaceTexture, wgpu::TextureView)> {
        let frame = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(frame) | CurrentSurfaceTexture::Suboptimal(frame) => {
                frame
            }
            CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return None;
            }
            other => {
                log::warn!("skipped frame: {other:?}");
                return None;
            }
        };
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(self.view_format),
            ..Default::default()
        });
        Some((frame, view))
    }
}

/// WebGPU when the browser has it, WebGL2 otherwise.
async fn create_instance() -> wgpu::Instance {
    wgpu::util::new_instance_with_webgpu_detection(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    })
    .await
}

async fn request_adapter(
    instance: &wgpu::Instance,
    surface: &wgpu::Surface<'_>,
) -> Result<wgpu::Adapter, String> {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(surface),
            ..Default::default()
        })
        .await
        .map_err(|e| e.to_string())?;
    log::info!("GPU backend: {:?}", adapter.get_info().backend);
    Ok(adapter)
}

async fn request_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), String> {
    adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("aether"),
            required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                .using_resolution(adapter.limits()),
            ..Default::default()
        })
        .await
        .map_err(|e| e.to_string())
}

fn surface_config(
    surface: &wgpu::Surface<'_>,
    adapter: &wgpu::Adapter,
    (width, height): (u32, u32),
) -> Result<(wgpu::SurfaceConfiguration, wgpu::TextureFormat), String> {
    let mut config = surface
        .get_default_config(adapter, width, height)
        .ok_or("surface not supported by adapter")?;
    let view_format = config.format.add_srgb_suffix();
    if view_format != config.format {
        config.view_formats.push(view_format);
    }
    Ok((config, view_format))
}
