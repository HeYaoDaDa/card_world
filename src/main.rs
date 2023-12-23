use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod ui;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    MainMenu,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_state::<MainMenuState>()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
            ui::UiPlugin,
        ))
        .run();
}
