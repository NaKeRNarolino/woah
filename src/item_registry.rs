use log::info;
use crate::item::Item;

pub struct ItemRegistry;

impl ItemRegistry {
    pub fn register_item(&self, item: Item) {
        info!("Registering an item!");
    }
}