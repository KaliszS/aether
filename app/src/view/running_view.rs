use engine::{BodyId, Renderer, SimulationClock};
use leptos::{prelude::*, task::spawn_local};
use web_sys::{HtmlCanvasElement, HtmlElement};

use super::{labels::Labels, render_loop};
use crate::{
    input,
    scenes::Scene,
    web::{canvas_size::fit_to_display, dom_events::Listener, frame_loop::FrameLoop},
};

/// Everything that keeps a canvas alive: render loop, labels and input. Stops when dropped.
pub struct RunningView {
    _frames: FrameLoop,
    _input: Vec<Listener>,
}

pub type ViewSlot = StoredValue<Option<RunningView>, LocalStorage>;

/// Starts rendering `scene` into `canvas`. The view lives as long as `slot`, which a
/// component owns, so closing the component stops the GPU work.
pub fn launch_into(
    slot: ViewSlot,
    canvas: HtmlCanvasElement,
    overlay: HtmlElement,
    scene: Scene,
    clock: SimulationClock,
    on_select: impl Fn(BodyId) + 'static,
) {
    fit_to_display(&canvas);
    spawn_local(async move {
        match launch(canvas, overlay, scene, clock, on_select).await {
            Ok(view) => {
                let _ = slot.try_set_value(Some(view));
            }
            Err(e) => log::error!("renderer init failed: {e}"),
        }
    });
}

async fn launch(
    canvas: HtmlCanvasElement,
    overlay: HtmlElement,
    scene: Scene,
    clock: SimulationClock,
    on_select: impl Fn(BodyId) + 'static,
) -> Result<RunningView, String> {
    let renderer = Renderer::new(canvas.clone()).await?;
    let labels = Labels::new(&overlay, &scene.world.borrow(), on_select);
    let input = input::attach(&canvas, scene.camera.clone());
    let frames = render_loop::start(canvas, renderer, scene, clock, labels);
    Ok(RunningView {
        _frames: frames,
        _input: input,
    })
}
