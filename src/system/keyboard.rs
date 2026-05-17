use bevy::prelude::*;

use crate::{
    character::crab::Crab,
};

pub struct KeyboardPlugin;
impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_crab);
    }
}

fn move_crab(
    mut query: Query<(&mut Transform, &mut Crab)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut crab)) = query.single_mut() else
        { return; };

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        crab.direction = Dir2::NORTH;
        crab.speed = 50.;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        crab.direction = Dir2::EAST;
        crab.speed = 50.;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        crab.direction = Dir2::SOUTH;
        crab.speed = 50.;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        crab.direction = Dir2::WEST;
        crab.speed = 50.;
    }

    match crab.direction {
        Dir2::NORTH => {
            transform.translation.y += crab.speed * time.delta_secs();
        },
        Dir2::EAST => {
            transform.translation.x += crab.speed * time.delta_secs();
        },
        Dir2::SOUTH => {
            transform.translation.y -= crab.speed * time.delta_secs();
        },
        Dir2::WEST => {
            transform.translation.x -= crab.speed * time.delta_secs();
        },
        _ => panic!("Invalid direction")
    }
}
