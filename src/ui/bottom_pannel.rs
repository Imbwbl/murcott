use eframe::egui;

use crate::{song::Song, ui::app::MurcottApp};

pub fn build(app: &mut MurcottApp, ui: &mut egui::Ui, songs: &[Song]) {
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
                                egui::ProgressBar::new(app.progress)
                                    .desired_height(5.0)
                                    .fill(egui::Color32::WHITE),
                            );
                            ui.label("3:29");

                            let response = progress_bar.interact(egui::Sense::click_and_drag()).on_hover_cursor(egui::CursorIcon::PointingHand);
                            if (response.clicked() || response.dragged())
                                && let Some(pos) = response.interact_pointer_pos()
                            {
                                app.progress =
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
                                    let knob_x = bar_rect.width().mul_add(app.progress, bar_rect.left());
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
                                egui::ProgressBar::new(app.volume)
                                    .desired_width(80.0)
                                    .desired_height(5.0)
                                    .fill(egui::Color32::WHITE),
                            );

                            let response = volume_bar.interact(egui::Sense::click_and_drag()).on_hover_cursor(egui::CursorIcon::PointingHand);

                            if (response.clicked() || response.dragged())
                                && let Some(pos) = response.interact_pointer_pos()
                            {
                                app.volume =
                                    ((pos.x - volume_bar.rect.left()) / volume_bar.rect.width()).clamp(0.0, 1.0);
                                // TODO: audio_sink.set_volume(app.volume);
                            }

                            // Optional: Scroll wheel adjustment
                            if response.hovered() {
                                let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                                if scroll != 0.0 {
                                    app.volume = scroll.mul_add(0.002, app.volume).clamp(0.0, 1.0);
                                    // TODO: audio_sink.set_volume(app.volume);
                                }
                            }

                            if ui.is_rect_visible(volume_bar.rect) {
                                let bar_rect = egui::Rect::from_center_size(
                                    volume_bar.rect.center(),
                                    egui::vec2(volume_bar.rect.width(), 5.0),
                                );
                                let painter = ui.painter();

                                if response.hovered() || response.dragged() {
                                    let knob_x = bar_rect.width().mul_add(app.volume, bar_rect.left());
                                    painter.circle_filled(
                                        egui::pos2(knob_x, bar_rect.center().y),
                                        6.0,
                                        egui::Color32::WHITE,
                                    );
                                }
                            }

                            let icon = if app.volume == 0.0 {
                                "🔇"
                            } else if app.volume < 0.5 {
                                "🔉"
                            } else {
                                "🔊"
                            };
                            if ui.button(icon).clicked() {
                                if app.volume > 0.0 {
                                    app.prev_volume = app.volume;
                                    app.volume = 0.0;
                                } else {
                                    app.volume = if app.prev_volume > 0.0 {
                                        app.prev_volume
                                    } else {
                                        0.5
                                    };
                                }
                                // TODO: audio_sink.set_volume(app.volume);
                            }
                        },
                    );
                });
            });
}