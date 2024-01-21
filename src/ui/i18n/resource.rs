use super::asset::FluentResourceAsset;
use bevy::prelude::*;
use fluent::{bundle::FluentBundle, FluentResource};
use fluent_content::Content;
use fluent_langneg::NegotiationStrategy;
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::{slice::from_ref, sync::Arc};
use unic_langid::LanguageIdentifier;

#[derive(Resource)]

pub struct I18n {
    pub language: LanguageIdentifier,
    pub fluent_bundles: Vec<FluentBundle<Arc<FluentResource>, IntlLangMemoizer>>,
    pub handles: Vec<Handle<FluentResourceAsset>>,
}

impl Default for I18n {
    fn default() -> Self {
        Self {
            language: "en".parse().unwrap(),
            fluent_bundles: Default::default(),
            handles: Default::default(),
        }
    }
}

impl I18n {
    pub fn content(&self, request: &str) -> String {
        self.fluent_bundles
            .iter()
            .find_map(|bundle| {
                let content = bundle.content(request);
                content
            })
            .unwrap_or(request.to_string())
    }

    pub fn load(&mut self, fluent_resource_assets: &Assets<FluentResourceAsset>) {
        debug!("reloading i18n");
        self.fluent_bundles.clear();
        let assects = self
            .handles
            .iter()
            .map(|handle| fluent_resource_assets.get(handle).unwrap())
            .collect::<Vec<_>>();
        let langs = assects
            .iter()
            .map(|assect| &assect.language)
            .collect::<Vec<_>>();
        trace!(
            "available language has {:?}",
            langs
                .iter()
                .map(|it| it.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        let langs = fallback_chain(langs, &self.language, &"en".parse().unwrap());
        trace!(
            "fallback chain language: {:?}",
            langs
                .iter()
                .map(|it| it.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        for language in langs {
            let mut bundle = FluentBundle::new_concurrent(vec![language.clone()]);
            //todo order by mods order
            for assect in assects.iter().filter(|it| it.language == language) {
                bundle.add_resource_overriding(assect.fluent_resource.clone());
            }
            self.fluent_bundles.push(bundle);
        }
    }
}

fn fallback_chain<'a>(
    locales: Vec<&'a LanguageIdentifier>,
    language: &'a LanguageIdentifier,
    default: &'a LanguageIdentifier,
) -> Vec<LanguageIdentifier> {
    let supported = fluent_langneg::negotiate_languages(
        from_ref(language),
        &locales,
        Some(&default),
        NegotiationStrategy::Filtering,
    );
    supported
        .into_iter()
        .copied()
        .map(|it| it.clone())
        .collect()
}
