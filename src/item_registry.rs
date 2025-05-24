use log::info;

pub struct ItemRegistry;

impl ItemRegistry {
    pub fn register_item(&self, item: Item) {
        info!("Registering an item!");
    }
}