use std::path::PathBuf;

pub trait BuildTarget : Send + Sync {
    fn path(&self) -> PathBuf;

    fn path_keyed(&self, key: &str) -> Option<PathBuf>;
}