use crate::code_gen::TEMPLATES;
use crate::core::Serializable;

/// An enum describing Block traits.
#[derive(Clone, Debug)]
pub enum BlockTrait {
    PlacementDirection {
        enabled_states: Vec<PlacementDirectionState>,
        y_rotation_offset: u8
    },
    PlacementPosition {
        enabled_states: Vec<PlacementDirectionState>,
    },
}


/// Allowed `enabled_states` for PlacementDirection trait.
#[derive(Clone, Debug)]
pub enum PlacementDirectionState {
    /// Correspondent to `minecraft:cardinal_direction`
    CardinalDirection,
    // Correspondent to `minecraft:facing_direction`
    FacingDirection
}

/// Allowed `enabled_states` for PlacementPosition trait.
#[derive(Clone, Debug)]
pub enum PlacementPositionState {
    /// Correspondent to `minecraft:block_face`
    BlockFace,
    /// Correspondent to `minecraft:vertical_half`
    VerticalHalf
}

impl PlacementDirectionState {
    pub fn render(&self) -> String {
        match &self {
            PlacementDirectionState::CardinalDirection => "minecraft_cardinal_direction",
            PlacementDirectionState::FacingDirection => "minecraft:facing_direction"
        }.to_string()
    }
}

impl PlacementPositionState {
    pub fn render(&self) -> String {
        match &self {
            PlacementPositionState::BlockFace => "minecraft:block_face",
            PlacementPositionState::VerticalHalf => "minecraft:vertical_half",
        }.to_string()
    }
}

impl Serializable for PlacementDirectionState {
    fn serialize(&self) -> String {
        self.render()
    }
}

impl Serializable for PlacementPositionState {
    fn serialize(&self) -> String {
        self.render()
    }
}

impl Serializable for BlockTrait {
    fn serialize(&self) -> String {
        match &self {
            BlockTrait::PlacementDirection { enabled_states, y_rotation_offset } => {
                let ser_states = enabled_states.into_iter().map(|x| format!("\"{}\"", x.serialize())).collect::<Vec<String>>().join(",");
                
                let add = format!(",\"y_rotation_offset\": {}", y_rotation_offset);
                
                let mut c = tera::Context::new();
                
                c.insert("id", "minecraft:placement_direction");
                c.insert("enabled_states", &ser_states);
                c.insert("additional", &add);
                
                TEMPLATES.render("block/block_trait.json", &c).unwrap()
            }
            BlockTrait::PlacementPosition { enabled_states } => {
                let ser_states = enabled_states.into_iter().map(|x| x.serialize()).collect::<Vec<String>>().join(",");
                let mut c = tera::Context::new();

                c.insert("id", "minecraft:placement_position");
                c.insert("enabled_states", &ser_states);
                c.insert("additional", "");

                TEMPLATES.render("block/block_trait.json", &c).unwrap()
            }
        }
    }
}