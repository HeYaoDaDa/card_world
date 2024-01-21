use std::{
    fs::File,
    io::{Read, Write},
    ops::{Deref, DerefMut},
    path::Path,
};

use bevy::{prelude::*, window::PresentMode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Resource)]
pub struct Options {
    pub language: String,
    pub v_sync: bool,
    pub show_fps: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            v_sync: false,
            show_fps: false,
        }
    }
}

#[derive(Event, Default)]
pub struct OptionsChangeEvent;

const OPTIONS_PATH: &str = "assets/data/options.json";

pub fn load_options(mut options: ResMut<Options>, mut window_query: Query<&mut Window>) {
    let options = options.deref_mut();
    let options_path = Path::new(OPTIONS_PATH);
    debug!("Start load options.json");
    if options_path.exists() {
        debug!("Exist options.json");
        let mut options_file = File::open(options_path).expect("Open options.json fail");
        let mut options_json = String::new();
        options_file
            .read_to_string(&mut options_json)
            .expect("Read options.json fail");
        let new_options: Options =
            serde_json::from_str(&options_json).expect("Parse options.json fail");
        *options = new_options;
        debug!("Read options.json success")
    } else {
        debug!("Not exist options.json");
        let mut options_file = File::create(options_path).expect("Create options.json fail");
        let options_json = serde_json::to_string(options).expect("JSONify options fail");
        options_file
            .write_all(options_json.as_bytes())
            .expect("Write options.json fail");
        debug!("Create options.json success")
    }
    let mut window = window_query.single_mut();
    window.present_mode = if options.v_sync {
        PresentMode::AutoVsync
    } else {
        PresentMode::AutoNoVsync
    };
}

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
