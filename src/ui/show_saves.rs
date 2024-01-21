use crate::ui::i18n::I18n;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::*;

use super::MainMenuState;

pub fn show_saves_system(
    mut ctx: EguiContexts,
    i18n: Res<I18n>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            if ui.button(i18n.content("core_cmn-btn-back")).clicked() {
                next_menu_state.set(MainMenuState::MainMenu);
            }
            if ui.button(i18n.content("core_saves_btn_new_game")).clicked() {
                next_menu_state.set(MainMenuState::NewGame);
            }
            //todo saves
        });
    });
}
