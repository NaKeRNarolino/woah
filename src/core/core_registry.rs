use crate::core::metadata::AddonMetadata;
use crate::item::Item;
use lazy_static::lazy_static;
use std::sync::RwLock;
use crate::block::Block;
use crate::block::client::BlockTexture;
use crate::item::client::ItemTexture;

pub(crate) struct CoreRegistry {
    pub addon_metadata: RwLock<AddonMetadata>,
    pub items: RwLock<Vec<Item>>,
    pub item_textures: RwLock<Vec<ItemTexture>>,
    pub blocks: RwLock<Vec<Block>>,
    pub block_textures: RwLock<Vec<BlockTexture>>,
}

lazy_static! {
    pub(crate) static ref REGISTRY: CoreRegistry = CoreRegistry {
        addon_metadata: RwLock::new(AddonMetadata::default()),
        items: RwLock::new(Vec::new()),
        item_textures: RwLock::new(Vec::new()),
        blocks: RwLock::new(Vec::new()),
        block_textures: RwLock::new(Vec::new()),
    };
}

impl CoreRegistry {
    pub fn set_addon_metadata(&self, addon_metadata: AddonMetadata) {
        *self.addon_metadata.write().unwrap() = addon_metadata;
    }

    pub fn register_item(&self, item: Item) {
        self.items.write().unwrap().push(item);
    }

    pub fn register_item_texture(&self, texture: ItemTexture) {
        self.item_textures.write().unwrap().push(texture);
    }

    pub fn register_block(&self, block: Block) {
        self.blocks.write().unwrap().push(block);
    }
    
    pub fn register_block_texture(&self, texture: BlockTexture) {
        self.block_textures.write().unwrap().push(texture);
    }
}