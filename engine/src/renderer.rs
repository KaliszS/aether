use web_sys::HtmlCanvasElement;

use crate::{background::Background, gpu::Gpu};

pub struct Renderer {
    gpu: Gpu,
    background: Background,
}

impl Renderer {
    pub async fn new(canvas: HtmlCanvasElement) -> Result<Self, String> {
        let gpu = Gpu::new(canvas).await?;
        let background = Background::new(&gpu);
        Ok(Self { gpu, background })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
    }

    pub fn render(&mut self, time: f32) {
        let Some((frame, view)) = self.gpu.acquire_frame() else {
            return;
        };
        self.background
            .prepare(&self.gpu.queue, self.gpu.size(), time);

        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.background.draw(&mut pass);
        }
        self.gpu.queue.submit([encoder.finish()]);
        self.gpu.queue.present(frame);
    }
}
