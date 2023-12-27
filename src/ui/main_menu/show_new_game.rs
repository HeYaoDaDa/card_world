use crate::ui::i18n::I18n;
use crate::MainMenuState;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::*;
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

pub fn show_new_game_system(
    mut ui: EguiContexts,
    i18n: Res<I18n>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    egui::Area::new("show_options")
        .movable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
        .show(ui.ctx_mut(), |ui| {
            if ui.button(i18n.content("mods")).clicked() {
                for modinfo in get_all_modinfo_json() {
                    info!("{}:{}", modinfo.id, modinfo.path);
                }
            };
            if ui.button(i18n.content("start")).clicked() {};
            if ui.button(i18n.content("back")).clicked() {
                next_menu_state.set(MainMenuState::MainMenu);
            }
        });
}

#[derive(Default, Deserialize)]
pub struct Modinfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub maintainers: Vec<String>,
    pub version: String,
    pub dependencies: Vec<String>,
    pub obsolete: bool,
    #[serde(skip)]
    pub path: String,
}

pub fn get_all_modinfo_json() -> Vec<Modinfo> {
    let paths = get_all_modinfo_path_by_path("assets/mods");
    let mut modinfos = Vec::new();
    for path in paths {
        let mut modinfo_file =
            File::open(&path).expect(&format!("Open {} fail", path.to_string_lossy()));
        let mut modinfo_json = String::new();
        modinfo_file
            .read_to_string(&mut modinfo_json)
            .expect(&format!("Read {} fail", path.to_string_lossy()));
        let mut modinfo: Modinfo = serde_json::from_str(&modinfo_json).expect(&format!(
            "JSONify {} to Modinfo fail",
            path.to_string_lossy()
        ));
        modinfo.path = path.parent().unwrap_or(&path).to_string_lossy().into();
        modinfos.push(modinfo);
    }
    modinfos
}

fn get_all_modinfo_path_by_path(path: &str) -> Vec<PathBuf> {
    if let Ok(entries) = fs::read_dir(path) {
        let mut dirs = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() && entry.file_name() == "modinfo.json" {
                        return vec![entry.path()];
                    } else if metadata.is_dir() {
                        dirs.push(entry);
                    }
                }
            }
        }
        let mut result = Vec::new();
        for entry in dirs {
            let paths = get_all_modinfo_path_by_path(entry.path().to_str().unwrap());
            if !paths.is_empty() {
                result.extend(paths);
            }
        }
        return result;
    }
    Vec::new()
}
