use super::NewGameOptions;
use crate::game::modinfo::{self};
use crate::ui::i18n::I18n;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::*;
use bevy_inspector_egui::bevy_egui::*;

pub fn show_new_game_system(
    ui: &mut EguiContexts,
    i18n: &Res<I18n>,
    new_game_options: &mut NewGameOptions,
) {
    egui::Window::new(i18n.content("new-game"))
        .id(Id::new("new_game"))
        .collapsible(false)
        .resizable(false)
        .open(&mut new_game_options.new_game_open)
        .show(ui.ctx_mut(), |ui| {
            if ui.button(i18n.content("mods")).clicked() {
                new_game_options.modinfos = modinfo::get_all_modinfo_json();
                new_game_options.mods_open = true;
            };
            if ui.button(i18n.content("start")).clicked() {};
        });

    egui::Window::new(i18n.content("mods"))
        .id(Id::new("new_game_mods"))
        .collapsible(false)
        .resizable(false)
        .open(&mut new_game_options.mods_open)
        .show(ui.ctx_mut(), |ui| {
            let locales: Vec<String> = i18n
                .localization
                .locales()
                .map(|it| it.to_string())
                .collect();
            for modinfo in &new_game_options.modinfos {
                ui.label(modinfo.get_name(&locales))
                    .on_hover_text(modinfo.get_description(&locales));
            }
        });
    if new_game_options.mods_open && !new_game_options.new_game_open {
        new_game_options.mods_open = false;
    }
}
