use std::path::PathBuf;

pub trait BuildTarget : Send + Sync {
    fn path(&self) -> PathBuf;
}