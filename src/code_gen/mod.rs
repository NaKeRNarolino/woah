use crate::core::core_registry::REGISTRY;
use crate::core::metadata::{AddonBp, AddonRp};
use crate::core::utilities::{JsonFormat, SerializeVec};
use crate::core::Serializable;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;
use tera::Tera;
use uuid::Uuid;
use template_encoder::template_encoder;

pub struct CodeGen {
    output_path: RwLock<PathBuf>
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
        output_path: RwLock::new(PathBuf::new())
    };
}

impl CodeGen {
    pub fn set_output_path(&self, path: PathBuf) {
        *self.output_path.write().unwrap() = path;
    }

    pub fn output_path(&self) -> PathBuf {
        (&*self.output_path.read().unwrap()).clone()
    }

    pub fn build(&self) -> anyhow::Result<()> {
        fs::create_dir_all(&*self.output_path.read().unwrap().join("BP"))?;
        fs::create_dir_all(&*self.output_path.read().unwrap().join("RP"))?;

        self.build_manifest();

        self.build_items();
        
        self.build_blocks();

        Ok(())
    }

    pub fn build_manifest(&self) {
        fs::write(self.output_path().join("BP/manifest.json"), &AddonBp.serialize().json_format()).unwrap();
        fs::write(self.output_path().join("RP/manifest.json"), &AddonRp.serialize().json_format()).unwrap();
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

    pub fn build_items(&self) {
        let items = REGISTRY.items.read().unwrap().clone();

        let output = self.output_path();

        fs::create_dir_all(&output.join("BP/items")).unwrap();

        for item in items {
            let path = output.join(format!("BP/items/{}.json", &item.id.render_underscore()));

            fs::write(path, item.serialize().json_format()).unwrap()
        }
        
        self.build_client_items();
    }
    
    pub fn build_client_items(&self) {
        let items = REGISTRY.item_textures.read().unwrap().clone();
        
        let item_textures_path = self.output_path().join(format!("RP/textures/items/{}", &REGISTRY.addon_metadata.read().unwrap().name));
        
        fs::create_dir_all(&item_textures_path).unwrap();
        
        for item in &items {
            let file_path = item_textures_path.join(
                format!("{}.png", &item.id.render_underscore())
            );
            
            item.sprite.build(file_path)
        }
        
        let item_texture_json_path = self.output_path().join("RP/textures/item_texture.json");
        
        let mut c = tera::Context::new();
        
        c.insert("name", &REGISTRY.addon_metadata.read().unwrap().name);
        c.insert("contents", &items.into_iter().map(|x| x.serialize()).collect::<Vec<String>>().join(","));
        
        let temp = TEMPLATES.render("items/item_texture.json", &c).unwrap();
        
        fs::write(item_texture_json_path, temp.json_format()).unwrap();
    }
    
    pub fn build_blocks(&self) {
        let blocks = REGISTRY.blocks.read().unwrap().clone();
        
        fs::create_dir_all(self.output_path().join("BP/blocks")).unwrap();
        
        for block in blocks {
            fs::write(self.output_path().join(
                format!("BP/blocks/{}.json", &block.id.render_underscore())
            ), block.serialize().json_format()).unwrap()
        }

        self.build_block_textures();
    }

    pub fn build_block_textures(&self) {
        let blocks = REGISTRY.block_textures.read().unwrap().clone();

        fs::create_dir_all(&self.output_path().join(format!("RP/textures/block/{}", &REGISTRY.addon_metadata.read().unwrap().name))).unwrap();

        for texture in &blocks {
            let path = &self.output_path().join(format!("RP/textures/block/{}/{}.png", &REGISTRY.addon_metadata.read().unwrap().name, &texture.id.render_underscore()));

            texture.sprite.build(path);
        }

        let block_texture_json_path = self.output_path().join("RP/textures/terrain_texture.json");

        let mut c = tera::Context::new();

        c.insert("name", &REGISTRY.addon_metadata.read().unwrap().name);
        c.insert("content", &blocks.serialize_vec().join(","));

        let contents = TEMPLATES.render("block/terrain_texture.json", &c).unwrap();

        fs::write(block_texture_json_path, contents.json_format()).unwrap()
    }
}