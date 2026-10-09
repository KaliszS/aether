use leptos::prelude::*;

use crate::components::{Sidebar, Viewport};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Sidebar />
        <Viewport />
    }
}
