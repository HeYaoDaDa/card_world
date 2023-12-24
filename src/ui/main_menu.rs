use crate::MainMenuState;
use bevy::prelude::*;

use self::{
    hidden_main_menu_system::hidden_main_menu_system, options::OptionsPlugin,
    show_main_menu_system::show_main_menu_system, update_main_menu_system::update_main_menu_system,
};

mod hidden_main_menu_system;
mod options;
mod show_main_menu_system;
mod update_main_menu_system;

#[derive(Component)]
struct MainMenuComp;

#[derive(Component)]
enum MainMenuButtion {
    OptionsButtion,
    ExitGameButton,
}
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OptionsPlugin)
            .add_systems(OnEnter(MainMenuState::MainMenu), show_main_menu_system)
            .add_systems(OnExit(MainMenuState::MainMenu), hidden_main_menu_system)
            .add_systems(
                Update,
                update_main_menu_system.run_if(in_state(MainMenuState::MainMenu)),
            );
    }
}
