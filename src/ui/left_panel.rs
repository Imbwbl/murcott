use eframe::egui;

pub fn build(ui: &mut egui::Ui) {
    egui::Panel::left("left_pannel")
            .resizable(false)
            .show(ui, |ui| {
                ui.heading("Library");
                ui.collapsing("Johann Sebastian Bach", |ui| {
                    ui.label("Aria");
                    ui.label("Tocata");
                });
            });
}