use std::ops::Deref;

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