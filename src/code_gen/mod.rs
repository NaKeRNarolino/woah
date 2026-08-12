pub mod generator;

use crate::core::core_registry::REGISTRY;
use crate::core::metadata::PackMetadata;
use crate::core::utilities::{BedrockSerializeVec, JsonFormat};
use crate::bedrock::BedrockSerializable;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tera::Tera;
use uuid::Uuid;
use proc_macros::template_encoder;
use crate::code_gen::generator::{GeneratorInstance, PackGenerator};
use crate::bedrock::bedrock_generator::WoahBedrockGenerator;
use crate::core::build_target::BuildTarget;

type Generators = Vec<Arc<dyn PackGenerator>>;
type Targets = Vec<Arc<dyn BuildTarget>>;

pub struct CodeGen {
    targets: RwLock<Targets>,
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
            &fs::read_to_string("./cache.woah").unwrap()
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
        targets: RwLock::new(vec![]),
        generators: RwLock::new(
            vec![
                WoahBedrockGenerator.generator()
            ]
        )
    };
}

impl CodeGen {
    pub fn set_targets(&self, targets: Targets) {
        *self.targets.write().unwrap() = targets;
    }

    // pub fn output_path(&self) -> PathBuf {
    //     (&*self.output_path.read().unwrap()).clone()
    // }


    pub fn cwd(&self) -> PathBuf {
        PathBuf::from("./")
    }

    pub fn set_generators(&self, generators: Generators) {
        *self.generators.write().unwrap() = generators;
    }

    pub fn generators(&self) -> Generators {
        (&*self.generators.read().unwrap()).clone()
    }

    pub fn targets(&self) -> Targets {
        (&*self.targets.read().unwrap()).clone()
    }

    pub fn metadata(&self) -> PackMetadata {
        (&*REGISTRY.pack_metadata.read().unwrap()).clone()
    }

    pub fn build(&self) -> anyhow::Result<()> {
        let generators = self.generators();
        dbg!(&self.targets().len());

        for target in self.targets() {
            fs::create_dir_all(target.path())?;

            let metadata = self.metadata();

            generators.iter().for_each(|generator| {
                generator.build_prepare(
                    target.clone(),
                    &metadata
                )
            });

            self.try_generate_uuid();

            self.build_manifest(&generators, &metadata, target.clone());

            self.build_items(&generators, &metadata, target.clone());

            self.build_blocks(&generators, &metadata, target.clone());

            self.build_entities(&generators, &metadata, target.clone());
        }

        Ok(())
    }

    pub fn build_manifest(&self, generators: &Generators, metadata: &PackMetadata, target: Arc<dyn BuildTarget>) {
        for generator in generators {
            generator.build_manifest(
                target.clone(),
                metadata,
            )
        }
    }

    pub fn try_generate_uuid(&self) {
        let config_path = self.cwd().join("cache.woah");
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

    pub fn build_items(&self, generators: &Generators, metadata: &PackMetadata, target: Arc<dyn BuildTarget>) {
        let items = REGISTRY.items.read().unwrap().clone();

        for generator in generators {
            generator.build_items(target.clone(), items.clone(), metadata)
        }

        self.build_client_items(generators, metadata, target);
    }
    
    pub fn build_client_items(&self, generators: &Generators, metadata: &PackMetadata, target: Arc<dyn BuildTarget>) {
        let items = REGISTRY.item_textures.read().unwrap().clone();

        for generator in generators {
            generator.build_client_items(target.clone(), items.clone(), metadata);
        }
    }
    
    pub fn build_blocks(&self, generators: &Generators, metadata: &PackMetadata, target: Arc<dyn BuildTarget>) {
        let blocks = REGISTRY.blocks.read().unwrap().clone();
        
        for generator in generators {
            generator.build_blocks(target.clone(), blocks.clone(), metadata)
        }

        self.build_block_textures(generators, metadata, target);
    }

    pub fn build_block_textures(&self, generators: &Generators, metadata: &PackMetadata, target: Arc<dyn BuildTarget>) {
        let blocks = REGISTRY.block_textures.read().unwrap().clone();

        for generator in generators {
            generator.build_client_blocks(target.clone(), blocks.clone(), metadata);
        }
    }

    pub fn build_entities(&self, generators: &Generators, metadata: &PackMetadata, target: Arc<dyn BuildTarget>) {
        let entities = REGISTRY.entities.read().unwrap().clone();

        for generator in generators {
            generator.build_entities(target.clone(), entities.clone(), metadata);
        }
    }
}