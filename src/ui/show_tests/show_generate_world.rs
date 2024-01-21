use crate::{game::generate_world, ui::i18n::I18n};
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::*,
    egui::{vec2, Color32},
};

use super::TestsState;

pub struct PerlinNoiseParamsUiState {
    pub seed: Option<u32>,
    pub width: u8,
    pub height: u8,
    pub scale: f64,
    pub min: i8,
    pub max: i8,
    pub weights: Vec<u8>,
    pub result: Option<Vec<Vec<i8>>>,
}

impl Default for PerlinNoiseParamsUiState {
    fn default() -> Self {
        Self {
            seed: None,
            width: 50,
            height: 50,
            scale: 10.,
            min: -3,
            max: 3,
            weights: vec![10, 130, 70, 15, 40, 20, 10],
            result: None,
        }
    }
}

pub fn show_generate_world_system(
    mut ctx: EguiContexts,
    i18n: Res<I18n>,
    mut next_menu_state: ResMut<NextState<TestsState>>,
    mut ui_state: Local<PerlinNoiseParamsUiState>,
) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            if ui.button(i18n.content("core_cmn-btn-back")).clicked() {
                next_menu_state.set(TestsState::Tests);
            }
            ui.horizontal(|ui| {
                ui.label("seed");
                let mut seed_input = if let Some(seed) = ui_state.seed {
                    seed.to_string()
                } else {
                    "".to_string()
                };
                ui.text_edit_singleline(&mut seed_input);
                if let Ok(seed_input) = seed_input.parse() {
                    ui_state.seed = Some(seed_input);
                } else {
                    ui_state.seed = None;
                }
            });
            ui.horizontal(|ui| {
                ui.label("width");
                let mut input = ui_state.width.to_string();
                ui.text_edit_singleline(&mut input);
                if let Ok(input) = input.parse() {
                    ui_state.width = input;
                }
            });
            ui.horizontal(|ui| {
                ui.label("height");
                let mut input = ui_state.height.to_string();
                ui.text_edit_singleline(&mut input);
                if let Ok(input) = input.parse() {
                    ui_state.height = input;
                }
            });
            ui.horizontal(|ui| {
                ui.label("scale");
                let mut input = ui_state.scale.to_string();
                ui.text_edit_singleline(&mut input);
                if let Ok(input) = input.parse() {
                    ui_state.scale = input;
                }
            });
            ui.horizontal(|ui| {
                ui.label("min");
                let mut input = ui_state.min.to_string();
                ui.text_edit_singleline(&mut input);
                if let Ok(input) = input.parse() {
                    ui_state.min = input;
                }
            });
            ui.horizontal(|ui| {
                ui.label("max");
                let mut input = ui_state.max.to_string();
                ui.text_edit_singleline(&mut input);
                if let Ok(input) = input.parse() {
                    ui_state.max = input;
                }
            });
            for i in 0..=ui_state.max - ui_state.min {
                ui.horizontal(|ui| {
                    let height = ui_state.min + i;
                    let weight = ui_state.weights[i as usize];
                    ui.label(height.to_string());
                    let mut input = weight.to_string();
                    ui.text_edit_singleline(&mut input);
                    if let Ok(input) = input.parse() {
                        ui_state.weights[i as usize] = input;
                    }
                });
            }
            if ui.button("start").clicked() {
                ui_state.result = Some(
                    generate_world::generate_elevation_map_with_perlin_noise(
                        ui_state.seed,
                        ui_state.width,
                        ui_state.height,
                        ui_state.scale,
                        ui_state.min,
                        ui_state.max,
                        &ui_state.weights,
                    )
                    .unwrap(),
                );
            }
            if ui_state.result.is_some() {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                        ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                        for column in ui_state.result.clone().unwrap() {
                            ui.vertical(|ui| {
                                for item in column.iter().rev() {
                                    let background_color = if *item > 0 {
                                        Color32::from_rgb(0, item.abs() as u8 * 80, 0)
                                    } else {
                                        Color32::from_rgb(0, 0, item.abs() as u8 * 80)
                                    };
                                    egui::Frame::none().fill(background_color).show(ui, |ui| {
                                        ui.set_width(20.);
                                        ui.set_height(20.);
                                        ui.label(item.to_string());
                                    });
                                }
                            });
                        }
                    });
                });
            }
        });
    });
}
