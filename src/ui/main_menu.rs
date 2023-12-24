use bevy::prelude::*;

use crate::MainMenuState;

use self::{show_main_menu::show_main_menu_system, show_options::show_options_system};

mod show_main_menu;
mod show_options;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                show_main_menu_system.run_if(in_state(MainMenuState::MainMenu)),
                show_options_system.run_if(in_state(MainMenuState::Options)),
            ),
        );
    }
}
