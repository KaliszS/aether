use std::{cell::RefCell, rc::Rc};

use engine::{OrbitCamera, Renderer, World};
use web_sys::HtmlCanvasElement;

use crate::{canvas_size::fit_to_display, frame_loop};

pub fn start(
    canvas: HtmlCanvasElement,
    mut renderer: Renderer,
    world: Rc<RefCell<World>>,
    camera: Rc<RefCell<OrbitCamera>>,
) {
    frame_loop::start(move |time, dt| {
        if fit_to_display(&canvas) {
            renderer.resize(canvas.width(), canvas.height());
        }
        let mut camera = camera.borrow_mut();
        camera.update(dt);
        renderer.render(&world.borrow(), &camera, time as f32);
    });
}
