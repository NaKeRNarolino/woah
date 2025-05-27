use crate::code_gen::TEMPLATES;
use crate::core::core_registry::REGISTRY;
use crate::core::Serializable;
use crate::core::sprite::Sprite;
use crate::core::utilities::Identifier;

#[derive(Clone)]
pub struct ItemTexture {
    pub id: Identifier,
    pub sprite: Sprite
}

impl ItemTexture {
    pub fn new(id: Identifier, sprite: Sprite) -> Self {
        Self { id, sprite }
    }
}

impl Serializable for ItemTexture {
    fn serialize(&self) -> String {
        let mut c = tera::Context::new();
        
        let texture_path = format!("textures/items/{}/{}.png", &REGISTRY.addon_metadata.read().unwrap().name, &self.id.render_underscore());
        
        c.insert("texture_path", &texture_path);
        c.insert("id", &self.id.render());
        
        TEMPLATES.render("items/item_texture_entry.json", &c).unwrap()
    }
}