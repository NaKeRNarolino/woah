use log::info;
use crate::core::core_registry::REGISTRY;
use crate::entity::Entity;
use crate::item::client::ItemTexture;

pub struct EntityRegistry {}

impl EntityRegistry {
    pub fn register_entity(&self, entity: Entity) {
        info!("@entity Registering entity {}", &entity.id);

        REGISTRY.register_entity(entity);
    }
}