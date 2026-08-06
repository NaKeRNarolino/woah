use std::path::PathBuf;
use std::sync::Arc;
use crate::code_gen::generator::GeneratorInstance;
use crate::core::build_target::BuildTarget;

pub mod bedrock_generator;
pub mod metadata;

pub trait BedrockSerializable {
    fn bedrock_serialize(&self) -> String;
}

#[derive(Clone)]
pub enum BedrockTarget {
    Develop {
        path: PathBuf
    },
    Local {
        path: PathBuf
    }
}

impl BedrockTarget {
    pub fn develop(path: impl Into<PathBuf>) -> Self {
        Self::Develop {
            path: path.into()
        }
    }

    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::Local {
            path: path.into()
        }
    }
}

impl BuildTarget for BedrockTarget {
    fn path(&self) -> PathBuf {
        match &self {
            BedrockTarget::Develop { path } => path.clone(),
            BedrockTarget::Local { path } => path.clone()
        }
    }
}

pub trait TargetInstance {
    /// A function that returns an Arc with the target in it.
    fn target(&self) -> Arc<Self> where Self: Clone {
        Arc::new(self.clone())
    }
}

impl<T> TargetInstance for T
where
    T : BuildTarget + Clone {
}