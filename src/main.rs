use bevy::{log::LogPlugin, prelude::*};

mod game;
mod ui;

fn main() {
    App::new()
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
                    ..default()
                }),
            ui::UiPlugin,
            game::GamePlugin,
        ))
        .run();
}
