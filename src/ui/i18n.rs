use super::MainMenuState;
pub use asset::FluentResourceAsset;
use asset::FluentResourceAssetLoader;
use bevy::prelude::*;
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
            .add_systems(
                Update,
                load::load_i18n_system.run_if(in_state(MainMenuState::Loading)),
            );
    }
}
