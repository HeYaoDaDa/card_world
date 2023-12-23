use bevy::prelude::*;

mod fps;
mod main_menu;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((fps::ShowFpsPlugin, main_menu::MainMenuPlugin))
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera2D"), Camera2dBundle::default()));
}
