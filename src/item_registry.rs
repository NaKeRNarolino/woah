use log::info;
use crate::core::core_registry::REGISTRY;
use crate::item::Item;

pub struct ItemRegistry;

impl ItemRegistry {
    pub fn register_item(&self, item: Item) {
        info!("@item Registering item {}", &item.id.render());

        REGISTRY.register_item(item);
    }
}