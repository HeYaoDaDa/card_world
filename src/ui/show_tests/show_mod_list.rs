use crate::{
    game::modinfo::{self, Modinfo},
    ui::i18n::I18n,
};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::*;

use super::TestsState;

pub fn show_mod_list_system(
    mut ctx: EguiContexts,
    i18n: Res<I18n>,
    mut next_menu_state: ResMut<NextState<TestsState>>,
    mut modinfos: Local<Option<Vec<Modinfo>>>,
) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            if ui.button(i18n.content("back")).clicked() {
                next_menu_state.set(TestsState::Tests);
            }
            if modinfos.is_none() {
                *modinfos = Some(modinfo::get_all_modinfo_json());
            }
            ui.vertical_centered(|ui| {
                let locales: Vec<String> = i18n
                    .localization
                    .locales()
                    .map(|it| it.to_string())
                    .collect();
                if let Some(modinfos) = modinfos.as_deref() {
                    for modinfo in modinfos {
                        ui.label(modinfo.get_name(&locales))
                            .on_hover_text(modinfo.get_description(&locales));
                    }
                }
            });
        });
    });
}
