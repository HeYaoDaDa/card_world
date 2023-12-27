use std::collections::BTreeMap;

use crate::game::options::{Options, OptionsChangeEvent};
use crate::ui::i18n::I18n;
use crate::MainMenuState;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_fluent::Locale;
use bevy_inspector_egui::bevy_egui::*;

pub fn show_options_system(
    mut ui: EguiContexts,
    i18n: Res<I18n>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut window_query: Query<&mut Window>,
    mut cur_locale: ResMut<Locale>,
    mut options: ResMut<Options>,
    mut options_change_event: EventWriter<OptionsChangeEvent>,
) {
    let mut window = window_query.single_mut();

    let language_map: BTreeMap<_, _> = vec![
        ("en-US".to_string(), "English".to_string()),
        ("zh-CN".to_string(), "中文简体".to_string()),
        ("zh-TW".to_string(), "中文繁体".to_string()),
    ]
    .into_iter()
    .collect();

    egui::Area::new("show_options")
        .movable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
        .show(ui.ctx_mut(), |ui| {
            egui::ComboBox::from_label(i18n.content("language"))
                .selected_text(language_map.get(&options.language).unwrap())
                .show_ui(ui, |ui| {
                    for (code, language) in language_map.iter() {
                        if ui
                            .selectable_value(
                                &mut cur_locale.requested,
                                code.parse().unwrap(),
                                language,
                            )
                            .changed()
                        {
                            next_menu_state.set(MainMenuState::Loading);
                            options.language = cur_locale.requested.to_string();
                            options_change_event.send_default();
                        }
                    }
                });
            if ui
                .checkbox(&mut options.v_sync, i18n.content("v-sync"))
                .clicked()
            {
                window.present_mode = if options.v_sync {
                    PresentMode::AutoVsync
                } else {
                    PresentMode::AutoNoVsync
                };
                options.v_sync = matches!(window.present_mode, PresentMode::AutoVsync);
                options_change_event.send_default();
            }
            let old_show_fps = options.show_fps;
            if ui
                .checkbox(&mut options.show_fps, i18n.content("show-fps"))
                .clicked()
            {
                options.show_fps = !old_show_fps;
                info!("current show_fps is {:?}", options.show_fps);
                options_change_event.send_default();
            }
            if ui.button(i18n.content("back")).clicked() {
                next_menu_state.set(MainMenuState::MainMenu);
            }
        });
}
