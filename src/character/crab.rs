use bevy::prelude::*;

use crate::{
    maze::maze::Maze, spritesheet::{
        atlas::SpriteSheet, 
        spritekind::SpriteKind
    }, system::config::Config
};

#[derive(Component)]
pub struct Crab {
    kind: CrabKind,
    pub speed: f32,
    pub direction: Dir2,
}

impl Crab {
    // Constants
	// Constructors
    pub fn from_kind(kind: CrabKind) -> Self {
        // TODO: This will do for now, but do refactor this as we get more types :)
        Self { 
            kind,
            speed: 0.,
            direction: Dir2::SOUTH,
        }
    }
	// Public functions
	// Private functions
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CrabKind {
    BasicCrab,
}
impl CrabKind {
    pub fn sprite_kind(kind: CrabKind) -> SpriteKind {
        match kind {
            CrabKind::BasicCrab => SpriteKind::BasicCrab,
        }
    }
}

pub struct CrabPlugin;
impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crabs);
    }
}
fn spawn_crabs(
    mut commands: Commands,
    maze: Res<Maze>,
    config: Res<Config>,
    sprite_sheet: Res<SpriteSheet>,
) {
    commands.spawn((
        Crab::from_kind(CrabKind::BasicCrab),
        Sprite::from_atlas_image(
            sprite_sheet.get_handle_image(), 
            sprite_sheet.get_sprite(CrabKind::sprite_kind(CrabKind::BasicCrab))
        ),
        Transform::from_xyz(
            config.x_pos(maze.starting_position.x), 
            config.y_pos(maze.starting_position.y), 
            1000.
        ).with_scale(Vec3::new(
            config.scale as f32, 
            config.scale as f32, 
            0.)
        ),
    ));
}
// fn move_crabs(
//     query: Query<(&mut Transform, &Crab)>,
//     config: Res<Config>,
//     time: Res<Time>,
// ) {
//     let stop_at = -config.scaled_size_f32();

//     for (mut transform, crab) in query {
//         if transform.translation.x > stop_at {
//             transform.translation.x -= crab.speed * time.delta_secs();
//         }
//     }
// }