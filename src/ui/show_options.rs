use crate::game::options::{Options, OptionsChangeEvent};
use crate::ui::i18n::I18n;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_inspector_egui::bevy_egui::egui::*;
use bevy_inspector_egui::bevy_egui::*;
use std::collections::BTreeMap;

use super::MainMenuState;

const LANGUAGES: [(&'static str, &'static str); 3] = [
    ("en", "English"),
    ("zh-Hans", "中文简体"),
    ("zh-Hant", "中文繁体"),
];

pub fn show_options_system(
    mut ctx: EguiContexts,
    i18n: Res<I18n>,
    mut window_query: Query<&mut bevy::window::Window>,
    mut options: ResMut<Options>,
    mut options_change_event: EventWriter<OptionsChangeEvent>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    let language_map: BTreeMap<_, _> = LANGUAGES.into_iter().collect();
    let mut window = window_query.single_mut();

    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            if ui.button(i18n.content("core_cmn-btn-back")).clicked() {
                next_menu_state.set(MainMenuState::MainMenu);
            }

            let item_width = (ui.available_width() - ui.spacing().item_spacing.x) / 2.;

            options_row(ui, item_width, i18n.content("core_options-label-language"), |ui| {
                egui::ComboBox::from_id_source("language_combo_box")
                    .selected_text(language_map.get(&options.language[..]).unwrap().to_string())
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for (code, language) in language_map.iter() {
                            ui.set_width(ui.available_width());
                            ui.selectable_value(
                                &mut options.language,
                                code.to_string(),
                                language.to_string(),
                            );
                        }
                    });
            });

            options_row(ui, item_width, i18n.content("core_options-label-v-sync"), |ui| {
                if ui.checkbox(&mut options.v_sync, "").clicked() {
                    window.present_mode = if options.v_sync {
                        PresentMode::AutoVsync
                    } else {
                        PresentMode::AutoNoVsync
                    };
                    options.v_sync = matches!(window.present_mode, PresentMode::AutoVsync);
                    options_change_event.send_default();
                }
            });

            options_row(ui, item_width, i18n.content("core_options-label-show-fps"), |ui| {
                let old_show_fps = options.show_fps;
                if ui.checkbox(&mut options.show_fps, "").clicked() {
                    options.show_fps = !old_show_fps;
                    options_change_event.send_default();
                }
            });
        });
    });
}

fn options_row<R>(
    ui: &mut Ui,
    item_width: f32,
    label: String,
    add_contents: impl FnOnce(&mut Ui) -> R,
) {
    ui.horizontal(|ui| {
        ui.scope(|ui| {
            ui.set_width(item_width);
            ui.label(label);
        });
        ui.scope(|ui| {
            ui.set_width(item_width);
            add_contents(ui);
        });
    });
}
