use bevy::prelude::*;

use crate::MainMenuState;

use self::{
    show_options_system::show_options_system, update_options_system::update_options_system,
};

mod show_options_system;
mod update_options_system;

#[derive(Component)]
struct OptionsComp;

#[derive(Component)]
struct OptionsBackComp;

#[derive(Component)]
enum OptionsInput {
    VSync,
}

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::Options), show_options_system)
            .add_systems(
                Update,
                update_options_system.run_if(in_state(MainMenuState::Options)),
            );
    }
}
