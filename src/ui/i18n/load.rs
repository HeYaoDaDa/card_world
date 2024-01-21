use super::asset::FluentResourceAsset;
use super::{I18n, MainMenuState};
use bevy::{asset::LoadState, prelude::*};
use std::fs;
use std::path::PathBuf;

pub fn load_i18n_system(
    asset_server: Res<AssetServer>,
    fluent_resource_assets: Res<Assets<FluentResourceAsset>>,
    mut i18n: ResMut<I18n>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    if i18n.handles.is_empty() {
        let mut ftl_paths = get_ftl_files_in_folder("assets/lang");
        ftl_paths.extend(get_mods_ftl_path("assets/mods"));
        for ftl_path in ftl_paths {
            i18n.handles.push(
                asset_server.load(
                    ftl_path
                        .strip_prefix("assets")
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                ),
            );
        }
    } else {
        for handle in &i18n.handles {
            if !matches!(asset_server.get_load_state(handle), Some(LoadState::Loaded)) {
                return;
            }
        }
        i18n.load(&fluent_resource_assets);
        next_menu_state.set(MainMenuState::MainMenu);
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
