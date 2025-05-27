use log::info;
use crate::core::core_registry::REGISTRY;
use crate::item::client::ItemTexture;
use crate::item::Item;

pub struct ItemRegistry {}

impl ItemRegistry {
    pub fn register_item(&self, item: Item) {
        info!("@item Registering item {}", &item.id.render());

        REGISTRY.register_item(item);
    }
}

pub struct ClientItemRegistry {}

impl ClientItemRegistry {
    pub fn register_item_texture(&self, texture: ItemTexture) {
        info!("@client:item Registering item texture {}", &texture.id);
        
        REGISTRY.register_item_texture(texture);
    }
}