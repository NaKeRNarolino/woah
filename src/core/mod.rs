pub mod utilities;

use std::sync::RwLock;
use eo::event_init;
use eo::events::Event;
use log::{log, Level, LevelFilter};
use crate::item_registry::ItemRegistry;

pub trait AddonStartupPoint {
    fn initialize(&self, events: &AddonRegistrationEvents);
}

pub struct AddonRegistrationEvents<'a> {
    pub item_registration: Event<'a, ItemRegistry>
}

impl<'a> AddonRegistrationEvents<'a> {
    pub fn new() -> Self {
        Self {
            item_registration: event_init!(ItemRegistry)
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
        events.item_registration.notify(ItemRegistry);
    }
}