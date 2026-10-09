use engine::Renderer;
use leptos::{html, prelude::*, task::spawn_local};

use crate::render_loop::{fit_to_display, start_render_loop};

#[component]
pub fn Viewport() -> impl IntoView {
    let canvas = NodeRef::<html::Canvas>::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas.get() {
            fit_to_display(&canvas);
            spawn_local(async move {
                match Renderer::new(canvas.clone()).await {
                    Ok(renderer) => start_render_loop(canvas, renderer),
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
