use std::collections::HashMap;

use crate::game::options::{Options, OptionsChangeEvent};
use bevy::{asset::LoadedFolder, prelude::*};
use bevy_fluent::{exts::fluent::BundleExt, BundleAsset, Locale, Localization, ResourceAsset};
use std::any::TypeId;

use super::{i18n::I18n, MainMenuState};

pub fn update_options_system(
    options: Res<Options>,
    mut old_options: Local<Options>,
    mut locale: ResMut<Locale>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    assets: Res<Assets<BundleAsset>>,
    mut i18n: ResMut<I18n>,
    menu_state: Res<State<MainMenuState>>,
    mut options_change_event: EventWriter<OptionsChangeEvent>,
) {
    if old_options.language != options.language {
        locale.requested = options
            .language
            .parse()
            .expect(&format!("Language code {} parse fail", options.language));
        if !matches!(menu_state.get(), MainMenuState::Loading) {
            i18n.localization = build(&loaded_folders, &assets, &locale, &i18n.handle);
        }
        if !old_options.language.is_empty() {
            options_change_event.send_default();
        }
        old_options.language = options.language.clone();
    }
}

fn build(
    loaded_folders: &Res<Assets<LoadedFolder>>,
    assets: &Res<Assets<BundleAsset>>,
    locale: &ResMut<Locale>,
    handle: &Handle<LoadedFolder>,
) -> Localization {
    let mut localization = Localization::new();
    if let Some(loaded_folder) = loaded_folders.get(handle) {
        let locale_entries: HashMap<_, _> = loaded_folder
            .handles
            .iter()
            .filter_map(|untyped_handle| {
                if untyped_handle.type_id() != TypeId::of::<BundleAsset>() {
                    if untyped_handle.type_id() != TypeId::of::<ResourceAsset>() {
                        warn!(
                            r#""{:?}" locale folder contains not only `BundleAsset` or `ResourceAsset` "{:?}"."#,
                            handle.path(), untyped_handle.path()
                        );
                    }
                    return None;
                }
                // TODO
                let typed_handle = untyped_handle.clone_weak().typed();
                if let Some(asset) = assets.get(&typed_handle) {
                    Some((asset.locale(), Entry { handle: typed_handle, asset }))
                } else {
                    error!(
                        "{:?} `BundleAsset` didn't receive.",
                        typed_handle.path(),
                    );
                    None
                }
            })
            .collect();
        let locales = locale.fallback_chain(locale_entries.keys().cloned());
        for locale in locales {
            localization.insert(&locale_entries[locale].handle, locale_entries[locale].asset);
        }
    } else {
        error!("{:?} locale folder didn't load.", handle.path());
    }
    localization
}

struct Entry<'a> {
    handle: Handle<BundleAsset>,
    asset: &'a BundleAsset,
}
