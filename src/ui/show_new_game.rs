use crate::{game::modinfo, ui::i18n::I18n};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::*;

use super::MainMenuState;

pub fn show_new_game_system(
    mut ctx: EguiContexts,
    i18n: Res<I18n>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            if ui.button(i18n.content("back")).clicked() {
                next_menu_state.set(MainMenuState::Saves);
            }
            ui.collapsing(i18n.content("mods"), |ui| {
                let locales: Vec<String> = i18n
                    .localization
                    .locales()
                    .map(|it| it.to_string())
                    .collect();
                for modinfo in modinfo::get_all_modinfo_json() {
                    ui.label(modinfo.get_name(&locales))
                        .on_hover_text(modinfo.get_description(&locales));
                }
            });
            if ui.button(i18n.content("start")).clicked() {}
        });
    });
}
