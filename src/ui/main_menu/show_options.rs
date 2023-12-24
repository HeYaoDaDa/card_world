use crate::MainMenuState;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_inspector_egui::bevy_egui::*;

pub fn show_options_system(
    mut ui: EguiContexts,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut window_query: Query<&mut Window>,
) {
    let mut window = window_query.single_mut();
    let mut vsync = matches!(window.present_mode, PresentMode::AutoVsync);
    egui::Area::new("show_options")
        .movable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
        .show(ui.ctx_mut(), |ui| {
            if ui.checkbox(&mut vsync, "VSync").clicked() {
                window.present_mode = if vsync {
                    PresentMode::AutoVsync
                } else {
                    PresentMode::AutoNoVsync
                };
            }
            if ui.button("Back").clicked() {
                next_menu_state.set(MainMenuState::MainMenu);
            }
        });
}
