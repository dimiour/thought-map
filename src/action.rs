use egui::Pos2;
use crate::{node::NodeKey, thought::Thought};

#[derive(Clone)]
pub struct Actions {
    pub start_drag: Option<(usize, Pos2, bool)>,
    pub thought_drag: Option<Thought>,
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            start_drag: None,
            thought_drag: None,
        }
    }
}