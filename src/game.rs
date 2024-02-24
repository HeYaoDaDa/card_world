use bevy::prelude::*;

use self::{
    modinfo::ModInfos,
    options::{Options, OptionsChangeEvent},
};

pub mod generate_world;
mod load_task;
pub mod modinfo;
pub mod options;
pub mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<Options>()
            .init_resource::<ModInfos>()
            .add_event::<OptionsChangeEvent>()
            .add_systems(
                OnEnter(AppState::Loading),
                (
                    options::spawn_load_options_task_system,
                    modinfo::spawn_load_modinfos_task_system,
                ),
            )
            .add_systems(
                Update,
                (
                    options::save_changed_options,
                    options::update_options_system,
                    options::handle_load_options_task_system.run_if(in_state(AppState::Loading)),
                    modinfo::handle_load_modinfos_task_system.run_if(in_state(AppState::Loading)),
                    load_task::handle_load_finish_system.run_if(in_state(AppState::Loading)),
                ),
            );
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
}
