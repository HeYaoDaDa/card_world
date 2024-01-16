use crate::{
    game::generate_world::{self, PerlinNoiseParams},
    ui::i18n::I18n,
};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::*;

use super::TestsState;

pub fn show_tests_generate_world_system(
    mut ctx: EguiContexts,
    i18n: Res<I18n>,
    mut next_menu_state: ResMut<NextState<TestsState>>,
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
                    weights: vec![1, 1, 1, 1, 1, 1, 1],
                };
                let map =
                    generate_world::generate_elevation_map_with_perlin_noise(None, 100, 50, params)
                        .unwrap();
                for column in &map {
                    for item in column.iter().rev() {
                        if *item >= 0 {
                            print!("+{}", item);
                        } else {
                            print!("{}", item);
                        }
                    }
                    println!();
                }
            }
        });
    });
}
