use bevy::prelude::*;
use bevy_fluent::Locale;

use crate::game::options::Options;

pub fn load_ui_options(options: Res<Options>, mut locale: ResMut<Locale>) {
    locale.requested = options
        .language
        .parse()
        .expect(&format!("Language code {} parse fail", options.language));
}
