use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <h1>"Aether"</h1>
            <p class="tagline">"Scale and physics of the cosmos"</p>
        </aside>
    }
}
