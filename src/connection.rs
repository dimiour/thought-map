use crate::node::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ConnectionKey(pub u32);

pub struct Connection {
    pub from: NodeKey,
    pub to: NodeKey,
}

impl Connection {
    pub fn line_segment(from: &Node, to: &Node, radius: f32) -> [egui::Pos2; 2] {        
        let full_line = to.position - from.position;
        let unit_segment = full_line / full_line.length();
        
        if radius * 2.0 > full_line.length() {
            return [
                egui::Vec2::ZERO.to_pos2(),
                egui::Vec2::ZERO.to_pos2() 
            ]
        }
        
        [
            (from.position + unit_segment * radius).to_pos2(), 
            (to.position - unit_segment * radius).to_pos2()
        ]
    }
    
    pub fn other(&self, key: &NodeKey) -> NodeKey {
        if self.from == *key {
            self.to.clone()
        } else {
            self.from.clone()
        }
    }
}