use crate::{AppState, MainMenuState};
use bevy::prelude::*;

use self::{
    show_main_menu_system::show_main_menu_system, update_main_menu_system::update_main_menu_system,
};

mod show_main_menu_system;
mod update_main_menu_system;

#[derive(Component)]
enum MainMenuButtion {
    ExitGameButton,
}
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), show_main_menu_system)
            .add_systems(
                Update,
                update_main_menu_system.run_if(in_state(MainMenuState::MainMenu)),
            );
    }
}
