use crate::ui::i18n::I18n;
use crate::MainMenuState;
use bevy::prelude::*;
use bevy_fluent::Locale;
use bevy_inspector_egui::bevy_egui::*;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

pub fn show_new_game_system(
    mut ui: EguiContexts,
    i18n: Res<I18n>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut modinfos: Local<Vec<Modinfo>>,
    locale: Res<Locale>,
) {
    egui::Area::new("show_options")
        .movable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
        .show(ui.ctx_mut(), |ui| {
            if ui.button(i18n.content("mods")).clicked() {
                *modinfos.deref_mut() = get_all_modinfo_json();
            };
            if ui.button(i18n.content("start")).clicked() {};
            if ui.button(i18n.content("back")).clicked() {
                next_menu_state.set(MainMenuState::MainMenu);
            }
        });
    if !modinfos.is_empty() {
        egui::Window::new(i18n.content("mods")).show(ui.ctx_mut(), |ui| {
            for modinfo in modinfos.deref() {
                ui.label(modinfo.get_name(&locale.requested.to_string()));
            }
            if ui.button(i18n.content("back")).clicked() {
                modinfos.clear();
            }
        });
    }
}

#[derive(Default, Deserialize)]
pub struct Modinfo {
    pub id: String,
    pub name: HashMap<String, String>,
    pub description: HashMap<String, String>,
    pub authors: Vec<String>,
    pub maintainers: Vec<String>,
    pub version: String,
    pub dependencies: Vec<String>,
    pub obsolete: bool,
    #[serde(skip)]
    pub path: String,
}

impl Modinfo {
    fn get_name(&self, locale: &str) -> &String {
        if self.name.is_empty() {
            warn!("mod {} miss name", self.id);
            &self.id
        } else {
            self.name
                .get(locale)
                .unwrap_or_else(|| self.name.get("en-US").unwrap())
        }
    }

    fn get_description(&self, locale: &str) -> &String {
        if self.description.is_empty() {
            warn!("mod {} miss description", self.id);
            &self.id
        } else {
            self.description
                .get(locale)
                .unwrap_or_else(|| self.description.get("en-US").unwrap())
        }
    }
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
