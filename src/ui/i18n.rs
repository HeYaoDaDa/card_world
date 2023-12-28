use bevy::{asset::LoadedFolder, prelude::*};
use bevy_fluent::Localization;
use fluent_content::Content;

#[derive(Default, Resource)]

pub struct I18n {
    pub localization: Localization,
    pub handle: Handle<LoadedFolder>,
}

impl I18n {
    pub fn content(&self, request: &str) -> String {
        self.localization.content(request).unwrap_or_else(|| {
            warn!("i18n id:{request} is miss");
            request.to_string()
        })
    }
}
