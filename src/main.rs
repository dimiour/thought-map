mod node;
mod thought;
mod connection;
mod action;
mod app;

fn main() {   
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 768.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Thought Map",
        options,
        Box::new(|_cc| Box::new(app::App::default())),
    );
}