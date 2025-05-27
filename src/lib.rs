pub mod core;
pub mod item;
mod code_gen;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::metadata::{AddonMetadata, ScriptModule};
    use crate::core::utilities::SemVer;
    use crate::core::{AddonStartupPoint, Woah};
    use crate::item::Item;
    use eo::{infix, sjson};
    use std::path::PathBuf;
    use crate::core::sprite::Sprite;
    use crate::item::client::ItemTexture;

    struct Addon;

    impl AddonStartupPoint for Addon {
        fn initialize(&self, events: &core::AddonRegistrationEvents) {
            events.item_registration.subscribe(|reg| {
                infix! {
                    reg register_item Item::new(
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
                }
            });
            
            events.client_item_registration.subscribe(|reg| {
                infix! {
                    reg register_item_texture ItemTexture::new(
                        ("x", "test").into(),
                        Sprite::read("./textures/gear_details_2.png")
                    )
                }
            });
        }

        fn metadata(&self) -> AddonMetadata {
            AddonMetadata::new(
                "Woah!!!",
                SemVer::new(1, 0, 0),
                "NaKeR",
                "Empty description.",
                SemVer::new(1, 21, 80),
                vec![
                    ScriptModule::new(
                        "@minecraft/server",
                        SemVer::new_beta(2, 0, 0)
                    ),
                    ScriptModule::new(
                        "@minecraft/server-ui",
                        SemVer::new_beta(2, 0, 0)
                    )
                ]
            )
        }

        fn build_path(&self) -> PathBuf {
            PathBuf::from("./woah/test/")
        }
    }

    #[test]
    fn main() {
        Woah::addon(Addon);
    }
}
