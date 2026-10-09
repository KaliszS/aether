use engine::BodyId;
use leptos::prelude::*;

use super::UiState;

#[derive(Clone)]
pub struct BodyEntry {
    pub id: BodyId,
    pub name: String,
}

#[component]
pub fn Sidebar(bodies: Vec<BodyEntry>, ui: UiState) -> impl IntoView {
    let can_compare = move || ui.compared_pair().is_some();
    view! {
        <aside class="sidebar">
            <header>
                <h1>"Aether"</h1>
                <p class="tagline">"Scale and physics of the cosmos"</p>
            </header>
            <h2 class="section-title">"Objects"</h2>
            <ul class="body-list">
                {bodies.into_iter().map(|body| view! { <BodyRow body ui /> }).collect_view()}
            </ul>
            <button
                class="primary-button"
                disabled=move || !can_compare()
                on:click=move |_| ui.comparison_open.set(true)
            >
                "Compare sizes"
            </button>
            <p class="hint">
                {move || if can_compare() { "" } else { "Pick two objects with the + buttons." }}
            </p>
        </aside>
    }
}

#[component]
fn BodyRow(body: BodyEntry, ui: UiState) -> impl IntoView {
    let id = body.id;
    let slot = move || ui.compared_slot(id);
    view! {
        <li class="body-row" class:focused=move || ui.focused.get() == Some(id)>
            <button class="body-name" on:click=move |_| ui.focused.set(Some(id))>
                {body.name}
            </button>
            <button
                class="compare-toggle"
                class:picked=move || slot().is_some()
                title="Pick for size comparison"
                on:click=move |_| ui.toggle_compared(id)
            >
                {move || slot().map_or("+".to_string(), |slot| (slot + 1).to_string())}
            </button>
        </li>
    }
}
