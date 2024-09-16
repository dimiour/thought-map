use egui::*;

use crate::thought::Thought;
use crate::connection::ConnectionKey;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct NodeKey(pub u32);

#[derive(Clone)]
pub struct Node {
    pub connections: Vec<ConnectionKey>,
    
    pub thought: Thought,
    
    pub position: Vec2,
    
    pub velocity: Vec2,
}

impl Node {
    pub fn new(position: Vec2, thought: Thought) -> Self {
        Self {
            connections: Vec::new(),
            thought,
            position,
            velocity: Vec2::ZERO,
        }
    }
    
    pub fn tick(&mut self) {
        self.position += self.velocity;
        self.velocity *= 0.9;
    }
    
    pub fn connection(&mut self, rhs: &Self) {
        let difference = rhs.position - self.position;
        let length = 200.0;

        let change = difference / difference.length();
        
        self.velocity += change * (difference.length() - length) * 0.01
    }
    
    pub fn repulse(&mut self, rhs: &Self) {     
        let difference = rhs.position - self.position;
        let length = 100.0;
        
        if difference.length() > length { return }
        
        let change = difference / difference.length();
        
        self.velocity += change * (difference.length() - length) * 0.02
    }
}