use std::ops::{RangeInclusive};
use crate::code_gen::TEMPLATES;
use crate::core::Serializable;
use crate::core::utilities::Identifier;

/// A struct describing a Block state.
#[derive(Clone, Debug)]
pub struct BlockState {
    id: Identifier,
    state_type: BlockStateType
}

/// An enum for all Block state types. (string and integer arrays, boolean states and integer range states)
#[derive(Clone, Debug)]
pub enum BlockStateType {
    String(Vec<String>),
    Boolean(),
    Integer(Vec<i64>),
    Range(RangeInclusive<i32>)
}

impl Serializable for BlockState {
    fn serialize(&self) -> String {
        match &self.state_type {
            BlockStateType::String(v) => {
                let ser = v.into_iter().map(|x| format!("\"{x}\"")).collect::<Vec<String>>().join(",");
                
                let id = &self.id.render();
                
                let mut c = tera::Context::new();
                
                c.insert("id", &id);
                c.insert("values", &ser);
                
                TEMPLATES.render("block/block_state_arr.json", &c).unwrap()
            },
            BlockStateType::Boolean() => {
                let ser = vec![true, false].into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");

                let id = &self.id.render();

                let mut c = tera::Context::new();

                c.insert("id", &id);
                c.insert("values", &ser);

                TEMPLATES.render("block/block_state_arr.json", &c).unwrap()
            },
            BlockStateType::Integer(v) => {
                let ser = v.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
                
                let id = &self.id.render();

                let mut c = tera::Context::new();

                c.insert("id", &id);
                c.insert("values", &ser);

                TEMPLATES.render("block/block_state_arr.json", &c).unwrap()
            },
            BlockStateType::Range(r) => {
                let id = &self.id.render();
                
                let mut c = tera::Context::new();
                
                c.insert("id", &id);
                c.insert("min", &r.clone().min().unwrap_or(0));
                c.insert("max", &r.clone().max().unwrap_or(15));
                
                TEMPLATES.render("block/block_state_range.json", &c).unwrap()
            }
        }
    }
}

impl BlockState {
    pub fn new(id: Identifier, state_type: BlockStateType) -> Self {
        Self {
            id, state_type
        }
    }
}