mod app;
mod input;
mod scenes;
mod ui;
mod view;
mod web;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Info);
    leptos::mount::mount_to_body(app::App);
}
