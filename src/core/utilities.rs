use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use formatjson::FormatJsonError;
use serde::{Deserialize, Serialize, Serializer};
use crate::bedrock::BedrockSerializable;

// A struct for describing `namespace:path` style identifiers.
#[derive(Clone, Debug, Deserialize)]
pub struct Identifier {
    namespace: String,
    path: String
}

impl Identifier {
    pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            path: path.into()
        }
    }
}

impl<T, E> From<(T, E)> for Identifier
where T:
    Into<String>,
E: Into<String> {
    fn from(value: (T, E)) -> Self {
        Self { namespace: value.0.into(), path: value.1.into() }
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        let (namespace, path) = value.split_once(':').unwrap();
        Self {
            namespace: namespace.to_string(),
            path: path.to_string(),
        }
    }
}

impl Identifier {
    pub fn render(&self) -> String {
        format!("{}:{}", &self.namespace, &self.path)
    }
    
    pub fn render_underscore(&self) -> String {
        format!("{}_{}", &self.namespace, &self.path)
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.render())
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.render())
    }
}

#[derive(Clone)]
pub enum ScriptModuleVer {
    SemVer(SemVer),
    String(String)
}

impl ScriptModuleVer {
    pub fn render(&self) -> String {
        match self {
            ScriptModuleVer::SemVer(v) => v.render_dotted(),
            ScriptModuleVer::String(s) => s.clone()
        }
    }
}

impl Default for ScriptModuleVer {
    fn default() -> Self {
        ScriptModuleVer::SemVer(SemVer::default())
    }
}

impl<T> From<T> for ScriptModuleVer
where T:
    Into<SemVer> {
    fn from(value: T) -> Self {
        ScriptModuleVer::SemVer(value.into())
    }
}

impl From<String> for ScriptModuleVer {
    fn from(value: String) -> Self {
        ScriptModuleVer::String(value)
    }
}

impl From<&str> for ScriptModuleVer {
    fn from(value: &str) -> Self {
        ScriptModuleVer::String(value.to_string())
    }
}

/// A struct for describing `major.minor.hotfix` & `major.minor.hotfix-beta` style versions.
#[derive(Default, Clone, Debug)]
pub struct SemVer {
    major: u32,
    minor: u32,
    hotfix: u32,
    postfix: String
}

impl SemVer {
    pub fn render_dotted(&self) -> String {
        format!("{}.{}.{}{}", self.major, self.minor, self.hotfix, self.postfix)
    }

    pub fn render_commas(&self) -> String {
        format!("{},{},{}", self.major, self.minor, self.hotfix)
    }

    pub fn new_beta(major: u32, minor: u32, hotfix: u32) -> Self {
        Self { major, minor, hotfix, postfix: "-beta".to_string() }
    }

    pub fn new(major: u32, minor: u32, hotfix: u32) -> Self {
        Self { major, minor, hotfix, postfix: "".to_string() }
    }

    pub fn new_postfix(major: u32, minor: u32, hotfix: u32, postfix: impl Into<String>) -> Self {
        Self { major, minor, hotfix, postfix: postfix.into() }
    }
    
    pub fn latest() -> Self {
        Self::new(1, 26, 30)
    }
}

impl Into<SemVer> for (u32, u32, u32, bool) {
    fn into(self) -> SemVer {
        if self.3 {
            SemVer::new_beta(self.0, self.1, self.2)
        } else {
            SemVer::new(self.0,  self.1, self.2)
        }
    }
}

impl Into<SemVer> for (u32, u32, u32) {
    fn into(self) -> SemVer {
        SemVer::new(self.0, self.1, self.2)
    }
}

impl Serialize for SemVer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.render_dotted())
    }
}

pub trait ContextMultiset {
    fn set_keys(&mut self, keys: HashMap<String, String>);
}

impl ContextMultiset for tera::Context {
    fn set_keys(&mut self, keys: HashMap<String, String>) {
        for (k, v) in keys {
            self.insert(k, &v);
        }
    }
}

pub trait JsonFormat {
    fn json_format(&self) -> String;
}

impl JsonFormat for String {
    fn json_format(&self) -> String {
        match formatjson::format_json(&self) {
            Ok(value) => value,
            Err(err) => {
                log::error!("A file with this contents has wrong json syntax. {}", &self);
                self.to_string()
            }
        }
    }
}

pub trait BedrockSerializeVec {
    fn serialize_vec(&self) -> Vec<String>;
}

impl<T: BedrockSerializable> BedrockSerializeVec for Vec<T> {
    fn serialize_vec(&self) -> Vec<String> {
        self.into_iter().map(|s| s.bedrock_serialize()).collect::<Vec<String>>()
    }
}

pub trait HoldBuilder<B : Default> {
    fn builder() -> B {
        B::default()
    }
}

#[macro_export]
macro_rules! hold_builders {
    ($($id:ident),+) => {
        use crate::core::utilities::HoldBuilder;
        $(
            paste::paste! {
                impl HoldBuilder<[< $id Builder >]> for $id {}
            }
        )*
    };
}

