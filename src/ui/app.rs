use eframe::egui;

use crate::{song::Song, ui};

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

        ui::menu_bar::build(ui);
        ui::bottom_pannel::build(self, ui, &songs);
        ui::left_panel::build(ui);
        ui::central_panel::build(ui, &songs);
    }
}
