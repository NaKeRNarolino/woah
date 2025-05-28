pub mod state;
pub mod permutation;
pub mod traits;
pub mod registry;
pub mod client;

use std::collections::HashMap;
use eo::sjson::{SJsonElement, SJsonValue, TransformHashMap};
use crate::block::permutation::BlockPermutation;
use crate::block::state::BlockState;
use crate::block::traits::BlockTrait;
use crate::code_gen::TEMPLATES;
use crate::core::Serializable;
use crate::core::utilities::{Identifier, SemVer, SerializeVec};

#[derive(Clone, Debug)]
pub struct Block {
    pub id: Identifier,
    pub components: HashMap<String, SJsonValue>,
    pub format_version: SemVer,
    pub states: Vec<BlockState>,
    pub permutations: Vec<BlockPermutation>,
    pub traits: Vec<BlockTrait>
}

impl Block {
    pub fn new(id: Identifier, components: Vec<SJsonElement>) -> Self {
        Self {
            id,
            components: components.transform_hashmap(),
            format_version: SemVer::latest(),
            permutations: Vec::new(),
            states: Vec::new(),
            traits: Vec::new()
        }
    }

    pub fn using_format_version(&self, format_version: SemVer) -> Self {
        Self {
            format_version,
            ..self.clone()
        }
    }
    
    pub fn using_states(&self, states: Vec<BlockState>) -> Self {
        Self {
            states,
            ..self.clone()
        }
    }
    
    pub fn using_permutations(&self, permutations: Vec<BlockPermutation>) -> Self {
        Self {
            permutations,
            ..self.clone()
        }
    }
    
    pub fn using_traits(&self, traits: Vec<BlockTrait>) -> Self {
        Self {
            traits,
            ..self.clone()
        }
    }
}

impl Serializable for Block {
    fn serialize(&self) -> String {
        let id = &self.id.render();
        let components = serde_json::to_string(&self.components).unwrap();
        let format_version = &self.format_version.render_dotted();
        let states = &self.states.serialize_vec().join(",");
        let permutations = &self.permutations.serialize_vec().join(",");
        let traits = &self.traits.serialize_vec().join(",");

        let mut c = tera::Context::new();

        c.insert("id", &id);
        c.insert("components", &components);
        c.insert("format_version", &format_version);
        c.insert("states", &states);
        c.insert("traits", &traits);
        c.insert("permutations", &permutations);

        TEMPLATES.render("block/block.json", &c).unwrap()
    }
}