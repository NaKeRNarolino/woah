use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use log::info;
use crate::block::Block;
use crate::block::client::BlockTexture;
use crate::code_gen::generator::PackGenerator;
use crate::code_gen::TEMPLATES;
use crate::core::metadata::PackMetadata;
use crate::bedrock::BedrockSerializable;
use crate::bedrock::metadata::{AddonBp, AddonRp, BedrockPath, BedrockPathResolver};
use crate::core::build_target::BuildTarget;
use crate::core::utilities::{BedrockSerializeVec, JsonFormat};
use crate::entity::Entity;
use crate::item::client::ItemTexture;
use crate::item::Item;

/// The default Bedrock generator of Woah, it's integrated with the framework itself.
#[derive(Clone)]
pub struct WoahBedrockGenerator;

impl PackGenerator for WoahBedrockGenerator {
    fn build_prepare(&self, target: Arc<dyn BuildTarget>, metadata: &PackMetadata) {
        fs::create_dir_all(&metadata.bedrock_path(BedrockPath::BPRoot, &target)).unwrap();
        fs::create_dir_all(&metadata.bedrock_path(BedrockPath::RPRoot, &target)).unwrap();
        info!("path: {}", &metadata.bedrock_path(BedrockPath::BPRoot, &target).to_str().unwrap());
    }

    fn build_manifest(&self, target: Arc<dyn BuildTarget>, metadata: &PackMetadata) {
        fs::write(metadata.bedrock_path(
            BedrockPath::bp("manifest.json"), &target
        ), &AddonBp.bedrock_serialize().json_format()).unwrap();
        fs::write(metadata.bedrock_path(
            BedrockPath::rp("manifest.json"), 
            &target
        ), &AddonRp.bedrock_serialize().json_format()).unwrap();
    }

    fn build_items(&self, target: Arc<dyn BuildTarget>, items: Vec<Item>, metadata: &PackMetadata) {
        fs::create_dir_all(&metadata.bedrock_path(BedrockPath::bp("items"), &target)).unwrap();

        for item in items {
            let path = metadata.bedrock_path(BedrockPath::bp(format!("items/{}.json", &item.id.render_underscore())), &target);

            fs::write(path, item.bedrock_serialize().json_format()).unwrap()
        }
    }

    fn build_client_items(&self, target: Arc<dyn BuildTarget>, items: Vec<ItemTexture>, metadata: &PackMetadata) {
        let item_textures_path = metadata.bedrock_path(BedrockPath::rp(format!("textures/items/{}", metadata.name)), &target);

        fs::create_dir_all(&item_textures_path).unwrap();

        for item in &items {
            let file_path = item_textures_path.join(
                format!("{}.png", &item.id.render_underscore())
            );

            item.sprite.build(file_path)
        }

        let item_texture_json_path = metadata.bedrock_path(BedrockPath::rp("textures/item_texture.json"), &target);

        let mut c = tera::Context::new();

        c.insert("name", &metadata.name);
        c.insert("contents", &items.into_iter().map(|x| x.bedrock_serialize()).collect::<Vec<String>>().join(","));

        let temp = TEMPLATES.render("items/item_texture.json", &c).unwrap();

        fs::write(item_texture_json_path, temp.json_format()).unwrap();
    }

    fn build_blocks(&self, target: Arc<dyn BuildTarget>, blocks: Vec<Block>, metadata: &PackMetadata) {
        fs::create_dir_all(metadata.bedrock_path(BedrockPath::bp("blocks"), &target)).unwrap();

        for block in blocks {
            fs::write(metadata.bedrock_path(
                BedrockPath::bp(format!("blocks/{}.json", &block.id.render_underscore())), &target
            ), block.bedrock_serialize().json_format()).unwrap()
        }
    }

    fn build_client_blocks(&self, target: Arc<dyn BuildTarget>, blocks: Vec<BlockTexture>, metadata: &PackMetadata) {
        fs::create_dir_all(metadata.bedrock_path(BedrockPath::rp(format!("textures/block/{}", metadata.name)), &target)).unwrap();

        for texture in &blocks {
            let path = &metadata.bedrock_path(BedrockPath::rp(format!("textures/block/{}/{}.png", metadata.name, &texture.id.render_underscore())), &target);

            texture.sprite.build(path);
        }

        let block_texture_json_path = metadata.bedrock_path(BedrockPath::rp("textures/terrain_texture.json"), &target);

        let mut c = tera::Context::new();

        c.insert("name", &metadata.name);
        c.insert("content", &blocks.serialize_vec().join(","));

        let contents = TEMPLATES.render("block/terrain_texture.json", &c).unwrap();

        fs::write(block_texture_json_path, contents.json_format()).unwrap()
    }

    fn build_entities(&self, target: Arc<dyn BuildTarget>, entities: Vec<Entity>, metadata: &PackMetadata) {
        fs::create_dir_all(metadata.bedrock_path(BedrockPath::bp(format!("entities/{}", metadata.name)), &target)).unwrap();

        dbg!(&entities);
        for entity in entities {
            let path = metadata.bedrock_path(
                BedrockPath::bp(format!("entities/{}/{}.json", metadata.name, entity.id.render_underscore())), &target
            );

            let ser = entity.bedrock_serialize();

            fs::write(path, ser.json_format()).unwrap()
        }
    }
}