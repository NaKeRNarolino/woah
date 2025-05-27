pub mod utilities;
pub mod metadata;
pub(crate) mod core_registry;
pub mod sprite;

use crate::code_gen::CODE_GEN;
use crate::core::core_registry::REGISTRY;
use crate::core::metadata::AddonMetadata;
use crate::item::item_registry::{ClientItemRegistry, ItemRegistry};
use eo::event_init;
use eo::events::Event;
use log::LevelFilter;
use std::path::PathBuf;
use std::sync::RwLock;

pub trait AddonStartupPoint {
    fn initialize(&self, events: &AddonRegistrationEvents);

    fn metadata(&self) -> AddonMetadata;

    fn build_path(&self) -> PathBuf;
}

pub struct AddonRegistrationEvents<'a> {
    pub item_registration: Event<'a, ItemRegistry>,
    pub client_item_registration: Event<'a, ClientItemRegistry>
}

impl<'a> AddonRegistrationEvents<'a> {
    pub fn new() -> Self {
        Self {
            item_registration: event_init!(ItemRegistry),
            client_item_registration: event_init!(ClientItemRegistry)
        }
    }
}

pub struct Woah;

impl Woah {
    /// The entry point of Woah. This function should only be called ONCE.
    pub fn addon(addon: impl AddonStartupPoint) {
        log::set_logger(&eo::logger::EoLogger).unwrap();
        log::set_max_level(LevelFilter::max());

        let events = AddonRegistrationEvents::new();
        addon.initialize(&events);
        events.item_registration.notify(ItemRegistry {});
        events.client_item_registration.notify(ClientItemRegistry {});

        REGISTRY.set_addon_metadata(addon.metadata());

        CODE_GEN.set_output_path(addon.build_path());
        
        CODE_GEN.try_generate_uuid();
        
        CODE_GEN.build().unwrap();
    }
}

pub trait Serializable {
    fn serialize(&self) -> String;
}