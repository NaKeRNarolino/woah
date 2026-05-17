use std::collections::HashMap;
use eo::sjson::{SJsonElement, SJsonValue, TransformHashMap};
use crate::code_gen::TEMPLATES;
use crate::bedrock::BedrockSerializable;
use crate::molang::Molang;

/// A struct for describing Block permutations.
#[derive(Clone, Debug)]
pub struct BlockPermutation {
    condition: Molang,
    components: HashMap<String, SJsonValue>
}

impl BlockPermutation {
    pub fn new(condition: Molang, components: Vec<SJsonElement>) -> Self {
        Self {
            condition,
            components: components.transform_hashmap()
        }
    }
}

impl BedrockSerializable for BlockPermutation {
    fn bedrock_serialize(&self) -> String {
        let components = serde_json::to_string(&self.components).unwrap();
        
        let condition = &self.condition.bedrock_serialize();
        
        let mut c = tera::Context::new();
        
        c.insert("condition", &condition);
        c.insert("components", &components);
        
        TEMPLATES.render("block/block_permutation.json", &c).unwrap()
    }
}