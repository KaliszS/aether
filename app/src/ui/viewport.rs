use engine::SimulationClock;
use leptos::{html, prelude::*};

use super::UiState;
use crate::{
    scenes::{Scene, seconds_since_j2000},
    view::{ViewSlot, launch_into},
};

/// Simulated seconds per real second: one day per second.
const TIME_SPEED: f64 = 86_400.0;

#[component]
pub fn Viewport(scene: StoredValue<Scene, LocalStorage>, ui: UiState) -> impl IntoView {
    let canvas = NodeRef::<html::Canvas>::new();
    let overlay = NodeRef::<html::Div>::new();
    let running: ViewSlot = StoredValue::new_local(None);

    Effect::new(move |_| {
        let (Some(canvas), Some(overlay)) = (canvas.get(), overlay.get()) else {
            return;
        };
        let clock = SimulationClock::new(seconds_since_j2000(js_sys::Date::now()), TIME_SPEED);
        let on_select = move |id| ui.focused.set(Some(id));
        launch_into(
            running,
            canvas,
            overlay.into(),
            scene.get_value(),
            clock,
            on_select,
        );
    });

    Effect::new(move |_| {
        if let Some(id) = ui.focused.get() {
            scene.with_value(|scene| scene.select(id));
        }
    });

    view! {
        <main class="viewport">
            <canvas node_ref=canvas />
            <div class="labels" node_ref=overlay></div>
        </main>
    }
}
