use bevy::{
    asset::{AssetLoader, FileAssetIo, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use tiled::Loader;

/// Asset to hold maps
#[derive(TypeUuid, Debug)]
#[uuid = "08e151d8-a821-4591-93c5-6125216ebb53"] // Any UUID will work
pub struct MapAsset {
    /// Holds the name of the map
    pub name: Option<String>,
    /// The Tiled map structure
    pub map: tiled::Map,
}

#[derive(Default)]
pub struct MapAssetLoader;

impl AssetLoader for MapAssetLoader {
    /// Load a map asset
    fn load<'a>(
        &'a self,
        _bytes: &'a [u8],
        load_context: &'a mut LoadContext<'_>,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            info!("Loading map...");
            let mut loader = Loader::new();
            let asset_io = load_context.asset_io();

            // Ensure that we received a FileAssetIo struct
            assert!(asset_io.is::<FileAssetIo>());

            // Retrive the absolute path to the map file
            let map_path = asset_io
                .downcast_ref::<FileAssetIo>()
                .unwrap()
                .root_path()
                .join(load_context.path());
            debug!("Found Map Path: {:?}", map_path);

            // Load the map
            let map = loader.load_tmx_map(map_path).unwrap();
            debug!("Map uses tiled {}", map.version());

            // Build the asset
            let asset = MapAsset { name: None, map };
            load_context.set_default_asset::<MapAsset>(LoadedAsset::new(asset));

            Ok(())
        })
    }

    /// Returns a list of supported extensions
    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}
