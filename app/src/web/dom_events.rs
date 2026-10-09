use std::any::Any;

use wasm_bindgen::{JsCast, closure::Closure, convert::FromWasmAbi};
use web_sys::{AddEventListenerOptions, EventTarget};

/// An event listener that is removed when dropped.
pub struct Listener {
    target: EventTarget,
    event: String,
    function: js_sys::Function,
    _closure: Box<dyn Any>,
}

impl Drop for Listener {
    fn drop(&mut self) {
        let _ = self
            .target
            .remove_event_listener_with_callback(&self.event, &self.function);
    }
}

/// Non-passive so handlers can call `prevent_default` (stops page scroll/zoom).
pub fn listen<E: FromWasmAbi + 'static>(
    target: &EventTarget,
    event: &str,
    handler: impl FnMut(E) + 'static,
) -> Listener {
    let closure = Closure::<dyn FnMut(E)>::new(handler);
    let function: js_sys::Function = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
    let options = AddEventListenerOptions::new();
    options.set_passive(false);
    target
        .add_event_listener_with_callback_and_add_event_listener_options(event, &function, &options)
        .unwrap();
    Listener {
        target: target.clone(),
        event: event.to_owned(),
        function,
        _closure: Box::new(closure),
    }
}
