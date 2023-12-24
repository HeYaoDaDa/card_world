use bevy::{
    asset::{LoadState, LoadedFolder},
    prelude::*,
};
use bevy_fluent::{FluentPlugin, Locale, Localization, LocalizationBuilder};
use fluent_content::Content;

use crate::MainMenuState;

pub struct I18nPlugin;

impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<I18n>()
            .insert_resource(
                Locale::new("en-US".parse().unwrap()).with_default("en-US".parse().unwrap()),
            )
            .add_plugins(FluentPlugin)
            .add_systems(Update, load_lang.run_if(in_state(MainMenuState::Loading)));
    }
}

#[derive(Default, Resource)]

pub struct I18n(pub Localization);

impl I18n {
    pub fn content(&self, request: &str) -> String {
        self.0.content(request).unwrap_or_else(|| {
            warn!("i18n id:{request} is miss");
            request.to_string()
        })
    }
}

fn load_lang(
    localization_builder: LocalizationBuilder,
    asset_server: Res<AssetServer>,
    mut i18n: ResMut<I18n>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut handle: Local<Option<Handle<LoadedFolder>>>,
) {
    let handle = &*handle.get_or_insert_with(|| asset_server.load_folder("lang"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        i18n.0 = localization_builder.build(handle);
        next_menu_state.set(MainMenuState::MainMenu);
    }
}
