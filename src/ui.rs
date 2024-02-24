use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod i18n;
mod load_font;
mod load_task;
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
        app.init_state::<MainMenuState>()
            .add_plugins((
                WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Backquote)),
                show_tests::ShowTestPlugin,
                i18n::I18nPlugin,
            ))
            .add_systems(Startup, spawn_camera)
            .add_systems(
                OnEnter(MainMenuState::Loading),
                load_font::spawn_load_font_task_system,
            )
            .add_systems(
                Update,
                (
                    show_fps::show_fps_system,
                    update_options::update_ui_options_system.run_if(not(in_state(MainMenuState::Loading))),
                    load_font::handle_load_font_task_system
                        .run_if(in_state(MainMenuState::Loading)),
                    load_task::handle_load_finish_system.run_if(in_state(MainMenuState::Loading)),
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
