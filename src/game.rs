use bevy::prelude::*;

use self::options::{load_options, save_changed_options, Options, OptionsChangeEvent};

pub mod options;
pub mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Options>()
            .add_event::<OptionsChangeEvent>()
            .add_systems(PreStartup, load_options)
            .add_systems(Update, save_changed_options);
    }
}
