use bevy::{input::common_conditions::input_toggle_active, prelude::*};

use bevy_inspector_egui::{bevy_egui::EguiContext, egui, quick::WorldInspectorPlugin};

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
        .add_systems(Startup, (spawn_camera, setup_font));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera2D"), Camera2dBundle::default()));
}

fn setup_font(mut context_query: Query<&mut EguiContext>) {
    let mut context = context_query.single_mut();
    let context = context.get_mut();
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    context.set_fonts(fonts);
}
