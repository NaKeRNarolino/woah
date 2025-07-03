pub mod utilities;
pub mod metadata;
pub(crate) mod core_registry;
pub mod sprite;
pub mod bedrock_generator;

use crate::code_gen::CODE_GEN;
use crate::core::core_registry::REGISTRY;
use crate::core::metadata::AddonMetadata;
use crate::item::item_registry::{ClientItemRegistry, ItemRegistry};
use eo::event_init;
use eo::events::Event;
use log::LevelFilter;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use crate::block::registry::{BlockRegistry, ClientBlockRegistry};
use crate::code_gen::generator::{GeneratorInstance, PackGenerator};
use crate::core::bedrock_generator::WoahBedrockGenerator;

/// The core trait for creating an Add-on pack. Implement this for your addon struct.
pub trait AddonStartupPoint {
    /// The add-on registration initialization point. Use [AddonRegistrationEvents] provided by the function, to register stuff (both on server (BP) and client (RP)).
    fn initialize(&self, events: &AddonRegistrationEvents);

    /// A function returning the [AddonMetadata](metadata::AddonMetadata) for the addon.
    fn metadata(&self) -> AddonMetadata;

    /// A function returning a [PathBuf] to a path where BP and RP folders will be generated.
    fn build_path(&self) -> PathBuf;

    /// A function returning the [PackGenerator](crate::code_gen::generator::PackGenerator)s for the pack. Defaults to the default Minecraft Bedrock generator.
    /// When passing a generator, call [.generator](crate::code_gen::generator::GeneratorInstance::generator) on it.
    fn generators(&self) -> Vec<Arc<dyn PackGenerator>> {
        vec![
            WoahBedrockGenerator.generator()
        ]
    }
}

/// Events for registering stuff. Subscribe to them using `.subscribe()`.
pub struct AddonRegistrationEvents<'a> {
    /// Item registration events. Register items here.
    pub item_registration: Event<'a, ItemRegistry>,
    /// Client item registration events. Register item textures here.
    pub client_item_registration: Event<'a, ClientItemRegistry>,
    /// Block registration events. Register blocks here.
    pub block_registration: Event<'a, BlockRegistry>,
    /// Client block registration. Register block textures here.
    pub client_block_registration: Event<'a, ClientBlockRegistry>,
}

impl<'a> AddonRegistrationEvents<'a> {
    pub(crate) fn new() -> Self {
        Self {
            item_registration: event_init!(ItemRegistry),
            client_item_registration: event_init!(ClientItemRegistry),
            block_registration: event_init!(BlockRegistry),
            client_block_registration: event_init!(ClientBlockRegistry)
        }
    }
}


/// The main struct for the `Woah` framework. Use this for initializing the add-on.
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
        events.block_registration.notify(BlockRegistry {});
        events.client_block_registration.notify(ClientBlockRegistry {});

        REGISTRY.set_addon_metadata(addon.metadata());

        CODE_GEN.set_output_path(addon.build_path());

        CODE_GEN.set_generators(
            addon.generators()
        );
        
        CODE_GEN.build().unwrap();
    }
}

pub trait Serializable {
    fn serialize(&self) -> String;
}