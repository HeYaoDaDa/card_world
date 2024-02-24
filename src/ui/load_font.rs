use bevy::{
    prelude::*,
    tasks::{IoTaskPool, Task},
    time::Stopwatch,
};

#[derive(Component)]
pub struct LoadFontTask {
    pub task: Task<&'static [u8]>,
    pub stopwatch: Stopwatch,
}

pub fn spawn_load_font_task_system(mut commands: Commands) {
    debug!("load font task start");
    let task =
        IoTaskPool::get().spawn(async { include_bytes!("../../assets/font.ttf").as_slice() });
    commands.spawn(LoadFontTask {
        task,
        stopwatch: Stopwatch::new(),
    });
}
