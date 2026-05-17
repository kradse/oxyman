#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SpriteKind {
    // Row 0 - Floors
    Air,
    Void,
    Lava,
    Water,
    BedRock,

    // Row 1 - Walls
    Dirt,

    // Row 2 - Birds
    SpaceBird,
    BasicBird,

    // Row 3 - Crabs
    BasicCrab,
}

impl SpriteKind {
    pub fn get_index(&self) -> usize {
        match self {
            // Row 0 - Floors
            SpriteKind::Air => 0,
            SpriteKind::Void => 1,
            SpriteKind::Lava => 3,
            SpriteKind::Water => 4,
            SpriteKind::BedRock => 2,

            // Row 1 - Walls
            SpriteKind::Dirt => 10,

            // Row 2 - Birds
            SpriteKind::SpaceBird => 20,
            SpriteKind::BasicBird => 21,

            // Row 3 - Crabs
            SpriteKind::BasicCrab => 30,

        }
    } 
}