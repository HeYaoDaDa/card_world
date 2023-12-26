use crate::ui::i18n::I18n;
use crate::MainMenuState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::*;
use bevy_inspector_egui::bevy_egui::*;

pub fn show_main_menu_system(
    mut ui: EguiContexts,
    i18n: Res<I18n>,
    mut exit: EventWriter<AppExit>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    CentralPanel::default().show(ui.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(100.0);

            ui.set_min_size(egui::Vec2::new(0.0, 100.0));

            ui.label(
                RichText::new(i18n.content("card-world"))
                    .heading()
                    .color(Color32::WHITE)
                    .size(50.0),
            );
        });

        ui.vertical_centered(|ui| {
            ui.set_min_width(100.0);
            ui.set_max_width(400.0);
            ui.vertical_centered_justified(|ui| {
                if ui
                    .add(egui::Button::new(
                        RichText::new(i18n.content("start-game"))
                            .color(Color32::WHITE)
                            .size(30.0),
                    ))
                    .clicked()
                {
                    next_menu_state.set(MainMenuState::NewGame);
                };

                ui.add_space(50.0);

                if ui
                    .add(egui::Button::new(
                        RichText::new(i18n.content("options"))
                            .color(Color32::WHITE)
                            .size(30.0),
                    ))
                    .clicked()
                {
                    next_menu_state.set(MainMenuState::Options);
                };

                ui.add_space(50.0);

                if ui
                    .add(egui::Button::new(
                        RichText::new(i18n.content("exit-game"))
                            .color(Color32::WHITE)
                            .size(30.0),
                    ))
                    .clicked()
                {
                    exit.send(AppExit);
                };
            });
        });
    });
}
