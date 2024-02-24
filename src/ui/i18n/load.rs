use crate::ui::load_task::LoadTask;

use super::asset::FluentResourceAsset;
use super::I18n;
use bevy::time::Stopwatch;
use bevy::{asset::LoadState, prelude::*};
use std::fs;
use std::path::PathBuf;

#[derive(Component)]
pub struct LoadI18nTask(Stopwatch);

pub fn spawn_load_i18n_task_system(
    mut commands: Commands,
    mut i18n: ResMut<I18n>,
    asset_server: Res<AssetServer>,
) {
    debug!("load i18n task start");
    let mut ftl_paths = get_ftl_files_in_folder("assets/lang");
    ftl_paths.extend(get_mods_ftl_path("assets/mods"));
    i18n.handles = ftl_paths
        .iter()
        .map(|ftl_path| {
            asset_server.load(
                ftl_path
                    .strip_prefix("assets")
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            )
        })
        .collect();
    commands.spawn((LoadI18nTask(Stopwatch::new()), LoadTask));
}

pub fn handle_load_i18n_task_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fluent_resource_assets: Res<Assets<FluentResourceAsset>>,
    time: Res<Time<Real>>,
    mut i18n: ResMut<I18n>,
    mut task_query: Query<(Entity, &mut LoadI18nTask)>,
) {
    if let Ok((task_entity, mut task)) = task_query.get_single_mut() {
        task.0.tick(time.delta());
        for handle in &i18n.handles {
            if !matches!(asset_server.get_load_state(handle), Some(LoadState::Loaded)) {
                return;
            }
        }
        i18n.load(&fluent_resource_assets);
        commands.entity(task_entity).despawn();
        debug!("load i18n task finish {}", task.0.elapsed().as_secs_f64());
    }
}

fn get_ftl_files_in_folder(folder_path: &str) -> Vec<PathBuf> {
    let mut ftl_files = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension().unwrap_or_default() == "ftl" {
                    ftl_files.push(file_path);
                }
            }
        }
    }

    ftl_files
}

fn get_mods_ftl_path(path: &str) -> Vec<PathBuf> {
    let mut ftl_files = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Ok(entries) = fs::read_dir(entry.path()) {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    if let Ok(metadata) = entry.metadata() {
                                        if metadata.is_dir()
                                            && entry.path().file_name().unwrap_or_default()
                                                == "lang"
                                        {
                                            ftl_files.extend(get_ftl_files_in_folder(
                                                entry.path().to_str().unwrap(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    ftl_files
}
