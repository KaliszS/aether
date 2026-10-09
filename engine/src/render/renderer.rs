use web_sys::HtmlCanvasElement;

use super::{frame_uniforms::FrameUniforms, gpu::Gpu, scene_pass::ScenePass};
use crate::{camera::OrbitCamera, world::World};

pub struct Renderer {
    gpu: Gpu,
    scene: ScenePass,
}

impl Renderer {
    pub async fn new(canvas: HtmlCanvasElement) -> Result<Self, String> {
        let gpu = Gpu::new(canvas).await?;
        let scene = ScenePass::new(&gpu);
        Ok(Self { gpu, scene })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
    }

    pub fn render(&mut self, world: &World, camera: &OrbitCamera, time: f32) {
        let Some((frame, view)) = self.gpu.acquire_frame() else {
            return;
        };
        let uniforms = FrameUniforms::new(world, camera, self.gpu.size(), time);
        self.scene.prepare(&self.gpu.queue, &uniforms);

        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        {
            let mut pass = begin_main_pass(&mut encoder, &view);
            self.scene.draw(&mut pass);
        }
        self.gpu.queue.submit([encoder.finish()]);
        self.gpu.queue.present(frame);
    }
}

fn begin_main_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    view: &'a wgpu::TextureView,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("main"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
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
    })
}
