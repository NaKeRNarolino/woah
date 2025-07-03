pub mod generator;

use crate::core::core_registry::REGISTRY;
use crate::core::metadata::{AddonBp, AddonMetadata, AddonRp};
use crate::core::utilities::{JsonFormat, SerializeVec};
use crate::core::Serializable;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tera::Tera;
use uuid::Uuid;
use template_encoder::template_encoder;
use crate::code_gen::generator::{GeneratorInstance, PackGenerator};
use crate::core::bedrock_generator::WoahBedrockGenerator;

type Generators = Vec<Arc<dyn PackGenerator>>;

pub struct CodeGen {
    output_path: RwLock<PathBuf>,
    generators: RwLock<Generators>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WoahConfig {
    pub uuid1b: String,
    pub uuid2b: String,
    pub uuid3b: String,
    pub uuid1r: String,
    pub uuid2r: String,
}

impl WoahConfig {
    pub fn read() -> WoahConfig {
        serde_json::from_str(
            &fs::read_to_string(CODE_GEN.output_path().join("_woah.json")).unwrap()
        ).unwrap()
    }
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        
        template_encoder!("./templates");
        
        tera
    };

    pub static ref CODE_GEN: CodeGen = CodeGen {
        output_path: RwLock::new(PathBuf::new()),
        generators: RwLock::new(
            vec![
                WoahBedrockGenerator.generator()
            ]
        )
    };
}

impl CodeGen {
    pub fn set_output_path(&self, path: PathBuf) {
        *self.output_path.write().unwrap() = path;
    }

    pub fn output_path(&self) -> PathBuf {
        (&*self.output_path.read().unwrap()).clone()
    }


    pub fn set_generators(&self, generators: Generators) {
        *self.generators.write().unwrap() = generators;
    }

    pub fn generators(&self) -> Generators {
        (&*self.generators.read().unwrap()).clone()
    }

    pub fn metadata(&self) -> AddonMetadata {
        (&*REGISTRY.addon_metadata.read().unwrap()).clone()
    }

    pub fn build(&self) -> anyhow::Result<()> {
        let generators = self.generators();

        fs::create_dir_all(self.output_path())?;

        generators.iter().for_each(|generator| {
            generator.build(self.output_path())
        });

        self.try_generate_uuid();

        let metadata = self.metadata();

        self.build_manifest(&generators, &metadata);

        self.build_items(&generators, &metadata);
        
        self.build_blocks(&generators, &metadata);

        Ok(())
    }

    pub fn build_manifest(&self, generators: &Generators, metadata: &AddonMetadata) {
        for generator in generators {
            generator.build_manifest(
                self.output_path(),
                metadata,
            )
        }
    }

    pub fn try_generate_uuid(&self) {
        let config_path = self.output_path().join("_woah.json");
        if let Err(_) = fs::read_to_string(&config_path) {
            fs::write(&config_path, serde_json::to_string_pretty(&WoahConfig {
                uuid1b: Uuid::new_v4().to_string(),
                uuid2b: Uuid::new_v4().to_string(),
                uuid3b: Uuid::new_v4().to_string(),
                uuid1r: Uuid::new_v4().to_string(),
                uuid2r: Uuid::new_v4().to_string()
            }).unwrap()).unwrap();
        }
    }

    pub fn build_items(&self, generators: &Generators, metadata: &AddonMetadata) {
        let items = REGISTRY.items.read().unwrap().clone();

        let output = self.output_path();

        for generator in generators {
            generator.build_items(output.clone(), items.clone(), metadata)
        }

        self.build_client_items(generators, metadata);
    }
    
    pub fn build_client_items(&self, generators: &Generators, metadata: &AddonMetadata) {
        let items = REGISTRY.item_textures.read().unwrap().clone();
        let output = self.output_path();

        for generator in generators {
            generator.build_client_items(output.clone(), items.clone(), metadata);
        }
    }
    
    pub fn build_blocks(&self, generators: &Generators, metadata: &AddonMetadata) {
        let blocks = REGISTRY.blocks.read().unwrap().clone();
        
        for generator in generators {
            generator.build_blocks(self.output_path(), blocks.clone(), metadata)
        }

        self.build_block_textures(generators, metadata);
    }

    pub fn build_block_textures(&self, generators: &Generators, metadata: &AddonMetadata) {
        let blocks = REGISTRY.block_textures.read().unwrap().clone();

        let output = self.output_path();

        for generator in generators {
            generator.build_block_textures(output.clone(), blocks.clone(), metadata);
        }
    }
}