pub mod utilities;
pub mod metadata;
pub(crate) mod core_registry;
pub mod sprite;
pub mod build_target;

use crate::code_gen::CODE_GEN;
use crate::core::core_registry::REGISTRY;
use crate::core::metadata::PackMetadata;
use crate::item::registry::{ClientItemRegistry, ItemRegistry};
use eo::event_init;
use eo::events::Event;
use log::LevelFilter;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use crate::block::registry::{BlockRegistry, ClientBlockRegistry};
use crate::code_gen::generator::{GeneratorInstance, PackGenerator};
use crate::bedrock::bedrock_generator::WoahBedrockGenerator;
use crate::core::build_target::BuildTarget;
use crate::entity::registry::EntityRegistry;

/// The core trait for creating a pack. Implement this for your pack struct.
pub trait PackImplementation {
    /// The add-on registration initialization point. Use [PackProcessingEvents] provided by the function, to register stuff (both on server (BP) and client (RP)).
    fn initialize(&self, events: &PackProcessingEvents);

    /// A function returning the [AddonMetadata](metadata::PackMetadata) for the addon.
    fn metadata(&self) -> PackMetadata;

    /// A function returning a [PathBuf] to a path where the pack folders will be generated.
    fn targets(&self) -> Vec<Arc<dyn BuildTarget>>;

    /// A function returning the [PackGenerator](crate::code_gen::generator::PackGenerator)s for the pack. Defaults to the default Minecraft Bedrock generator.
    /// When passing a generator, call [.generator](crate::code_gen::generator::GeneratorInstance::generator) on it.
    fn generators(&self) -> Vec<Arc<dyn PackGenerator>> {
        vec![
            WoahBedrockGenerator.generator()
        ]
    }
}

pub struct PackFinalization {}

pub enum PackPart {
    BehaviourPack,
    ResourcePack,
    Custom(String)
}

pub struct PackPath {
    part: PackPart,
    path: String
}

impl PackPath {
    pub fn new(part: PackPart, path: impl Into<String>) -> Self {
        Self {
            part, path: path.into()
        }
    }
}

impl<T> From<T> for PackPath
where T:
    ToString {
    fn from(value: T) -> Self {
        let string = value.to_string();

        let split = string.split(":").collect::<Vec<&str>>();

        let part = if split[0].to_lowercase() == "bp" {
            PackPart::BehaviourPack
        } else if split[0].to_lowercase() == "rp" {
            PackPart::ResourcePack
        } else {
            PackPart::Custom(split[0].to_string())
        };

        PackPath {
            part,
            path: split[1].to_string()
        }
    }
}

impl PackFinalization {
    fn attach_file(&self, path: impl Into<PackPath>) {

    }
}

/// Events for registering stuff. Subscribe to them using `.subscribe()`.
pub struct PackProcessingEvents<'a> {
    /// Item registration events. Register items here.
    pub item_registration: Event<'a, ItemRegistry>,
    /// Client item registration events. Register item textures here.
    pub client_item_registration: Event<'a, ClientItemRegistry>,
    /// Block registration events. Register blocks here.
    pub block_registration: Event<'a, BlockRegistry>,
    /// Client block registration. Register block textures here.
    pub client_block_registration: Event<'a, ClientBlockRegistry>,
    /// Block registration events. Register blocks here.
    pub entity_registration: Event<'a, EntityRegistry>,
    /// Pack finalization, runs after the registration and main codegen phases
    pub finalization: Event<'a, PackFinalization>
}

impl<'a> PackProcessingEvents<'a> {
    pub(crate) fn new() -> Self {
        Self {
            item_registration: event_init!(ItemRegistry),
            client_item_registration: event_init!(ClientItemRegistry),
            block_registration: event_init!(BlockRegistry),
            client_block_registration: event_init!(ClientBlockRegistry),
            entity_registration: event_init!(EntityRegistry),
            finalization: event_init!(PackFinalization),
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

        let events = PackProcessingEvents::new();
        pack.initialize(&events);
        events.item_registration.notify(ItemRegistry {});
        events.client_item_registration.notify(ClientItemRegistry {});
        events.block_registration.notify(BlockRegistry {});
        events.client_block_registration.notify(ClientBlockRegistry {});
        events.entity_registration.notify(EntityRegistry {});

        REGISTRY.set_pack_metadata(pack.metadata());

        // CODE_GEN.set_output_path(pack.build_path());
        CODE_GEN.set_targets(
            pack.targets()
        );
        CODE_GEN.set_generators(
            pack.generators()
        );
        
        CODE_GEN.build().unwrap();

        events.finalization.notify(PackFinalization {});
    }
}

