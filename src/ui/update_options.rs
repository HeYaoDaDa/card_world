use crate::game::options::{Options, OptionsChangeEvent};
use bevy::prelude::*;

use super::{
    i18n::{FluentResourceAsset, I18n},
    MainMenuState,
};

pub fn update_options_system(
    options: Res<Options>,
    mut old_options: Local<Options>,
    mut i18n: ResMut<I18n>,
    fluent_resource_assets: Res<Assets<FluentResourceAsset>>,
    menu_state: Res<State<MainMenuState>>,
    mut options_change_event: EventWriter<OptionsChangeEvent>,
) {
    if old_options.language != options.language {
        i18n.language = options
            .language
            .parse()
            .expect(&format!("Language code {} parse fail", options.language));
        if !matches!(menu_state.get(), MainMenuState::Loading) {
            i18n.load(&fluent_resource_assets);
        }
        if !old_options.language.is_empty() {
            options_change_event.send_default();
        }
        old_options.language = options.language.clone();
    }
}
