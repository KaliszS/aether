use engine::{Renderer, SimulationClock};
use web_sys::HtmlCanvasElement;

use super::labels::Labels;
use crate::{
    scenes::Scene,
    web::{
        canvas_size::fit_to_display,
        frame_loop::{self, FrameLoop},
        view_rect::ViewRect,
    },
};

pub fn start(
    canvas: HtmlCanvasElement,
    mut renderer: Renderer,
    scene: Scene,
    mut clock: SimulationClock,
    labels: Labels,
) -> FrameLoop {
    frame_loop::start(move |time, dt| {
        if fit_to_display(&canvas) {
            renderer.resize(canvas.width(), canvas.height());
        }
        clock.advance(dt);
        scene.step(clock.seconds, dt);

        let world = scene.world.borrow();
        let camera = scene.camera.borrow();
        renderer.render(&world, &camera, time as f32);
        labels.update(&world, &camera, &ViewRect::of(&canvas));
    })
}
