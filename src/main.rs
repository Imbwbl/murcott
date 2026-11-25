use eframe::egui;

mod ui;
mod song;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::Vec2::new(1080.0, 540.0)),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "murcott",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::app::MurcottApp::new(cc)))),
    );
}

