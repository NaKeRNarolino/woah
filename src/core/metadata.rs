use std::collections::HashMap;
use derive_builder::Builder;
use eo::sjson::SJsonElement;
use crate::core::utilities::SemVer;
use crate::bedrock::metadata::BedrockSpecificMetadata;

/// Metadata for an Add-on.
#[derive(Default, Clone, Builder)]
#[builder(setter(into))]
pub struct PackMetadata {
    pub name: String,
    pub version: SemVer,
    pub author: String,
    pub description: String,
    #[builder(default = "None")]
    pub additional: Option<AdditionalMetadata>
}

#[derive(Clone, Default, Builder)]
#[builder(setter(into))]
pub struct AdditionalMetadata {
    #[builder(default = "None")]
    pub(crate) bedrock_specific: Option<BedrockSpecificMetadata>,
    #[builder(default = "None")]
    pub(crate) additional: Option<HashMap<String, SJsonElement>>
}

impl PackMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}

