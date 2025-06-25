pub mod core;
pub mod item;
mod code_gen;
pub mod block;
pub mod molang;
pub mod entity;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::metadata::{AddonMetadata, ScriptModule};
    use crate::core::utilities::SemVer;
    use crate::core::{AddonStartupPoint, Woah};
    use crate::item::Item;
    use eo::{infix, sjson};
    use std::path::PathBuf;
    use crate::block::Block;
    use crate::block::client::BlockTexture;
    use crate::block::permutation::BlockPermutation;
    use crate::block::state::{BlockState, BlockStateType};
    use crate::block::traits::{BlockTrait, PlacementDirectionState};
    use crate::core::sprite::Sprite;
    use crate::item::client::ItemTexture;
    use crate::molang::Molang;

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
                            minecraft:icon = "item_icon",
                            x:custom_component {
                                prop = 10
                            }
                        }
                    ).using_format_version((1, 21, 80).into())
                }
            });
            
            events.client_item_registration.subscribe(|reg| {
                infix! {
                    reg register_texture ItemTexture::new(
                        ("x", "test").into(),
                        Sprite::read("./textures/gear_details_2.png")
                    )
                }
            });
            
            events.block_registration.subscribe(|reg| {
                infix! {
                    reg register_block Block::new(
                        ("x", "test_block").into(),
                        sjson! {
                            minecraft:friction {
                                value = 0.2
                            }
                        }
                    ).using_states(vec![BlockState::new(
                        ("x", "test_block_state").into(),
                        BlockStateType::Range(1..=3)
                    )])
                    .using_permutations(
                        vec![
                            BlockPermutation::new(
                                Molang::new("q.block_state('x:test_block_state') == 2") | Molang::new("q.block_state('x:test_block_state') == 3"),
                                sjson! {
                                    minecraft:friction {
                                        value = 0.1
                                    }
                                }
                            )
                        ]
                    )
                    .using_traits(
                        vec![
                            BlockTrait::PlacementDirection {
                                enabled_states: vec![ PlacementDirectionState::CardinalDirection ],
                                y_rotation_offset: 0
                            }
                        ]
                    )
                }
            });
            
            events.client_block_registration.subscribe(|reg| {
                infix! {
                    reg register_texture BlockTexture::new(
                        ("x", "test_block").into(),
                        Sprite::read("./textures/gear_details_2.png")
                    )
                }
            })
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
