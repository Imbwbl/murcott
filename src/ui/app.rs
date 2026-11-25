use eframe::egui;

use crate::{song, ui};

#[derive(Default)]
pub struct MurcottApp {
    pub progress: f32,
    pub prev_volume: f32,
    pub volume: f32,
}

impl MurcottApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MurcottApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        let songs = song::get_songs(std::path::Path::new("C:\\Users\\pi72mlj\\Music"));
        
        ui::menu_bar::build(ui);
        ui::bottom_pannel::build(self, ui, songs.clone());
        ui::left_panel::build(ui);
        ui::central_panel::build(ui, songs);
    }
}
