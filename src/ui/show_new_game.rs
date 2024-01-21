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
            if ui.button(i18n.content("core_cmn-btn-back")).clicked() {
                next_menu_state.set(MainMenuState::Saves);
            }
            ui.collapsing(i18n.content("core_new-game-btn-mods"), |ui| {
                for modinfo in modinfo::get_all_modinfo_json() {
                    ui.label(i18n.content(&modinfo.name))
                        .on_hover_text(i18n.content(&modinfo.description));
                }
            });
            if ui.button(i18n.content("core_new-game-btn-start")).clicked() {}
        });
    });
}
