use super::MainMenuOptions;
use crate::game::options::{Options, OptionsChangeEvent};
use crate::ui::i18n::I18n;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_inspector_egui::bevy_egui::egui::*;
use bevy_inspector_egui::bevy_egui::*;
use std::collections::BTreeMap;

pub fn show_options_system(
    ui: &mut EguiContexts,
    i18n: &Res<I18n>,
    mut window_query: Query<&mut bevy::window::Window>,
    mut options: ResMut<Options>,
    mut options_change_event: EventWriter<OptionsChangeEvent>,
    main_menu_options: &mut MainMenuOptions,
) {
    let mut window = window_query.single_mut();

    let language_map: BTreeMap<_, _> = vec![
        ("en", "English"),
        ("zh-Hans", "中文简体"),
        ("zh-Hant", "中文繁体"),
    ]
    .into_iter()
    .collect();

    egui::Window::new(i18n.content("options"))
        .id(Id::new("options"))
        .collapsible(false)
        .resizable(false)
        .open(&mut main_menu_options.options_open)
        .show(ui.ctx_mut(), |ui| {
            egui::ComboBox::from_label(i18n.content("language"))
                .selected_text(language_map.get(&options.language[..]).unwrap().to_string())
                .show_ui(ui, |ui| {
                    for (code, language) in language_map.iter() {
                        ui.selectable_value(
                            &mut options.language,
                            code.to_string(),
                            language.to_string(),
                        );
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
        });
}
