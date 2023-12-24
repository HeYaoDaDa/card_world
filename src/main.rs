use bevy::prelude::*;

mod ui;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    Loading,
    MainMenu,
    Options,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_state::<MainMenuState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            ui::UiPlugin,
        ))
        .run();
}
