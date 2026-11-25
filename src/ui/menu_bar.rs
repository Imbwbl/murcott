use eframe::egui;

pub fn build(ui: &mut egui::Ui) {
    egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Settings").clicked() {

                }
                if ui.button("Download musics").clicked() {
                    
                }
                if ui.button("Exit").clicked() {
                    ui.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
}