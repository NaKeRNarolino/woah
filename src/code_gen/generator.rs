use std::path::PathBuf;
use std::sync::Arc;
use crate::block::Block;
use crate::block::client::BlockTexture;
use crate::core::metadata::PackMetadata;
use crate::item::client::ItemTexture;
use crate::item::Item;

pub trait GeneratorInstance {
    /// A function that returns an Arc with the generator in it.
    fn generator(&self) -> Arc<Self> where Self: Clone {
        Arc::new(self.clone())
    }
}

impl<T> GeneratorInstance for T
where 
    T : PackGenerator + Clone {
}

pub trait PackGenerator : Send + Sync {
    /// Meant to generate necessary folders and stuff like that.
    fn build(&self, output_path: PathBuf);
    /// Meant to generate the main parts of the pack, like the manifest in Bedrock add-ons.
    fn build_manifest(&self, output_path: PathBuf, metadata: &PackMetadata);
    /// Meant to generate stuff for items, not client-sided.
    fn build_items(&self, output_path: PathBuf, items: Vec<Item>, metadata: &PackMetadata);
    /// Meant to generate client-sided stuff for items.
    fn build_client_items(&self, output_path: PathBuf, items: Vec<ItemTexture>, metadata: &PackMetadata);
    /// Meant to generate stuff for blocks, not client-sided.
    fn build_blocks(&self, output_path: PathBuf, blocks: Vec<Block>, metadata: &PackMetadata);
    /// Meant to generate client-sided stuff for blocks.
    fn build_client_blocks(&self, output_path: PathBuf, blocks: Vec<BlockTexture>, metadata: &PackMetadata);
}