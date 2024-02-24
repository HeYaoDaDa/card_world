use super::MainMenuState;
pub use asset::FluentResourceAsset;
use asset::FluentResourceAssetLoader;
use bevy::prelude::*;
pub use load::LoadI18nTask;
pub use resource::I18n;

mod asset;
mod load;
mod resource;

pub struct I18nPlugin;

impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<I18n>()
            .init_asset::<FluentResourceAsset>()
            .init_asset_loader::<FluentResourceAssetLoader>()
            .add_systems(PreStartup, load::spawn_load_i18n_task_system)
            .add_systems(
                Update,
                load::handle_load_i18n_task_system.run_if(in_state(MainMenuState::Loading)),
            );
    }
}
