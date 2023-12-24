use bevy::{input::common_conditions::input_toggle_active, prelude::*};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod fps;
mod i18n;
mod main_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
            fps::FpsPlugin,
            main_menu::MainMenuPlugin,
            i18n::I18nPlugin,
        ))
        .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera2D"), Camera2dBundle::default()));
}
