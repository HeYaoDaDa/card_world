use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::*;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_fps_system);
    }
}

fn show_fps_system(mut ui: EguiContexts, time: Res<Time<Real>>) {
    egui::Area::new("show_fps")
        .constrain(true)
        .show(ui.ctx_mut(), |ui| {
            let delta_sec = time.delta_seconds_f64();
            let mut fps = 0.0;
            if delta_sec != 0.0 {
                fps = 1.0 / delta_sec;
            }
            ui.horizontal(|ui| {
                ui.label("FPS:");
                ui.label(format!("{:.0}", fps));
            });
        });
}
