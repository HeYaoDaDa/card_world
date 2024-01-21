use crate::ui::i18n::I18n;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::*;
use bevy_inspector_egui::bevy_egui::*;

use super::MainMenuState;

pub fn show_main_menu_system(
    mut ui: EguiContexts,
    i18n: Res<I18n>,
    mut exit: EventWriter<AppExit>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    CentralPanel::default().show(ui.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.label(RichText::new(i18n.content("core_main-menu-title")).size(50.0));
        });

        ui.vertical_centered_justified(|ui| {
            ui.set_width(400.0);

            if ui
                .button(RichText::new(i18n.content("core_main-menu-btn-start")).size(30.0))
                .clicked()
            {
                next_menu_state.set(MainMenuState::Saves);
            };

            if ui
                .button(RichText::new(i18n.content("core_main-menu-btn-options")).size(30.0))
                .clicked()
            {
                next_menu_state.set(MainMenuState::Options);
            };

            if ui.button(RichText::new(i18n.content("core_main-menu-btn-tests")).size(30.0)).clicked() {
                next_menu_state.set(MainMenuState::Tests);
            };

            if ui
                .button(RichText::new(i18n.content("core_main-menu-btn-exit")).size(30.0))
                .clicked()
            {
                exit.send(AppExit);
            };
        });
    });
}
