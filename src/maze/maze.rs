use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

use crate::{maze::tile::{Tile, TileKind}, spritesheet::atlas::{SpriteSheet, load_spritesheet}, system::config::Config};

#[derive(Resource, Deserialize)]
pub struct Maze {
    pub tiles: Vec<Vec<TileKind>>,
    pub starting_position: Vec2,
}
impl Maze {
    	// Constants
    const DEFAULT_CONFIG_PATH: &'static str = "assets/config/103.json";
    
	// Constructors
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;
        let maze: Maze = serde_json::from_str(&json)?;
        Ok(maze)
    }
    
	// Public functions
	// Private functions
}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_maze);
        app.add_systems(Startup, spawn_maze.after(load_spritesheet));
    }
}

fn load_maze(mut commands: Commands) {
    let Ok(maze) = Maze::from_file(Maze::DEFAULT_CONFIG_PATH) else {
        panic!("Unable to find level");
    };
    
    commands.insert_resource(maze);
}

fn spawn_maze(
    mut commands: Commands,
    maze: Res<Maze>,
    config: Res<Config>,
    sprite_sheet: Res<SpriteSheet>,
) {
    for (y, row) in maze.tiles.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            commands.spawn((
                Tile::from_kind(tile),
                Sprite::from_atlas_image(
                    sprite_sheet.get_handle_image(),
                    sprite_sheet.get_sprite(TileKind::sprite_kind(tile))
                ),
                Transform::from_xyz(
                    config.x_pos(x as f32), 
                    config.y_pos(y as f32), 
                    100.
                ).with_scale(Vec3::new(
                    config.scale as f32, 
                    config.scale as f32, 
                    0.)
                ),
            ));
        }
    }
}