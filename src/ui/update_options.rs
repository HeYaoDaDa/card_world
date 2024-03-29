use std::ops::{Deref, DerefMut};

use crate::{game::options::OptionsChangeEvent, Options};
use bevy::prelude::*;

use super::i18n::{FluentResourceAsset, I18n};

pub fn update_ui_options_system(
    options: Res<Options>,
    mut old_options: Local<Option<Options>>,
    mut i18n: ResMut<I18n>,
    fluent_resource_assets: Res<Assets<FluentResourceAsset>>,
    mut options_change_event: EventWriter<OptionsChangeEvent>,
) {
    if let Some(old_options) = old_options.deref_mut() {
        let mut change = false;
        if old_options.language != options.language {
            old_options.language = options.language.clone();
            change = true;
            i18n.language = options
                .language
                .parse()
                .expect(&format!("Language code {} parse fail", options.language));
            i18n.load(&fluent_resource_assets);
        }
        if old_options.show_fps != options.show_fps {
            old_options.show_fps = options.show_fps;
            change = true;
        }
        if change {
            debug!("ui options update");
            options_change_event.send_default();
        }
    } else {
        *old_options = Some(options.deref().clone());
    }
}
