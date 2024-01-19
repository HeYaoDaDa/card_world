use crate::{
    game::generate_world::{self, PerlinNoiseParams},
    ui::i18n::I18n,
};
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::*,
    egui::{Color32, RichText},
};

use super::TestsState;

pub fn show_tests_generate_world_system(
    mut ctx: EguiContexts,
    i18n: Res<I18n>,
    mut next_menu_state: ResMut<NextState<TestsState>>,
    mut map: Local<Vec<Vec<i8>>>,
) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            if ui.button(i18n.content("back")).clicked() {
                next_menu_state.set(TestsState::Tests);
            }
            if ui.button("start").clicked() {
                let params = PerlinNoiseParams {
                    scale: 10.,
                    max: 3,
                    min: -3,
                    weights: vec![1, 2, 2, 2, 2, 2, 1],
                };
                *map =
                    generate_world::generate_elevation_map_with_perlin_noise(None, 60, 50, params)
                        .unwrap();
            }
            if !map.is_empty() {
                ui.horizontal(|ui| {
                    for column in &map {
                        ui.vertical(|ui| {
                            for item in column.iter().rev() {
                                if *item >= 0 {
                                    ui.label(
                                        RichText::new(item.to_string()).color(Color32::from_rgb(
                                            0,
                                            item.abs() as u8 * 25,
                                            0,
                                        )),
                                    );
                                } else {
                                    ui.label(
                                        RichText::new(item.abs().to_string())
                                            .color(Color32::from_rgb(0, 0, item.abs() as u8 * 25)),
                                    );
                                }
                            }
                        });
                    }
                });
            }
        });
    });
}
