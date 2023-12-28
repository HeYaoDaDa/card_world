use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::*;

use crate::game::options::Options;

pub fn show_fps_system(mut ui: EguiContexts, time: Res<Time<Real>>, options: Res<Options>) {
    if options.show_fps {
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
}
