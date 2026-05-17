use crate::code_gen::TEMPLATES;
use crate::core::core_registry::REGISTRY;
use crate::bedrock::BedrockSerializable;
use crate::core::sprite::Sprite;
use crate::core::utilities::Identifier;

/// A struct for describing Item textures.
#[derive(Clone, Debug)]
pub struct ItemTexture {
    pub id: Identifier,
    pub sprite: Sprite
}

impl ItemTexture {
    pub fn new(id: Identifier, sprite: Sprite) -> Self {
        Self { id, sprite }
    }
}

impl BedrockSerializable for ItemTexture {
    fn bedrock_serialize(&self) -> String {
        let mut c = tera::Context::new();
        
        let texture_path = format!("textures/items/{}/{}.png", &REGISTRY.pack_metadata.read().unwrap().name, &self.id.render_underscore());
        
        c.insert("texture_path", &texture_path);
        c.insert("id", &self.id.render());
        
        TEMPLATES.render("generic/atlas.json", &c).unwrap()
    }
}