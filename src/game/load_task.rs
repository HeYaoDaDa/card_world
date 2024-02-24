use bevy::prelude::*;

use super::AppState;

#[derive(Component)]
pub struct LoadTask;

pub fn handle_load_finish_system(
    mut app_state: ResMut<NextState<AppState>>,
    load_task_query: Query<&LoadTask>,
) {
    if load_task_query.is_empty() {
        app_state.set(AppState::MainMenu);
        debug!("all game load task finish");
    }
}
