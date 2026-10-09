mod app;
mod canvas_size;
mod components;
mod demo;
mod frame_loop;
mod input;
mod render_loop;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Info);
    leptos::mount::mount_to_body(app::App);
}
