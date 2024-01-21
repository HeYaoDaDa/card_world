use bevy::{log::LogPlugin, prelude::*};

mod game;
mod ui;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::TRACE,
                    filter: "info,wgpu=error,naga=warn,card_world=trace".into(),
                }),
            ui::UiPlugin,
            game::GamePlugin,
        ))
        .run();
}
