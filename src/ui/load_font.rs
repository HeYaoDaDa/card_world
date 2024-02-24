use bevy::{
    prelude::*,
    tasks::{self, IoTaskPool, Task},
    time::Stopwatch,
};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui};

use crate::ui::load_task::LoadTask;

#[derive(Component)]
pub struct LoadFontTask {
    pub task: Task<&'static [u8]>,
    pub stopwatch: Stopwatch,
}

pub fn spawn_load_font_task_system(mut commands: Commands) {
    debug!("load font task start");
    let task =
        IoTaskPool::get().spawn(async { include_bytes!("../../assets/font.ttf").as_slice() });
    commands.spawn((
        LoadFontTask {
            task,
            stopwatch: Stopwatch::new(),
        },
        LoadTask,
    ));
}

pub fn handle_load_font_task_system(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mut context_query: Query<&mut EguiContext>,
    mut task_query: Query<(Entity, &mut LoadFontTask)>,
) {
    if let Ok((task_entity, mut task)) = task_query.get_single_mut() {
        task.stopwatch.tick(time.delta());
        if task.task.is_finished() {
            let mut context = context_query.single_mut();
            let context = context.get_mut();
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "noto_font".to_owned(),
                egui::FontData::from_static(tasks::block_on(&mut task.task)),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "noto_font".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("noto_font".to_owned());
            context.set_fonts(fonts);
            debug!(
                "load font task finish {}",
                task.stopwatch.elapsed().as_secs_f64()
            );
            commands.entity(task_entity).despawn();
        }
    }
}
