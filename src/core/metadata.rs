use crate::code_gen::{WoahConfig, TEMPLATES};
use crate::core::core_registry::REGISTRY;
use crate::core::utilities::SemVer;
use crate::core::Serializable;

#[derive(Default)]
pub struct AddonMetadata {
    pub name: String,
    pub version: SemVer,
    pub author: String,
    pub description: String,
    pub min_engine_version: SemVer,
    pub script_modules: Vec<ScriptModule>
}

impl AddonMetadata {
    pub fn new(name: impl Into<String>, version: SemVer, author: impl Into<String>, description: impl Into<String>, min_engine_version: SemVer, script_modules: Vec<ScriptModule>) -> Self {
        Self {
            name: name.into(), version,
            author: author.into(),
            description: description.into(),
            min_engine_version,
            script_modules
        }
    }
}

#[derive(Clone)]
pub struct ScriptModule {
    name: String,
    version: SemVer,
}

impl ScriptModule {
    pub fn new(name: impl Into<String>, version: SemVer) -> Self {
        Self {
            name: name.into(), version
        }
    }

    pub fn render(&self) -> String {
        let mut context = tera::Context::new();

        context.insert("name", &self.name);
        context.insert("version", &self.version.render_dotted());

        TEMPLATES.render("manifest/script_module.json", &context).unwrap()
    }
}

impl Serializable for ScriptModule {
    fn serialize(&self) -> String {
        self.render()
    }
}

pub(crate) struct AddonBp;
pub(crate) struct AddonRp;

impl Serializable for AddonBp {
    fn serialize(&self) -> String {
        let md = &REGISTRY.addon_metadata.read().unwrap();

        let mut c = tera::Context::new();

        let conf = WoahConfig::read();

        let serialized_script_modules = md.script_modules.clone().into_iter().map(
            |x| x.serialize()
        ).collect::<Vec<String>>().join(",");

        c.insert("name", &md.name);
        c.insert("version", &md.version.render_commas());
        c.insert("author", &md.author);
        c.insert("description", &md.description);
        c.insert("min_engine_version", &md.min_engine_version.render_commas());
        c.insert("uuid_1", &conf.uuid1b);
        c.insert("uuid_2", &conf.uuid2b);
        c.insert("uuid_3", &conf.uuid3b);
        c.insert("use_scripts", &(md.script_modules.len() > 0));
        c.insert("script_modules", &serialized_script_modules);
        
        TEMPLATES.render("manifest/behavior_pack.json", &c).unwrap()
    }
}

impl Serializable for AddonRp {
    fn serialize(&self) -> String {
        let md = &REGISTRY.addon_metadata.read().unwrap();

        let mut c = tera::Context::new();

        let conf = WoahConfig::read();

        c.insert("name", &md.name);
        c.insert("version", &md.version.render_commas());
        c.insert("author", &md.author);
        c.insert("description", &md.description);
        c.insert("min_engine_version", &md.min_engine_version.render_commas());
        c.insert("uuid_1", &conf.uuid1r);
        c.insert("uuid_2", &conf.uuid2r);

        TEMPLATES.render("manifest/resource_pack.json", &c).unwrap()
    }
}