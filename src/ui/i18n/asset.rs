use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::*;
use bevy::utils::thiserror;
use fluent::FluentResource;
use std::io;
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;
use unic_langid::LanguageIdentifier;

#[derive(Asset, TypePath, Debug)]
pub struct FluentResourceAsset {
    pub language: LanguageIdentifier,
    pub source: String,
    pub path: String,
    pub fluent_resource: Arc<FluentResource>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Default)]
pub struct FluentResourceAssetLoader;

impl AssetLoader for FluentResourceAssetLoader {
    type Asset = FluentResourceAsset;
    type Settings = ();
    type Error = Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        debug!(
            "loading ftl asset {}",
            load_context.path().to_str().unwrap_or_default()
        );
        Box::pin(async move {
            let mut content = String::new();
            reader.read_to_string(&mut content).await?;
            let fluent_resource = match FluentResource::try_new(content) {
                Ok(fluent_resource) => fluent_resource,
                Err((fluent_resource, errors)) => {
                    error_span!("try_new").in_scope(|| {
                        for error in errors {
                            error!(%error);
                        }
                    });
                    fluent_resource
                }
            };
            let file_name = load_context.path().file_stem().unwrap().to_string_lossy();
            let asset = FluentResourceAsset {
                language: file_name.parse().unwrap(),
                source: get_source(load_context.path()),
                path: load_context.path().to_string_lossy().to_string(),
                fluent_resource: Arc::new(fluent_resource),
            };
            trace!(
                "load ftl asset {}, source: {}, language: {}",
                asset.path,
                asset.source,
                asset.language
            );
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}

fn get_source(path: &Path) -> String {
    let mut mod_id = None;
    let mut parent_path = path.parent();
    for _ in 0..3 {
        if let Some(parent_path_value) = parent_path {
            if let Some(parent_file_name) = parent_path_value.file_name() {
                let parent_file_name = parent_file_name.to_str().unwrap();
                if parent_file_name == "mods" {
                    return mod_id.unwrap();
                } else {
                    mod_id = Some(parent_file_name.to_string());
                    parent_path = parent_path_value.parent();
                }
            }
        }
    }
    "core".to_string()
}
