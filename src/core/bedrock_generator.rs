use std::fs;
use std::path::PathBuf;
use log::Metadata;
use crate::block::Block;
use crate::block::client::BlockTexture;
use crate::code_gen::generator::PackGenerator;
use crate::code_gen::TEMPLATES;
use crate::core::core_registry::REGISTRY;
use crate::core::metadata::{AddonBp, AddonMetadata, AddonRp};
use crate::core::Serializable;
use crate::core::utilities::{JsonFormat, SerializeVec};
use crate::item::client::ItemTexture;
use crate::item::Item;

/// The default Bedrock generator of Woah, it's tightly integrated with the framework itself.
#[derive(Clone)]
pub struct WoahBedrockGenerator;

impl PackGenerator for WoahBedrockGenerator {
    fn build(&self, output_path: PathBuf) {
        fs::create_dir_all(&output_path.join("BP")).unwrap();
        fs::create_dir_all(&output_path.join("RP")).unwrap();
    }

    fn build_manifest(&self, output_path: PathBuf, metadata: &AddonMetadata) {
        fs::write(output_path.join("BP/manifest.json"), &AddonBp.serialize().json_format()).unwrap();
        fs::write(output_path.join("RP/manifest.json"), &AddonRp.serialize().json_format()).unwrap();
    }

    fn build_items(&self, output_path: PathBuf, items: Vec<Item>, metadata: &AddonMetadata) {
        fs::create_dir_all(&output_path.join("BP/items")).unwrap();

        for item in items {
            let path = output_path.join(format!("BP/items/{}.json", &item.id.render_underscore()));

            fs::write(path, item.serialize().json_format()).unwrap()
        }
    }

    fn build_client_items(&self, output_path: PathBuf, items: Vec<ItemTexture>, metadata: &AddonMetadata) {
        let item_textures_path = output_path.join(format!("RP/textures/items/{}", metadata.name));

        fs::create_dir_all(&item_textures_path).unwrap();

        for item in &items {
            let file_path = item_textures_path.join(
                format!("{}.png", &item.id.render_underscore())
            );

            item.sprite.build(file_path)
        }

        let item_texture_json_path = output_path.join("RP/textures/item_texture.json");

        let mut c = tera::Context::new();

        c.insert("name", &metadata.name);
        c.insert("contents", &items.into_iter().map(|x| x.serialize()).collect::<Vec<String>>().join(","));

        let temp = TEMPLATES.render("items/item_texture.json", &c).unwrap();

        fs::write(item_texture_json_path, temp.json_format()).unwrap();
    }

    fn build_blocks(&self, output_path: PathBuf, blocks: Vec<Block>, metadata: &AddonMetadata) {
        fs::create_dir_all(output_path.join("BP/blocks")).unwrap();

        for block in blocks {
            fs::write(output_path.join(
                format!("BP/blocks/{}.json", &block.id.render_underscore())
            ), block.serialize().json_format()).unwrap()
        }
    }

    fn build_block_textures(&self, output_path: PathBuf, blocks: Vec<BlockTexture>, metadata: &AddonMetadata) {
        fs::create_dir_all(&output_path.join(format!("RP/textures/block/{}", metadata.name))).unwrap();

        for texture in &blocks {
            let path = &output_path.join(format!("RP/textures/block/{}/{}.png", metadata.name, &texture.id.render_underscore()));

            texture.sprite.build(path);
        }

        let block_texture_json_path = output_path.join("RP/textures/terrain_texture.json");

        let mut c = tera::Context::new();

        c.insert("name", &metadata.name);
        c.insert("content", &blocks.serialize_vec().join(","));

        let contents = TEMPLATES.render("block/terrain_texture.json", &c).unwrap();

        fs::write(block_texture_json_path, contents.json_format()).unwrap()
    }
}