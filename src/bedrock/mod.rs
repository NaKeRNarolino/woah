pub mod bedrock_generator;
pub(crate) mod metadata;

pub trait BedrockSerializable {
    fn bedrock_serialize(&self) -> String;
}