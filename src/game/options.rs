use std::{
    fs::File,
    io::Write,
    ops::{Deref, DerefMut},
    path::Path,
};

use bevy::{prelude::*, window::PresentMode};

use crate::{Options, OPTIONS_PATH};

#[derive(Event, Default)]
pub struct OptionsChangeEvent;

pub fn save_changed_options(
    options: Res<Options>,
    mut options_change_event: EventReader<OptionsChangeEvent>,
) {
    if !options_change_event.is_empty() {
        options_change_event.clear();
        debug!("Start Save options.json");
        let options_path = Path::new(OPTIONS_PATH);
        let mut options_file = if options_path.exists() {
            debug!("Exist options.json");
            File::create(options_path).expect("Open options.json fail")
        } else {
            debug!("Not exist options.json");
            File::create(options_path).expect("Create options.json fail")
        };
        let options_json = serde_json::to_string(options.deref()).expect("JSONify options fail");
        options_file
            .write_all(options_json.as_bytes())
            .expect("Write options.json fail");
        debug!("Save options.json success")
    }
}

pub fn update_options_system(
    options: Res<Options>,
    mut old_options: Local<Option<Options>>,
    mut options_change_event: EventWriter<OptionsChangeEvent>,
    mut window_query: Query<&mut bevy::window::Window>,
) {
    if let Some(old_options) = old_options.deref_mut() {
        let mut change = false;
        if old_options.v_sync != options.v_sync {
            old_options.v_sync = options.v_sync;
            change = true;
            let mut window = window_query.single_mut();
            window.present_mode = if options.v_sync {
                PresentMode::AutoVsync
            } else {
                PresentMode::AutoNoVsync
            };
            options_change_event.send_default();
        }
        if change {
            debug!("options update");
            options_change_event.send_default();
        }
    } else {
        *old_options = Some(options.deref().clone());
    }
}
