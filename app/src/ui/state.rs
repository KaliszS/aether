use engine::BodyId;
use leptos::prelude::*;

/// Reactive UI state shared by the sidebar, the main view and the comparison window.
#[derive(Clone, Copy)]
pub struct UiState {
    /// Body the main camera flies to and follows.
    pub focused: RwSignal<Option<BodyId>>,
    /// Up to two bodies picked for the size comparison, oldest first.
    pub compared: RwSignal<Vec<BodyId>>,
    pub comparison_open: RwSignal<bool>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            focused: RwSignal::new(None),
            compared: RwSignal::new(Vec::new()),
            comparison_open: RwSignal::new(false),
        }
    }
}

impl UiState {
    /// Adds or removes a body; picking a third drops the oldest.
    pub fn toggle_compared(&self, id: BodyId) {
        self.compared.update(|ids| {
            if let Some(index) = ids.iter().position(|&other| other == id) {
                ids.remove(index);
            } else {
                if ids.len() == 2 {
                    ids.remove(0);
                }
                ids.push(id);
            }
        });
    }

    /// 0 or 1 if the body is picked for comparison.
    pub fn compared_slot(&self, id: BodyId) -> Option<usize> {
        self.compared
            .with(|ids| ids.iter().position(|&other| other == id))
    }

    pub fn compared_pair(&self) -> Option<[BodyId; 2]> {
        self.compared
            .with(|ids| <[BodyId; 2]>::try_from(ids.as_slice()).ok())
    }
}
