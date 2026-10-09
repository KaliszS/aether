use std::{cell::RefCell, rc::Rc};

use engine::{OrbitCamera, Renderer};
use leptos::{html, prelude::*, task::spawn_local};

use crate::{canvas_size::fit_to_display, demo::demo_world, input, render_loop};

#[component]
pub fn Viewport() -> impl IntoView {
    let canvas = NodeRef::<html::Canvas>::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas.get() {
            fit_to_display(&canvas);
            spawn_local(async move {
                match Renderer::new(canvas.clone()).await {
                    Ok(renderer) => {
                        let world = Rc::new(RefCell::new(demo_world()));
                        let camera = Rc::new(RefCell::new(OrbitCamera::default()));
                        input::attach(&canvas, camera.clone());
                        render_loop::start(canvas, renderer, world, camera);
                    }
                    Err(e) => log::error!("renderer init failed: {e}"),
                }
            });
        }
    });

    view! {
        <main class="viewport">
            <canvas node_ref=canvas />
        </main>
    }
}
