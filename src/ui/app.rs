use eframe::egui;

use crate::{song, ui};

#[derive(Default)]
pub struct MurcottApp {
    pub songs: Vec<song::Song>,
    pub progress: f32,
    pub prev_volume: f32,
    pub volume: f32,
}

impl MurcottApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, songs: Vec<song::Song>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            songs: songs,
            ..Self::default()
        }
    }
}

impl eframe::App for MurcottApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame,) {

        
        ui::menu_bar::build(ui);
        ui::bottom_pannel::build(self, ui, self.songs.clone());
        ui::left_panel::build(ui);
        ui::central_panel::build(ui, self.songs.clone());
    }
}
