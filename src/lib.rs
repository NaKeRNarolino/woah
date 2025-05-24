mod core;
mod item_registry;
mod item;

#[cfg(test)]
mod tests {
    use eo::sjson;
    use crate::core::{AddonStartupPoint, Woah};
    use crate::core::utilities::Identifier;
    use crate::item::Item;
    use super::*;

    struct Addon;

    impl AddonStartupPoint for Addon {
        fn initialize(&self, events: &core::AddonRegistrationEvents) {
            events.item_registration.subscribe(|reg| {
                reg.register_item(
                    Item::new(
                        ("x", "test").into(),
                        sjson! {
                            minecraft:damage {
                                value = 7
                            },
                            minecraft:display_name {
                                value = "Test of sJSON as a primary component writing method"
                            },
                            minecraft:icon = "item_icon"
                        }   
                    )
                );
            })
        }
    }

    #[test]
    fn main() {
        Woah::addon(Addon);
        
    }
}
