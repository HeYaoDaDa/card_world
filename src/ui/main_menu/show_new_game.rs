use crate::MainMenuState;
use crate::{
    game::modinfo::{self, Modinfo},
    ui::i18n::I18n,
};
use bevy::prelude::*;
use bevy_fluent::Locale;
use bevy_inspector_egui::bevy_egui::*;
use std::ops::{Deref, DerefMut};

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
                *modinfos.deref_mut() = modinfo::get_all_modinfo_json();
            };
            if ui.button(i18n.content("start")).clicked() {};
            if ui.button(i18n.content("back")).clicked() {
                next_menu_state.set(MainMenuState::MainMenu);
            }
        });
    if !modinfos.is_empty() {
        egui::Window::new(i18n.content("mods")).show(ui.ctx_mut(), |ui| {
            for modinfo in modinfos.deref() {
                ui.label(modinfo.get_name(&locale.requested.to_string()))
                    .on_hover_text(modinfo.get_description(&locale.requested.to_string()));
            }
            if ui.button(i18n.content("back")).clicked() {
                modinfos.clear();
            }
        });
    }
}
