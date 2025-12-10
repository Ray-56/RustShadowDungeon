//! RON asset loader for Bevy 0.17
//!
//! 自定义 RON 资产加载器
//!
//! Loads RON files as custom assets (LootTableAsset, ItemDefinitionAsset, etc.)

use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use ron::de::from_str;
use serde::Deserialize;

/// Generic RON asset loader
///
/// 通用 RON 资产加载器
/// 
/// This loader can be used for any asset type that implements Deserialize
pub struct RonAssetLoader<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Default for RonAssetLoader<T> {
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> AssetLoader for RonAssetLoader<T>
where
    T: Asset + TypePath + for<'de> Deserialize<'de>,
{
    type Asset = T;
    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let contents = String::from_utf8(bytes)?;
        let asset: T = from_str(&contents)?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

/// RON asset loader for LootTableAsset
pub type LootTableAssetLoader = RonAssetLoader<crate::infrastructure::components::loot::LootTableAsset>;

/// RON asset loader for ItemDefinitionAsset
/// Note: ItemDefinitionAsset needs to be wrapped in a container struct for RON deserialization
#[derive(Deserialize, TypePath, Asset, Clone, Debug)]
pub struct ItemDefinitionAssetContainer {
    pub items: Vec<crate::infrastructure::components::loot::ItemDefinitionAsset>,
}

pub type ItemDefinitionAssetLoader = RonAssetLoader<ItemDefinitionAssetContainer>;

