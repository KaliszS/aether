#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// What the user did, in CSS pixels, independent of the input device.
#[derive(Debug)]
pub enum Gesture {
    Orbit {
        dx: f64,
        dy: f64,
    },
    Pan {
        dx: f64,
        dy: f64,
    },
    /// `factor` < 1 zooms in, towards `at`.
    Zoom {
        factor: f64,
        at: Point,
    },
}

/// What a single-pointer drag does; chosen when the first pointer goes down.
#[derive(Clone, Copy, Default)]
pub enum DragMode {
    #[default]
    Orbit,
    Pan,
}

struct Pointer {
    id: i32,
    at: Point,
}

/// Turns raw pointer positions into gestures: one pointer drags, two pointers pan and pinch.
#[derive(Default)]
pub struct PointerTracker {
    pointers: Vec<Pointer>,
    drag_mode: DragMode,
}

impl PointerTracker {
    pub fn press(&mut self, id: i32, at: Point, drag_mode: DragMode) {
        if self.pointers.is_empty() {
            self.drag_mode = drag_mode;
        }
        self.pointers.push(Pointer { id, at });
    }

    pub fn release(&mut self, id: i32) {
        self.pointers.retain(|p| p.id != id);
    }

    pub fn move_to(&mut self, id: i32, at: Point) -> Vec<Gesture> {
        let Some(index) = self.pointers.iter().position(|p| p.id == id) else {
            return Vec::new();
        };
        let (center_before, spread_before) = (self.center(), self.spread());
        self.pointers[index].at = at;
        let (center_after, spread_after) = (self.center(), self.spread());

        let (dx, dy) = (
            center_after.x - center_before.x,
            center_after.y - center_before.y,
        );
        match (self.pointers.len(), spread_before, spread_after) {
            (1, ..) => match self.drag_mode {
                DragMode::Orbit => vec![Gesture::Orbit { dx, dy }],
                DragMode::Pan => vec![Gesture::Pan { dx, dy }],
            },
            (2, Some(before), Some(after)) if after > 0.0 => {
                vec![
                    Gesture::Pan { dx, dy },
                    Gesture::Zoom {
                        factor: before / after,
                        at: center_after,
                    },
                ]
            }
            _ => Vec::new(),
        }
    }

    fn center(&self) -> Point {
        let n = self.pointers.len().max(1) as f64;
        let (x, y) = self
            .pointers
            .iter()
            .fold((0.0, 0.0), |(x, y), p| (x + p.at.x, y + p.at.y));
        Point { x: x / n, y: y / n }
    }

    fn spread(&self) -> Option<f64> {
        match self.pointers.as_slice() {
            [a, b] => Some((a.at.x - b.at.x).hypot(a.at.y - b.at.y)),
            _ => None,
        }
    }
}
