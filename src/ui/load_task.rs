use bevy::prelude::*;

use super::MainMenuState;

#[derive(Component)]
pub struct LoadTask;

pub fn handle_load_finish_system(
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    load_task_query: Query<&LoadTask>,
) {
    if load_task_query.is_empty() {
        main_menu_state.set(MainMenuState::MainMenu);
        debug!("all ui load task finish");
    }
}
