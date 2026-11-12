use anyhow::Result;
use eframe::egui::{self, Panel, UiKind::LeftPanel, panel, viewport};
use egui_extras::Column;


struct Song {
    track: u32,
    title: &'static str,
    artist: &'static str,
    album: &'static str,
    length: &'static str,
    filename: &'static str,
}

fn main() -> Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::Vec2::new(1080.0, 540.0)),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "murcott",
        native_options,
        Box::new(|cc| Ok(Box::new(MurcottApp::new(cc)))),
    );
    Ok(())
}

#[derive(Default)]
struct MurcottApp {}

impl MurcottApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MurcottApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {

        let headers = [
            "Track",
            "Title",
            "Artist",
            "Album",
            "Length",
            "Filename",
        ];

        let songs = [
            Song { track: 1, title: "Prélude", artist: "Claude Debussy", album: "Suite bergamasque", length: "4:10", filename: "01 Prélude.ogg" },
            Song { track: 2, title: "Menuet", artist: "Claude Debussy", album: "Suite bergamasque", length: "4:50", filename: "02 Menuet.ogg" },
            Song { track: 3, title: "Clair de lune", artist: "Claude Debussy", album: "Suite bergamasque", length: "5:00", filename: "03 Clair de lune.ogg" },
            Song { track: 4, title: "Passepied", artist: "Claude Debussy", album: "Suite bergamasque", length: "3:50", filename: "04 Passepied.ogg" },
            Song { track: 1, title: "Gymnopédie No. 1", artist: "Erik Satie", album: "Gymnopédies", length: "3:20", filename: "01 Gymnopédie No. 1.ogg" },
            Song { track: 2, title: "Gymnopédie No. 2", artist: "Erik Satie", album: "Gymnopédies", length: "2:40", filename: "02 Gymnopédie No. 2.ogg" },
            Song { track: 3, title: "Gymnopédie No. 3", artist: "Erik Satie", album: "Gymnopédies", length: "2:30", filename: "03 Gymnopédie No. 3.ogg" },
            Song { track: 1, title: "Nocturne in B-flat minor, Op. 9 No. 1", artist: "Frédéric Chopin", album: "Nocturnes, Op. 9", length: "5:30", filename: "01 Nocturne in B-flat minor.ogg" },
            Song { track: 2, title: "Nocturne in E-flat major, Op. 9 No. 2", artist: "Frédéric Chopin", album: "Nocturnes, Op. 9", length: "4:30", filename: "02 Nocturne in E-flat major.ogg" },
            Song { track: 3, title: "Nocturne in B major, Op. 9 No. 3", artist: "Frédéric Chopin", album: "Nocturnes, Op. 9", length: "6:40", filename: "03 Nocturne in B major.ogg" },
            Song { track: 1, title: "Aria", artist: "Johann Sebastian Bach", album: "Goldberg Variations", length: "4:00", filename: "01 Aria.ogg" },
            Song { track: 2, title: "Variatio 1", artist: "Johann Sebastian Bach", album: "Goldberg Variations", length: "1:50", filename: "02 Variatio 1.ogg" },
            Song { track: 3, title: "Variatio 2", artist: "Johann Sebastian Bach", album: "Goldberg Variations", length: "1:40", filename: "03 Variatio 2.ogg" },
        ];

        egui::Panel::bottom("bottom_pannel").show(ui, |ui| ui.label("bottom_pannel"));

            egui::Panel::left("left_pannel")
                .resizable(false)
                .show(ui, |ui| {
                    ui.heading("Library");
                    ui.collapsing("Johann Sebastian Bach", |ui| {
                        ui.label("Aria");
                        ui.label("Tocata");
                    });
                });


        egui::CentralPanel::default().show(ui, |ui| {
            
            egui_extras::TableBuilder::new(ui)
                .id_salt("songs_panel")
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::remainder().resizable(true))
                .header(20.0, |mut header| {
                    for header_name in headers {
                        if headers.last().unwrap().eq(&header_name) {
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    ui.heading(header_name);
                                });
                            });
                        } else {
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                    ui.heading(header_name);
                                });
                            });
                        }
                    }
                })
                .body(|mut body| {
                    body.rows(20.0, songs.len(), |mut row| {
                        let row_index = row.index();
                        let song = &songs[row_index];
                        
                        row.col(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(song.track.to_string());
                            });
                        });
                        row.col(|ui| {
                            ui.label(song.title);
                        });
                        row.col(|ui| {
                            ui.label(song.artist);
                        });
                        row.col(|ui| {
                            ui.label(song.album);
                        });
                        row.col(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(song.length);
                            });
                        });
                        row.col(|ui| {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.label(song.filename);
                            });
                        });
                    });
                });
            /*egui::Grid::new("grid").striped(true).show(ui, |ui| {
                ui.heading("Track");
                ui.heading("Title");
                ui.heading("Artist");
                ui.heading("Album");
                ui.heading("Length");
                ui.heading("File name");
                ui.end_row();

                ui.label("Second row, first column");
                ui.label("Second row, second column");
                ui.label("Second row, third column");
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Same");
                    ui.label("cell");
                });
                ui.label("Third row, second column");
                ui.end_row();
            });*/
        });
    }
}
