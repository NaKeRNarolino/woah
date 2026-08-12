pub mod core;
pub mod item;
mod code_gen;
pub mod block;
pub mod molang;
pub mod entity;
pub mod bedrock;


use std::collections::HashMap;
pub use eo;
use eo::sjson::{SJsonElement, SJsonValue, TransformHashMap};
pub use proc_macros::woah;

#[cfg(test)]
mod tests {
    use crate::block::components::v1_19_10::DestroyTime;
use eo::sjson::HasSJsonIdent;
    use super::*;
    use crate::core::metadata::{AdditionalMetadata, AdditionalMetadataBuilder, PackMetadata, PackMetadataBuilder};
    use crate::core::utilities::{HoldBuilder, Identifier, SemVer};
    use crate::core::{PackImplementation, Woah};
    use crate::item::{Item, ItemBuilder};
    use eo::sjson;
    use std::path::PathBuf;
    use std::sync::Arc;
    use eo::sjson::{ToSJson};
    use image::Rgba;
    use rand::random;
    use serde_json::json;
    use crate::bedrock::{BedrockTarget, TargetInstance};
    use crate::bedrock::bedrock_generator::WoahBedrockGenerator;
    use crate::bedrock::metadata::{BedrockSpecificMetadata, BedrockSpecificMetadataBuilder, ScriptModule, ScriptModuleName};
    use crate::block::Block;
    use crate::block::client::BlockTexture;
    use crate::block::components::v1_26_20::MaterialInstances;
    use crate::block::permutation::BlockPermutation;
    use crate::block::state::{BlockState, BlockStateType};
    use crate::block::traits::{BlockTrait, PlacementDirectionState};
    use crate::code_gen::generator::{GeneratorInstance, PackGenerator};
    use crate::core::build_target::BuildTarget;
    use crate::core::sprite::Sprite;
    use crate::entity::component_group::EntityComponentGroup;
    use crate::entity::Entity;
    use crate::entity::event::{EntityEvent, EntityFilter, EntityEventQueueCommand};
    use crate::entity::property::{EntityEnumProperty, EntityIntProperty, EntityProperty};
    use crate::item::client::ItemTexture;
    use crate::molang::Molang;
    use crate::item::components::v1_26_10::*;

    struct Addon;

    impl PackImplementation for Addon {
        fn initialize(&self, events: &core::PackProcessingEvents) {
            events.entity_registration.subscribe(|reg| {
                reg.register_entity(woah! {
                    @Entity {
                        id = "cool:entity";
                        components = sjson! {};
                        properties = vec![
                            @EntityProperty {
                                id = "a:b";
                                client_sync = true;
                                property = @EntityEnumProperty {
                                    values = map vec!["a", "b", "c"];
                                    default = "a";
                                }
                            }
                        ];
                        component_groups = vec![
                            @EntityComponentGroup {
                                id = "cool_component_group";
                                components = sjson! {};
                            }
                        ];
                        events = vec![
                            @EntityEvent {
                                filters = vec![
                                    @EntityFilter {
                                        operator = "==";
                                        test = "has_biome_tag";
                                        value = "plains";
                                    }
                                ];
                                randomize = vec![
                                    @EntityEvent {
                                        weight = 90.0;
                                    },
                                    @EntityEvent {
                                        weight = 10.0;
                                        sequence = vec![
                                            @EntityEvent {
                                                filters = vec![

                                                ]
                                            }
                                        ]
                                    }
                                ];
                                queue_command = @EntityEventQueueCommand {
                                    target = "self";
                                    command("/hi");
                                    command("/bye");
                                };
                                set_property = sjson! {
                                    hi = 2
                                };

                            }
                        ]
                    }
                })
            });

            events.item_registration.subscribe(|reg| {
                for i in 1..=100 {
                    let name = format!("Item No. {i}");
                    let icon = format!("woah:item_icon_{i}");
                    reg.register_item(
                        woah! {
                            @Item {
                                id = ("woah", format!("item_{i}"));
                                components = sjson! {
                                    minecraft:display_name = $name,
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
                            minecraft:display_name = "Cool Woah Block"
                        };
                        permutations = vec![
                            BlockPermutation::new(
                                Molang::new("q.block_state('woah:val') > 2"),
                                sjson! {
                                    minecraft:material_instances {
                                        * {
                                            texture = "texture_2"
                                        }
                                    }
                                }
                            )
                        ]
                    }
                })
            });
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
                            min_engine_version = (1, 26, 40);
                            script_modules = vec![
                                @ScriptModule {
                                    name = ScriptModuleName::Server;
                                    version = (2, 8, 0);
                                },
                                @ScriptModule {
                                    name = ScriptModuleName::Ui;
                                    version = (2, 1, 0);
                                }
                            ];
                        };
                    };
                }
            }
        }

        fn targets(&self) -> Vec<Arc<dyn BuildTarget>> {
            vec![
                BedrockTarget::develop("./woah/develop").target()
            ]
        }
    }

    #[test]
    fn main() {
        Woah::pack(Addon);
    }
}
