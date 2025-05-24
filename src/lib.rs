mod core;
mod item_registry;
mod item;

#[cfg(test)]
mod tests {
    use crate::core::{AddonStartupPoint, Woah};
    use super::*;

    struct Addon;

    impl AddonStartupPoint for Addon {
        fn initialize(&self, events: &core::AddonEvents) {
            events.item_registration.subscribe(|reg| {
                reg.register_item(/* ... */);
            })
        }
    }

    #[test]
    fn main() {
        Woah::addon(Addon);
    }
}
