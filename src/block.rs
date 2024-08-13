use bevy::prelude::*;

pub enum BlockType {
    Air,
    Solid,
}

#[derive(Component)]
pub struct Block {
    pub block_type: BlockType,
}
