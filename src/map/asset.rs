use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture,
};
use futures::io::AsyncReadExt;

/// Asset to hold maps
#[derive(TypeUuid, TypePath, Debug, Asset)]
#[uuid = "08e151d8-a821-4591-93c5-6125216ebb53"] // Any UUID will work
pub struct MapAsset {
    /// Holds the name of the map
    pub name: Option<String>,
    /// The LDtk project structure
    pub project: ldtk::Project,
}

#[derive(Default)]
pub struct MapAssetLoader;

impl AssetLoader for MapAssetLoader {
    type Asset = MapAsset;
    type Settings = ();
    type Error = anyhow::Error;

    /// Load a map asset
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext<'_>,
    ) -> BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
        Box::pin(async move {
            info!("Loading map...");
            let mut bytes: Vec<u8> = vec![];
            reader.read_to_end(&mut bytes).await?;
            let project: ldtk::Project = serde_json::from_slice(&bytes)?;

            // Build the asset
            let asset = MapAsset { name: None, project };

            Ok(asset)
        })
    }

    /// Returns a list of supported extensions
    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
