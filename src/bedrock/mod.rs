pub mod bedrock_generator;
pub mod metadata;

pub trait BedrockSerializable {
    fn bedrock_serialize(&self) -> String;
}