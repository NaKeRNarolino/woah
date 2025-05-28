use log::info;
use crate::block::Block;
use crate::core::core_registry::REGISTRY;

pub struct BlockRegistry {}

impl BlockRegistry {
    pub fn register_block(&self, block: Block) {
        info!("@block Registering block {}", &block.id.render());

        REGISTRY.register_block(block);
    }
}