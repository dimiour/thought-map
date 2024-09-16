use core::f32;
use std::collections::HashMap;
use eframe::glow::TIME_ELAPSED;
use egui::*;

use crate::node::*;
use crate::connection::*;
use crate::action::*;
use crate::thought::*;

pub struct App {
    nodes: HashMap<NodeKey, Node>,
    
    new_node_key: NodeKey,
    
    connections: HashMap<ConnectionKey, Connection>,
    
    new_connection_key: ConnectionKey,
    
    actions: Actions,
    
    selected: Option<NodeKey>,
    
    inbox: Vec<Thought>,
    
    new_thoughts: Vec<Thought>,
    
    mode: Mode,
}

enum Mode {
    Normal,
    Connect,
}

impl Mode {
    fn next(&self) -> Self {
        match self {
            Self::Normal => Self::Connect,
            Self::Connect => Self::Normal,
        }
    }
    
    fn text(&self) -> &str {
        match self {
            Self::Normal => "Normal",
            Self::Connect => "Connect",
        }
    }
    
    fn is_normal(&self) -> bool {
        match self {
            Self::Normal => true,
            _ => false,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            new_node_key: NodeKey(1),
            connections: HashMap::new(),
            new_connection_key: ConnectionKey(1),
            actions: Actions::default(),
            selected: None,
            inbox: Vec::new(),
            new_thoughts: Vec::new(),
            mode: Mode::Normal,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        
        self.node_physics();
        
        self.draw(ctx);
    }
    

}



impl App {
    fn side_panel(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.heading("Inbox");
        ui.separator();
        
        let painter = ctx.layer_painter(LayerId::new(Order::Background, Id::new("nodes")));
        
        for (index, thought) in self.inbox.clone().iter().enumerate() {
            let inbox_thought = ui.add(Label::new(&thought.contents).sense(Sense::click()).sense(Sense::drag()));
            if inbox_thought.double_clicked() {
                self.inbox.remove(index);
                break
            }
            
            if let Some(pointer_pos) = ctx.pointer_hover_pos() {
                if inbox_thought.dragged() {
                    if let Some((drag_index, start_pos, ended)) = self.actions.start_drag {
                        if pointer_pos.distance(start_pos) > 20.0 && !ended {
                            self.actions.start_drag.as_mut().unwrap().2 = true;
                            self.actions.thought_drag = Some(thought.clone());
                            self.inbox.remove(drag_index);
                            //painter.text(pointer_pos, Align2::CENTER_CENTER, &thought.contents, FontId::default(), Color32::WHITE);
                        }
                    }
                }
                
                if inbox_thought.drag_started() {
                    self.actions.start_drag = Some((index, pointer_pos, false));
                }
            }
            
            if inbox_thought.drag_released() {
                self.actions.start_drag = None;
            }
                            
            ui.separator();
        }
        
        if ui.ui_contains_pointer() {
            if let Some(thought) = self.actions.thought_drag.clone() {
                let pointer_pos = ctx.input().pointer.hover_pos().unwrap();
                painter.text(pointer_pos, Align2::CENTER_CENTER, &thought.contents, FontId::default(), Color32::WHITE);
                
                if !ctx.input().pointer.any_down() {
                    self.actions.thought_drag = None;
                    self.inbox.push(thought.clone());
                }
            }
        }
    }
    
    fn central_panel(&mut self, ctx: &Context, ui: &mut Ui) {
        let radius = 10.0;
        let line_cuttoff = radius * 4.0;
        
        if ui.button("New Thought").clicked() || (ctx.input().key_pressed(Key::Enter) && self.new_thoughts.is_empty()) {        
            self.new_thoughts.push(Thought::new(String::new()));
        }
        
        if ui.button(self.mode.text()).clicked() {
            self.mode = self.mode.next();
            self.selected = None;
        }
        
        let painter = ctx.layer_painter(LayerId::new(Order::Background, Id::new("nodes"))).with_clip_rect(ui.max_rect());
        
        let pointer_in_map = ui.max_rect().contains(ctx.input().pointer.hover_pos().unwrap_or(Pos2::new(f32::NAN, 0.0)));
        
        let hovered = self.get_node_at(ctx.input().pointer.hover_pos().unwrap_or(Pos2::new(f32::NAN, 0.0)).to_vec2()-ctx.input().pointer.delta());

        if self.mode.is_normal() {
            if let Some(hovered_key) = hovered {
                if ctx.input().pointer.primary_down() && pointer_in_map {
                    if let Some(selected) = self.nodes.get_mut(&hovered_key) {
                        selected.velocity = Vec2::ZERO;
                        selected.position += ctx.input().pointer.delta();
                    }
                }
                
                if ctx.input().pointer.any_click() && pointer_in_map {
                    self.selected = match self.selected {
                        Some(selected_key) => {
                            if selected_key == hovered_key {
                                None
                            } else {
                                Some(hovered_key)
                            }
                        },
                        
                        None => Some(hovered_key),
                    };
                }
            }
        } else {
            if let Some(start_node_key) = self.selected.clone() {
                if let Some(pointer_pos) = ctx.input().pointer.hover_pos() {
                    let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("connection_preview")));
                    let start_node = self.nodes.get(&start_node_key).unwrap();
                    
                    
                    // painter.line_segment(
                    //     [start_node.position.to_pos2(), pointer_pos],
                    //     Stroke::new(1.0, Color32::LIGHT_BLUE),
                    // );

                    // if ctx.input().pointer.any_released() {
                    //     if let Some(end_node_key) = hovered {
                    //         if start_node_key != end_node_key && !self.connections.values().any(
                    //             |connection| 
                    //             connection.from == start_node_key && connection.to == end_node_key 
                    //             || connection.from == end_node_key && connection.to == start_node_key
                    //         ) {
                    //             self.create_connection(&start_node_key, &end_node_key);
                    //         }
                    //     } else if !pointer_in_map {
                    //         if let Some(start_node) = self.nodes.get(&start_node_key) {
                    //             self.inbox.push(start_node.thought.clone());
                    //             self.remove_node(start_node_key);
                    //         }
                    //     }
                        
                    //     self.selected = None;
                    // }
                }
            } else {
                if let Some(hovered_key) = hovered {
                    if ctx.input().pointer.any_pressed() && pointer_in_map {
                        self.selected = Some(hovered_key);
                    }
                }
            }
        }
                    
        for (key, node) in self.nodes.iter() {            
            let color = if self.selected == Some(*key) {
                Color32::WHITE
            } else {
                if let Some(hovered_key) = hovered {
                    if key == &hovered_key {
                        Color32::LIGHT_GRAY
                    } else {
                        Color32::GRAY
                    }
                } else {
                    Color32::GRAY
                }
            };
            
            painter.circle_stroke(node.position.to_pos2(), radius, Stroke::new(0.3, color));
            painter.text(Pos2::new(node.position.x, node.position.y + 20.0), Align2::CENTER_CENTER, &node.thought.contents, FontId::default(), color);
        }
        
        for connection in &self.connections.values().collect::<Vec<_>>() {
            let from = self.nodes.get(&connection.from).unwrap();
            let to = self.nodes.get(&connection.to).unwrap();
            
            let color = if let Some(hovered_key) = hovered {
                if connection.from == hovered_key || connection.to == hovered_key {
                    Color32::LIGHT_GRAY
                } else {
                    Color32::GRAY
                }
            } else {
                Color32::GRAY
            };
            
            
            painter.line_segment(
                Connection::line_segment(from, to, line_cuttoff), 
                Stroke::new(0.3, color)
            );
        }
            
        for (index, new_thought) in self.new_thoughts.iter_mut().enumerate() {   
            if let Some(is_submited) = Window::new("New Thought").id(Id::new(index)).collapsible(false).show(ctx, |ui| {
                let response = ui.text_edit_singleline(&mut new_thought.contents);
                
                if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    return Some(true);
                }
                
                else if !response.has_focus() && ui.input().key_pressed(egui::Key::Escape) {
                    return Some(false);
                }
                
                else if !response.has_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    response.request_focus();
                }
                
                ui.horizontal(|ui| {
                    if ui.button("Submit").clicked() {
                        Some(true)
                    } else if ui.button("Cancel").clicked() {
                        Some(false)
                    } else {
                        None
                    }
                }).inner
            }).unwrap().inner.unwrap() {
                if is_submited && new_thought.contents.len() > 0 {
                    self.inbox.push(new_thought.clone());
                }
                
                self.new_thoughts.remove(index);
                
                break;
            }
        }
        
        if ui.ui_contains_pointer() {
            if let Some(thought) = self.actions.thought_drag.clone() {
                let pointer_pos = ctx.input().pointer.hover_pos().unwrap();
                painter.text(pointer_pos, Align2::CENTER_CENTER, &thought.contents, FontId::default(), Color32::WHITE);
                
                if !ctx.input().pointer.any_down() {
                    self.actions.thought_drag = None;
                    self.create_node(pointer_pos.to_vec2(), thought.clone());
                }
            }
        }
    }
    
    fn draw(&mut self, ctx: &Context) {
        ctx.set_visuals(Visuals::dark());        
        
        SidePanel::right("Inbox").resizable(true).show(ctx, |ui| self.side_panel(ctx, ui));
        CentralPanel::default().show(ctx, |ui| self.central_panel(ctx, ui));
    }
    
}

impl App {
    fn node_physics(&mut self) {        
        let dummy = self.nodes.clone();
        for (key, node) in &mut self.nodes {
            for connection_key in node.connections.clone().iter() {
                let connection = self.connections.get(connection_key).unwrap();
                let other = dummy.get(&connection.other(key)).unwrap();
                
                node.connection(other);
            }
            
            for (dummy_key, dummy_node) in &dummy {
                if key == dummy_key { continue }
                
                node.repulse(dummy_node);
            }
            
            if key != self.selected.as_ref().unwrap_or(&NodeKey(0)) { node.tick(); } 
        }
    }
    
    fn remove_connection(&mut self, key: &ConnectionKey) { 
        self.connections.remove(key); 
        for node in self.nodes.values_mut() {
            node.connections.retain(|conn_key| conn_key != key);
        }
    }
    
    fn get_node(&self, key: &NodeKey) -> Option<&Node> { self.nodes.get(key) }
    
    fn get_node_mut(&mut self, key: &NodeKey) -> Option<&mut Node> { self.nodes.get_mut(key) }
    
    fn remove_node(&mut self, key: NodeKey) { 
        for connection_key in self.nodes.get(&key).unwrap().connections.clone() {
            self.remove_connection(&connection_key);
        } 
        self.nodes.remove(&key); 
    }
    
    fn get_node_at(&self, position: Vec2) -> Option<NodeKey> {
        let radius = 15.0;
        
        for (key, node) in &self.nodes {
            if (node.position - position).length() <= radius {
                return Some(key.clone());
            }
        }
        
        None
    }
    
    fn create_connection(&mut self, from: &NodeKey, to: &NodeKey) {
        self.connections.insert(self.new_connection_key.clone(), Connection { from: from.clone(), to: to.clone() });
        self.nodes.get_mut(from).unwrap().connections.push(self.new_connection_key.clone());
        self.nodes.get_mut(to).unwrap().connections.push(self.new_connection_key.clone());
        self.new_connection_key.0 += 1;
    }
    
    fn create_node(&mut self, position: Vec2, thought: Thought) -> NodeKey {
        self.nodes.insert(self.new_node_key.clone(), Node::new(position, thought));
        let key = self.new_node_key.clone();
        self.new_node_key.0 += 1;
        key
    }
}