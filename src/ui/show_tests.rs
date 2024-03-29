use crate::ui::i18n::I18n;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::*;

use super::MainMenuState;

mod show_generate_world;

pub struct ShowTestPlugin;

impl Plugin for ShowTestPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TestsState>().add_systems(
            Update,
            (
                show_tests_system
                    .run_if(in_state(MainMenuState::Tests).and_then(in_state(TestsState::Tests))),
                show_generate_world::show_generate_world_system
                    .run_if(in_state(TestsState::GenerateWorld)),
            ),
        );
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum TestsState {
    #[default]
    Tests,
    GenerateWorld,
}

fn show_tests_system(
    mut ctx: EguiContexts,
    i18n: Res<I18n>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut next_tests_state: ResMut<NextState<TestsState>>,
) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            if ui.button(i18n.content("core_cmn-btn-back")).clicked() {
                next_menu_state.set(MainMenuState::MainMenu);
            }
            if ui.button(i18n.content("core_tests-btn-generate-world")).clicked() {
                next_tests_state.set(TestsState::GenerateWorld);
            }
        });
    });
}
