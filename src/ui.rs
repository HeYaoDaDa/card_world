use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui, quick::WorldInspectorPlugin};
use i18n::LoadI18nTask;

mod i18n;
mod load_font;
mod show_fps;
mod show_main_menu;
mod show_new_game;
mod show_options;
mod show_saves;
mod show_tests;
mod update_options;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_plugins((
                WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
                show_tests::ShowTestPlugin,
                i18n::I18nPlugin,
            ))
            .add_systems(Startup, (spawn_camera, setup_font))
            .add_systems(
                Update,
                (
                    show_fps::show_fps_system,
                    update_options::update_ui_options_system,
                    handle_ui_load_finish_system.run_if(in_state(MainMenuState::Loading)),
                    show_main_menu::show_main_menu_system.run_if(in_state(MainMenuState::MainMenu)),
                    show_options::show_options_system.run_if(in_state(MainMenuState::Options)),
                    show_saves::show_saves_system.run_if(in_state(MainMenuState::Saves)),
                    show_new_game::show_new_game_system.run_if(in_state(MainMenuState::NewGame)),
                ),
            );
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    Loading,
    MainMenu,
    Saves,
    Options,
    Tests,
    NewGame,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera2D"), Camera2dBundle::default()));
}

fn handle_ui_load_finish_system(
    mut app_state: ResMut<NextState<MainMenuState>>,
    load_i18n_task_query: Query<&LoadI18nTask>,
) {
    if load_i18n_task_query.is_empty() {
        app_state.set(MainMenuState::MainMenu);
        debug!("all ui load task finish");
    }
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
