use crate::ui::i18n::I18n;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::*;
use bevy_inspector_egui::bevy_egui::*;
use bevy_inspector_egui::egui::Vec2;

use super::MainMenuState;

pub fn show_main_menu_system(
    mut ui: EguiContexts,
    i18n: Res<I18n>,
    mut exit: EventWriter<AppExit>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    Area::new("main_menu")
        .movable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::default())
        .show(ui.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(RichText::new(i18n.content("card-world")).size(50.0));
            });

            ui.vertical_centered(|ui| {
                ui.set_min_width(100.0);
                ui.set_max_width(400.0);
                ui.vertical_centered_justified(|ui| {
                    if ui
                        .add(egui::Button::new(
                            RichText::new(i18n.content("start-new-game")).size(30.0),
                        ))
                        .clicked()
                    {
                        next_menu_state.set(MainMenuState::Saves);
                    };

                    if ui
                        .add(egui::Button::new(
                            RichText::new(i18n.content("options")).size(30.0),
                        ))
                        .clicked()
                    {
                        next_menu_state.set(MainMenuState::Options);
                    };

                    if ui
                        .add(egui::Button::new(
                            RichText::new(i18n.content("exit-game")).size(30.0),
                        ))
                        .clicked()
                    {
                        exit.send(AppExit);
                    };
                });
            });
        });
}
