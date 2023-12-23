use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
            ui::UiPlugin,
        ))
        .run();
}
