use std::rc::Rc;

use engine::{BodyId, OrbitCamera, World};
use leptos::prelude::document;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement};

use crate::web::{
    dom_events::{Listener, listen},
    view_rect::{Point, ViewRect},
};

/// Labels closer than this to their parent's are hidden to avoid clutter.
const MIN_PARENT_SPACING_PX: f64 = 24.0;
/// Above this on-screen radius the body is clearly visible, so the marker ring hides.
const RING_RADIUS_PX: f64 = 7.0;
const NAME_GAP_PX: f64 = 6.0;

/// Clickable body names drawn over the canvas as HTML.
pub struct Labels {
    items: Vec<Label>,
}

struct Label {
    root: HtmlElement,
    ring: HtmlElement,
    name: HtmlElement,
    _click: Listener,
}

impl Labels {
    pub fn new(
        container: &HtmlElement,
        world: &World,
        on_select: impl Fn(BodyId) + 'static,
    ) -> Self {
        let on_select = Rc::new(on_select);
        let items = world
            .bodies()
            .iter()
            .enumerate()
            .map(|(id, body)| {
                let on_select = on_select.clone();
                let label = Label::new(&body.name, move || on_select(id));
                container.append_child(&label.root).unwrap();
                label
            })
            .collect();
        Self { items }
    }

    pub fn update(&self, world: &World, camera: &OrbitCamera, view: &ViewRect) {
        for (label, body) in self.items.iter().zip(world.bodies()) {
            let Some(at) = camera.project(body.position) else {
                label.hide();
                continue;
            };
            let crowded = body.orbit.is_some_and(|orbit| {
                camera
                    .project(world.body(orbit.parent).position)
                    .is_some_and(|parent| {
                        view.view_to_pixels((at - parent).length()) < MIN_PARENT_SPACING_PX
                    })
            });
            if crowded {
                label.hide();
                continue;
            }
            let radius_px = view.view_to_pixels(camera.apparent_radius(body.position, body.radius));
            label.show(view.to_local(at), radius_px);
        }
    }
}

impl Label {
    fn new(text: &str, on_click: impl Fn() + 'static) -> Self {
        let root = element("button", "label");
        let ring = element("span", "label-ring");
        let name = element("span", "label-name");
        name.set_text_content(Some(text));
        root.append_child(&ring).unwrap();
        root.append_child(&name).unwrap();
        let _click = listen(&root, "click", move |_: Event| on_click());
        Self {
            root,
            ring,
            name,
            _click,
        }
    }

    fn show(&self, at: Point, radius_px: f64) {
        let style = self.root.style();
        style.set_property("display", "").unwrap();
        style
            .set_property(
                "transform",
                &format!("translate({:.1}px, {:.1}px)", at.x, at.y),
            )
            .unwrap();
        let ring_visible = radius_px < RING_RADIUS_PX;
        self.ring
            .style()
            .set_property("opacity", if ring_visible { "1" } else { "0" })
            .unwrap();
        let gap = radius_px.max(RING_RADIUS_PX) + NAME_GAP_PX;
        self.name
            .style()
            .set_property("transform", &format!("translateX({gap:.1}px)"))
            .unwrap();
    }

    fn hide(&self) {
        self.root.style().set_property("display", "none").unwrap();
    }
}

fn element(tag: &str, class: &str) -> HtmlElement {
    let element: HtmlElement = document().create_element(tag).unwrap().unchecked_into();
    element.set_class_name(class);
    element
}
