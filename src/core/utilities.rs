use std::collections::HashMap;
use std::fmt::Display;
use serde::{Deserialize, Serialize, Serializer};

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

impl Into<Identifier> for (&str, &str) {
    fn into(self) -> Identifier {
        Identifier::new(self.0, self.1)
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

#[derive(Default, Clone, Debug)]
pub struct SemVer {
    major: u32,
    minor: u32,
    hotfix: u32,
    beta: bool
}

impl SemVer {
    pub fn render_dotted(&self) -> String {
        format!("{}.{}.{}{}", self.major, self.minor, self.hotfix, if self.beta { "-beta" } else { "" })
    }

    pub fn render_commas(&self) -> String {
        format!("{},{},{}", self.major, self.minor, self.hotfix)
    }

    pub fn new_beta(major: u32, minor: u32, hotfix: u32) -> Self {
        Self { major, minor, hotfix, beta: true }
    }

    pub fn new(major: u32, minor: u32, hotfix: u32) -> Self {
        Self { major, minor, hotfix, beta: false }
    }
    
    pub fn latest() -> Self {
        Self::new(1, 21, 80)
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
        formatjson::format_json(&self).unwrap()
    }
}