use engine::{BodyId, SimulationClock};
use leptos::{html, prelude::*};

use super::UiState;
use crate::{
    scenes::{Scene, caption, comparison_scene},
    view::{ViewSlot, launch_into},
};

/// Modal with its own canvas showing two bodies side by side at true relative size.
/// The main simulation keeps running behind it.
#[component]
pub fn ComparisonWindow(
    scene: StoredValue<Scene, LocalStorage>,
    pair: [BodyId; 2],
    ui: UiState,
) -> impl IntoView {
    let (pair_scene, text) = scene.with_value(|scene| {
        let world = scene.world.borrow();
        (comparison_scene(&world, pair), caption(&world, pair))
    });
    let pair_scene = StoredValue::new_local(pair_scene);
    let canvas = NodeRef::<html::Canvas>::new();
    let overlay = NodeRef::<html::Div>::new();
    let running: ViewSlot = StoredValue::new_local(None);

    Effect::new(move |_| {
        let (Some(canvas), Some(overlay)) = (canvas.get(), overlay.get()) else {
            return;
        };
        let frozen = SimulationClock::new(0.0, 0.0);
        let on_select = move |id| pair_scene.with_value(|scene| scene.select(id));
        launch_into(
            running,
            canvas,
            overlay.into(),
            pair_scene.get_value(),
            frozen,
            on_select,
        );
    });

    let close = move |_| ui.comparison_open.set(false);
    view! {
        <div class="modal-backdrop" on:click=close>
            <section class="modal" on:click=|e| e.stop_propagation()>
                <header class="modal-header">
                    <h2>"Size comparison"</h2>
                    <button class="icon-button" aria-label="Close" on:click=close>
                        "×"
                    </button>
                </header>
                <p class="modal-caption">{text}</p>
                <div class="modal-viewport">
                    <canvas node_ref=canvas />
                    <div class="labels" node_ref=overlay></div>
                </div>
            </section>
        </div>
    }
}
