pub mod core;
pub mod item;
mod code_gen;
pub mod block;
pub mod molang;
pub mod entity;
mod bedrock;

use std::collections::HashMap;
pub use eo;
use eo::sjson::{SJsonElement, SJsonValue, TransformHashMap};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::metadata::{AdditionalMetadataBuilder, PackMetadata, PackMetadataBuilder};
    use crate::core::utilities::{Identifier, SemVer};
    use crate::core::{PackImplementation, Woah};
    use crate::item::{Item, ItemBuilder};
    use eo::sjson;
    use std::path::PathBuf;
    use eo::sjson::ToSJson;
    use image::Rgba;
    use rand::random;
    use crate::bedrock::metadata::{BedrockSpecificMetadata, BedrockSpecificMetadataBuilder, ScriptModule};
    use crate::core::sprite::Sprite;
    use crate::item::client::ItemTexture;
    struct Addon;

    impl PackImplementation for Addon {
        fn initialize(&self, events: &core::PackRegistrationEvents) {
            events.item_registration.subscribe(|reg| {
                for i in 1..=100 {
                    let damage = i.sjson();
                    let name = format!("Item No. {i}").sjson();
                    let icon = format!("woah:item_icon_{i}").sjson();
                    reg.register_item(
                        ItemBuilder::default()
                            .id(Identifier::new("woah", format!("item_{i}")))
                            .components(
                                sjson! {
                                    minecraft:damage {
                                       value = $damage
                                    },
                                    minecraft:display_name {
                                        value = $name
                                    },
                                    minecraft:icon = $icon
                                }
                            ).build().unwrap()
                    )
                }
            });

            events.client_item_registration.subscribe(|reg| {
                for i in 1..=100 {
                    let mut sprite = Sprite::read("./textures/item.png");
                    sprite.accept(|_, _, color| {
                        Rgba([
                            (color.0[0] as f32 * random::<f32>()) as u8,
                            (color.0[1] as f32 * random::<f32>()) as u8,
                            (color.0[2] as f32 * random::<f32>()) as u8,
                            (color.0[3] as f32 * random::<f32>()) as u8,
                        ])
                    });

                    reg.register_texture(
                        ItemTexture::new(
                            Identifier::new("woah", format!("item_icon_{i}")),
                            sprite
                        )
                    )
                }
            })
        }

        fn metadata(&self) -> PackMetadata {
            PackMetadataBuilder::default()
                .name("WoahTest")
                .version((1, 0, 0))
                .author("NaKeR")
                .description("Nothing here")
                .additional(
                    AdditionalMetadataBuilder::default()
                        .bedrock_specific(
                            BedrockSpecificMetadataBuilder::default()
                                .min_engine_version((1, 26, 20))
                                .script_modules(vec![
                                    ScriptModule::new(
                                        "@minecraft/server",
                                        SemVer::new_beta(2, 0, 0)
                                    ),
                                    ScriptModule::new(
                                        "@minecraft/server-ui",
                                        SemVer::new_beta(2, 0, 0)
                                    )
                                ]).build().unwrap()
                        ).build().unwrap()
                ).build().unwrap()
        }

        fn build_path(&self) -> PathBuf {
            PathBuf::from("./woah/test/")
        }
    }

    #[test]
    fn main() {
        Woah::pack(Addon);
    }
}
