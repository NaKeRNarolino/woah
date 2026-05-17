pub mod utilities;
pub mod metadata;
pub(crate) mod core_registry;
pub mod sprite;

use crate::code_gen::CODE_GEN;
use crate::core::core_registry::REGISTRY;
use crate::core::metadata::PackMetadata;
use crate::item::item_registry::{ClientItemRegistry, ItemRegistry};
use eo::event_init;
use eo::events::Event;
use log::LevelFilter;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use crate::block::registry::{BlockRegistry, ClientBlockRegistry};
use crate::code_gen::generator::{GeneratorInstance, PackGenerator};
use crate::bedrock::bedrock_generator::WoahBedrockGenerator;

/// The core trait for creating a pack. Implement this for your pack struct.
pub trait PackImplementation {
    /// The add-on registration initialization point. Use [PackRegistrationEvents] provided by the function, to register stuff (both on server (BP) and client (RP)).
    fn initialize(&self, events: &PackRegistrationEvents);

    /// A function returning the [AddonMetadata](metadata::PackMetadata) for the addon.
    fn metadata(&self) -> PackMetadata;

    /// A function returning a [PathBuf] to a path where the pack folders will be generated.
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
pub struct PackRegistrationEvents<'a> {
    /// Item registration events. Register items here.
    pub item_registration: Event<'a, ItemRegistry>,
    /// Client item registration events. Register item textures here.
    pub client_item_registration: Event<'a, ClientItemRegistry>,
    /// Block registration events. Register blocks here.
    pub block_registration: Event<'a, BlockRegistry>,
    /// Client block registration. Register block textures here.
    pub client_block_registration: Event<'a, ClientBlockRegistry>,
}

impl<'a> PackRegistrationEvents<'a> {
    pub(crate) fn new() -> Self {
        Self {
            item_registration: event_init!(ItemRegistry),
            client_item_registration: event_init!(ClientItemRegistry),
            block_registration: event_init!(BlockRegistry),
            client_block_registration: event_init!(ClientBlockRegistry)
        }
    }
}


/// The main struct for the `Woah` framework. Use this for initializing the pack.
pub struct Woah;

impl Woah {
    /// The entry point of Woah. This function should only be called ONCE.
    pub fn pack(pack: impl PackImplementation) {
        log::set_logger(&eo::logger::EoLogger).unwrap();
        log::set_max_level(LevelFilter::max());

        let events = PackRegistrationEvents::new();
        pack.initialize(&events);
        events.item_registration.notify(ItemRegistry {});
        events.client_item_registration.notify(ClientItemRegistry {});
        events.block_registration.notify(BlockRegistry {});
        events.client_block_registration.notify(ClientBlockRegistry {});

        REGISTRY.set_pack_metadata(pack.metadata());

        CODE_GEN.set_output_path(pack.build_path());

        CODE_GEN.set_generators(
            pack.generators()
        );
        
        CODE_GEN.build().unwrap();
    }
}

