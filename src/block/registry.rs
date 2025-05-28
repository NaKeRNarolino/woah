use log::info;
use crate::block::Block;
use crate::block::client::BlockTexture;
use crate::core::core_registry::REGISTRY;

pub struct BlockRegistry {}

impl BlockRegistry {
    pub fn register_block(&self, block: Block) {
        info!("@block Registering block {}", &block.id.render());

        REGISTRY.register_block(block);
    }
}

pub struct ClientBlockRegistry {}

impl ClientBlockRegistry {
    pub fn register_texture(&self, texture: BlockTexture) {
        info!("@client:block Registering block texture {}", &texture.id);
        
        REGISTRY.register_block_texture(texture);
    }   
}
