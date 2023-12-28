use bevy::{
    asset::{LoadState, LoadedFolder},
    prelude::*,
};
use bevy_fluent::LocalizationBuilder;

use super::{i18n::I18n, MainMenuState};

pub fn load_i18n_system(
    localization_builder: LocalizationBuilder,
    asset_server: Res<AssetServer>,
    mut i18n: ResMut<I18n>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut handle: Local<Option<Handle<LoadedFolder>>>,
) {
    let handle = &*handle.get_or_insert_with(|| asset_server.load_folder("lang"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        i18n.localization = localization_builder.build(handle);
        i18n.handle = handle.clone();
        next_menu_state.set(MainMenuState::MainMenu);
    }
}
