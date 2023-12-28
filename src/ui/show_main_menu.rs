use crate::game::modinfo::Modinfo;
use crate::game::options::{Options, OptionsChangeEvent};
use crate::ui::i18n::I18n;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::*;
use bevy_inspector_egui::bevy_egui::*;

mod show_new_game;
mod show_options;

struct NewGameOptions {
    new_game_open: bool,
    mods_open: bool,
    modinfos: Vec<Modinfo>,
}

impl Default for NewGameOptions {
    fn default() -> Self {
        Self {
            new_game_open: false,
            mods_open: false,
            modinfos: Default::default(),
        }
    }
}

pub struct MainMenuOptions {
    options_open: bool,

    new_game_options: NewGameOptions,
}

impl Default for MainMenuOptions {
    fn default() -> Self {
        Self {
            options_open: false,
            new_game_options: Default::default(),
        }
    }
}

pub fn show_main_menu_system(
    mut ui: EguiContexts,
    i18n: Res<I18n>,
    mut exit: EventWriter<AppExit>,
    window_query: Query<&mut bevy::window::Window>,
    options: ResMut<Options>,
    options_change_event: EventWriter<OptionsChangeEvent>,
    mut main_menu_options: Local<MainMenuOptions>,
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
                        RichText::new(i18n.content("start-new-game"))
                            .color(Color32::WHITE)
                            .size(30.0),
                    ))
                    .clicked()
                {
                    main_menu_options.new_game_options.new_game_open =
                        !main_menu_options.new_game_options.new_game_open;
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
                    main_menu_options.options_open = !main_menu_options.options_open;
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
    show_options::show_options_system(
        &mut ui,
        &i18n,
        window_query,
        options,
        options_change_event,
        &mut main_menu_options,
    );
    show_new_game::show_new_game_system(&mut ui, &i18n, &mut main_menu_options.new_game_options);
}
