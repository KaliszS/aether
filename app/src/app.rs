use leptos::prelude::*;

use crate::{
    scenes::overview_scene,
    ui::{BodyEntry, ComparisonWindow, Sidebar, UiState, Viewport},
};

#[component]
pub fn App() -> impl IntoView {
    let scene = overview_scene();
    let bodies = scene
        .world
        .borrow()
        .bodies()
        .iter()
        .enumerate()
        .map(|(id, body)| BodyEntry {
            id,
            name: body.name.clone(),
        })
        .collect();
    let scene = StoredValue::new_local(scene);
    let ui = UiState::default();

    view! {
        <Sidebar bodies ui />
        <Viewport scene ui />
        {move || {
            ui.comparison_open
                .get()
                .then(|| ui.compared_pair())
                .flatten()
                .map(|pair| view! { <ComparisonWindow scene pair ui /> })
        }}
    }
}
