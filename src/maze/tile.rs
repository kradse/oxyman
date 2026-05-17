use bevy::prelude::*;
use serde::Deserialize;
use std::convert::TryFrom;

use crate::{
    spritesheet::{
        spritekind::SpriteKind
    },
};

#[derive(Component)]
pub struct Tile {
    _kind: TileKind,
}
impl Tile {
    // Constants
	// Constructors
    pub fn from_kind(_kind: TileKind) -> Self {
        // TODO: This will do for now, but do refactor this as we get more types :)
        Self { 
            _kind,
        }
    }
	// Public functions
	// Private functions
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "u8")]
pub enum TileKind {
    Floor, // 0
    Wall,  // 1
}
impl TileKind {
    pub fn sprite_kind(kind: TileKind) -> SpriteKind {
        match kind {
            TileKind::Floor => SpriteKind::Air,
            TileKind::Wall => SpriteKind::Dirt,
        }
    }
}
impl TryFrom<u8> for TileKind {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TileKind::Floor),
            1 => Ok(TileKind::Wall),
            n => Err(format!("unknown tile-value: {n}")),
        }
    }
}