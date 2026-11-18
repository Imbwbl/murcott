use eframe::egui;
use egui_extras::Column;

struct Song {
    track: u32,
    title: &'static str,
    artist: &'static str,
    album: &'static str,
    length: &'static str,
    filename: &'static str,
}

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::Vec2::new(1080.0, 540.0)),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "murcott",
        native_options,
        Box::new(|cc| Ok(Box::new(MurcottApp::new(cc)))),
    );
}

#[derive(Default)]
struct MurcottApp {
    progress: f32,
    prev_volume: f32,
    volume: f32,
}

impl MurcottApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MurcottApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let headers = ["Track", "Title", "Artist", "Album", "Length", "Filename"];

        let songs = [
            Song {
                track: 1,
                title: "Prélude",
                artist: "Claude Debussy",
                album: "Suite bergamasque",
                length: "4:10",
                filename: "01 Prélude.ogg",
            },
            Song {
                track: 2,
                title: "Menuet",
                artist: "Claude Debussy",
                album: "Suite bergamasque",
                length: "4:50",
                filename: "02 Menuet.ogg",
            },
            Song {
                track: 3,
                title: "Clair de lune",
                artist: "Claude Debussy",
                album: "Suite bergamasque",
                length: "5:00",
                filename: "03 Clair de lune.ogg",
            },
            Song {
                track: 4,
                title: "Passepied",
                artist: "Claude Debussy",
                album: "Suite bergamasque",
                length: "3:50",
                filename: "04 Passepied.ogg",
            },
            Song {
                track: 1,
                title: "Gymnopédie No. 1",
                artist: "Erik Satie",
                album: "Gymnopédies",
                length: "3:20",
                filename: "01 Gymnopédie No. 1.ogg",
            },
            Song {
                track: 2,
                title: "Gymnopédie No. 2",
                artist: "Erik Satie",
                album: "Gymnopédies",
                length: "2:40",
                filename: "02 Gymnopédie No. 2.ogg",
            },
            Song {
                track: 3,
                title: "Gymnopédie No. 3",
                artist: "Erik Satie",
                album: "Gymnopédies",
                length: "2:30",
                filename: "03 Gymnopédie No. 3.ogg",
            },
            Song {
                track: 1,
                title: "Nocturne in B-flat minor, Op. 9 No. 1",
                artist: "Frédéric Chopin",
                album: "Nocturnes, Op. 9",
                length: "5:30",
                filename: "01 Nocturne in B-flat minor.ogg",
            },
            Song {
                track: 2,
                title: "Nocturne in E-flat major, Op. 9 No. 2",
                artist: "Frédéric Chopin",
                album: "Nocturnes, Op. 9",
                length: "4:30",
                filename: "02 Nocturne in E-flat major.ogg",
            },
            Song {
                track: 3,
                title: "Nocturne in B major, Op. 9 No. 3",
                artist: "Frédéric Chopin",
                album: "Nocturnes, Op. 9",
                length: "6:40",
                filename: "03 Nocturne in B major.ogg",
            },
            Song {
                track: 1,
                title: "Aria",
                artist: "Johann Sebastian Bach",
                album: "Goldberg Variations",
                length: "4:00",
                filename: "01 Aria.ogg",
            },
            Song {
                track: 2,
                title: "Variatio 1",
                artist: "Johann Sebastian Bach",
                album: "Goldberg Variations",
                length: "1:50",
                filename: "02 Variatio 1.ogg",
            },
            Song {
                track: 3,
                title: "Variatio 2",
                artist: "Johann Sebastian Bach",
                album: "Goldberg Variations",
                length: "1:40",
                filename: "03 Variatio 2.ogg",
            },
        ];

        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Settings").clicked() {

                }
                if ui.button("Download musics").clicked() {
                    
                }
                if ui.button("Quit").clicked() {
                    ui.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });

        egui::Panel::bottom("bottom_pannel")
            .resizable(true)
            .show(ui, |ui| {
                ui.columns(3, |columns| {
                    columns[0].vertical(|ui| {
                        ui.strong(songs[0].title);
                        ui.label(songs[0].artist);
                    });

                    /*columns[1].vertical_centered(|ui| {
                        ui.horizontal(|ui| {
                            let _ = ui.button("shuffle");
                            let _ = ui.button("backward");
                            let _ = ui.button("play");
                            let _ = ui.button("forward");
                            let _ = ui.button("loop");
                        });
                    });
                    columns[1].vertical_centered( |ui| {
                        ui.horizontal(|ui| {
                            ui.label("2:07");
                            ui.add(
                                egui::ProgressBar::new(0.5)
                                    //.desired_width(100.0)
                                    .desired_height(5.0)
                                    .fill(egui::Color32::WHITE),
                            );
                            ui.label("3:29");
                    }   );
                    });*/

                    columns[1].vertical_centered(|ui| {
                        let button_size = egui::vec2(28.0, 24.0);
                        let total_buttons_w = (5.0_f32 - 1.0_f32)
                            .mul_add(ui.spacing().item_spacing.x, 5.0 * button_size.x);

                        ui.allocate_ui_with_layout(
                            egui::vec2(total_buttons_w, button_size.y),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                let _ = ui.add_sized(button_size, egui::Button::new("🔀"));
                                let _ = ui.add_sized(button_size, egui::Button::new("⏮"));
                                let _ = ui.add_sized(button_size, egui::Button::new("⏯"));
                                let _ = ui.add_sized(button_size, egui::Button::new("⏭"));
                                let _ = ui.add_sized(button_size, egui::Button::new("🔁"));
                            },
                        );

                        ui.horizontal(|ui| {
                            ui.label("2:07");
                            let progress_bar = ui.add(
                                egui::ProgressBar::new(self.progress)
                                    .desired_height(5.0)
                                    .fill(egui::Color32::WHITE),
                            );
                            ui.label("3:29");

                            let response = progress_bar.interact(egui::Sense::click_and_drag()).on_hover_cursor(egui::CursorIcon::PointingHand);
                            if (response.clicked() || response.dragged())
                                && let Some(pos) = response.interact_pointer_pos()
                            {
                                self.progress =
                                    ((pos.x - progress_bar.rect.left()) / progress_bar.rect.width()).clamp(0.0, 1.0);
                                // TODO: set_sound_to();
                            }

                            if ui.is_rect_visible(progress_bar.rect) {
                                let bar_rect = egui::Rect::from_center_size(
                                    progress_bar.rect.center(),
                                    egui::vec2(progress_bar.rect.width(), 5.0),
                                );
                                let painter = ui.painter();

                                

                                if response.hovered() || response.dragged() {
                                    let knob_x = bar_rect.width().mul_add(self.progress, bar_rect.left());
                                    painter.circle_filled(
                                        egui::pos2(knob_x, bar_rect.center().y),
                                        6.0,
                                        egui::Color32::WHITE,
                                    );
                                }
                            }

                        });
                    });

                    columns[2].with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            let volume_bar = ui.add(
                                egui::ProgressBar::new(self.volume)
                                    .desired_width(80.0)
                                    .desired_height(5.0)
                                    .fill(egui::Color32::WHITE),
                            );

                            let response = volume_bar.interact(egui::Sense::click_and_drag()).on_hover_cursor(egui::CursorIcon::PointingHand);

                            if (response.clicked() || response.dragged())
                                && let Some(pos) = response.interact_pointer_pos()
                            {
                                self.volume =
                                    ((pos.x - volume_bar.rect.left()) / volume_bar.rect.width()).clamp(0.0, 1.0);
                                // TODO: audio_sink.set_volume(self.volume);
                            }

                            // Optional: Scroll wheel adjustment
                            if response.hovered() {
                                let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                                if scroll != 0.0 {
                                    self.volume = scroll.mul_add(0.002, self.volume).clamp(0.0, 1.0);
                                    // TODO: audio_sink.set_volume(self.volume);
                                }
                            }

                            if ui.is_rect_visible(volume_bar.rect) {
                                let bar_rect = egui::Rect::from_center_size(
                                    volume_bar.rect.center(),
                                    egui::vec2(volume_bar.rect.width(), 5.0),
                                );
                                let painter = ui.painter();

                                if response.hovered() || response.dragged() {
                                    let knob_x = bar_rect.width().mul_add(self.volume, bar_rect.left());
                                    painter.circle_filled(
                                        egui::pos2(knob_x, bar_rect.center().y),
                                        6.0,
                                        egui::Color32::WHITE,
                                    );
                                }
                            }

                            let icon = if self.volume == 0.0 {
                                "🔇"
                            } else if self.volume < 0.5 {
                                "🔉"
                            } else {
                                "🔊"
                            };
                            if ui.button(icon).clicked() {
                                if self.volume > 0.0 {
                                    self.prev_volume = self.volume;
                                    self.volume = 0.0;
                                } else {
                                    self.volume = if self.prev_volume > 0.0 {
                                        self.prev_volume
                                    } else {
                                        0.5
                                    };
                                }
                                // TODO: audio_sink.set_volume(self.volume);
                            }
                        },
                    );
                });
            });

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
                .column(Column::auto().resizable(true))
                .header(20.0, |mut header| {
                    for header_name in headers {
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
}
