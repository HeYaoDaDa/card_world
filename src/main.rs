use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use bevy::{log::LogPlugin, prelude::*};
use serde::{Deserialize, Serialize};

mod game;
mod ui;

const OPTIONS_PATH: &str = "assets/data/options.json";

fn main() {
    let mut options = Options::default();
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
        options = new_options;
        debug!("Read options.json success")
    } else {
        debug!("Not exist options.json");
        let mut options_file = File::create(options_path).expect("Create options.json fail");
        let options_json = serde_json::to_string(&options).expect("JSONify options fail");
        options_file
            .write_all(options_json.as_bytes())
            .expect("Write options.json fail");
        debug!("Create options.json success")
    }

    App::new()
        .insert_resource(options.clone())
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: if options.v_sync {
                            bevy::window::PresentMode::AutoVsync
                        } else {
                            bevy::window::PresentMode::AutoNoVsync
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::TRACE,
                    filter: "info,wgpu=error,naga=warn,card_world=trace".into(),
                    ..default()
                }),
            ui::UiPlugin,
            game::GamePlugin,
        ))
        .run();
}

#[derive(Serialize, Deserialize, Resource, Clone)]
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
