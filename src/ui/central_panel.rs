use eframe::egui;
use egui_extras::Column;

use crate::song::Song;

const HEADERS: [&str; 6] = ["Track", "Title", "Artist", "Album", "Length", "Filename"];

pub fn build(ui: &mut egui::Ui, songs: &[Song]) {
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
            .column(Column::auto().resizable(true))
            .header(20.0, |mut header| {
                for header_name in HEADERS {
                    header.col(|ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading(header_name);
                        });
                    });
                }
            })
            .body(|body| {
                body.rows(20.0, songs.len(), |mut row| {
                    let row_index = row.index();
                    let song = &songs[row_index];

                    row.col(|ui| {
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                ui.label(song.track.to_string());
                            },
                        );
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
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                ui.label(song.length);
                            },
                        );
                    });
                    row.col(|ui| {
                        ui.label(song.filename);
                    });
                });
            });
    });
}