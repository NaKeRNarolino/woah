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
pub use proc_macros::woah;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::metadata::{AdditionalMetadata, AdditionalMetadataBuilder, PackMetadata, PackMetadataBuilder};
    use crate::core::utilities::{HoldBuilder, Identifier, SemVer};
    use crate::core::{PackImplementation, Woah};
    use crate::item::{Item, ItemBuilder};
    use eo::sjson;
    use std::path::PathBuf;
    use eo::sjson::ToSJson;
    use image::Rgba;
    use rand::random;
    use crate::bedrock::metadata::{BedrockSpecificMetadata, BedrockSpecificMetadataBuilder, ScriptModule};
    use crate::block::Block;
    use crate::block::client::BlockTexture;
    use crate::block::permutation::BlockPermutation;
    use crate::block::state::{BlockState, BlockStateType};
    use crate::block::traits::{BlockTrait, PlacementDirectionState};
    use crate::core::sprite::Sprite;
    use crate::item::client::ItemTexture;
    use crate::molang::Molang;

    struct Addon;

    impl PackImplementation for Addon {
        fn initialize(&self, events: &core::PackRegistrationEvents) {
            events.item_registration.subscribe(|reg| {
                for i in 1..=100 {
                    let damage = i.sjson();
                    let name = format!("Item No. {i}").sjson();
                    let icon = format!("woah:item_icon_{i}").sjson();
                    reg.register_item(
                        woah! {
                            @Item {
                                id = ("woah", format!("item_{i}"));
                                components = sjson! {
                                    minecraft:damage {
                                        value = $damage
                                    },
                                    minecraft:display_name {
                                        value = $name
                                    },
                                    minecraft:icon = $icon
                                };
                            }
                        }
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
            });

            events.block_registration.subscribe(|reg| {
                reg.register_block(woah! {
                    @Block {
                        format_version = (1, 26, 20);
                        id = "woah:block";
                        states = vec![
                            BlockState::new(
                                "woah:val".into(),
                                BlockStateType::Range(0..=5)
                            ),
                            BlockState::new(
                                "woah:toggle".into(),
                                BlockStateType::Boolean
                            )
                        ];
                        traits = vec![
                            BlockTrait::PlacementDirection {
                                enabled_states: vec![PlacementDirectionState::CardinalDirection],
                                y_rotation_offset: 0
                            }
                        ];
                        components = sjson! {
                            minecraft:display_name = "Hi"
                        };
                        permutations = vec![
                            BlockPermutation::new(
                                Molang::new("q.block_state('woah:val') > 2") & Molang::new("q.block_state('woah:toggle')"),
                                sjson! {
                                    minecraft:mining_speed {
                                        speed = 10 // I don't remember the syntax let's assume this is the component
                                    },
                                    minecraft:material_instances {
                                        * {
                                            texture = "woah:block"
                                        }
                                    }
                                }
                            )
                        ]
                    }
                })
            });

            events.client_block_registration.subscribe(|reg| {
                reg.register_texture(
                    BlockTexture::new(
                        "woah:block".into(),
                        Sprite::read("./textures/block.png")
                    )
                )
            })
        }

        fn metadata(&self) -> PackMetadata {
            woah! {
                @PackMetadata {
                    name = "WoahTest";
                    version = (1, 0, 0);
                    author = "NaKeR";
                    description = "Smth";
                    additional = @AdditionalMetadata {
                        bedrock_specific = @BedrockSpecificMetadata {
                            min_engine_version = (1, 26, 20);
                            script_modules = vec![
                                ScriptModule::new(
                                    "@minecraft/server",
                                    SemVer::new_beta(2, 0, 0)
                                ),
                                ScriptModule::new(
                                    "@minecraft/server-ui",
                                    SemVer::new_beta(2, 0, 0)
                                )
                            ];
                        }
                    };
                }
            }
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
