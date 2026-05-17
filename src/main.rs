use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{
            Backends, 
            WgpuSettings,
            RenderCreation, 
        },
    },
};

mod camera;
use camera::CameraPlugin;
mod maze;
use maze::maze::MazePlugin;
mod system;
use system::{
    config::ConfigPlugin,
    keyboard::KeyboardPlugin,
};

mod spritesheet;
use spritesheet::atlas::SpriteSheetPlugin;

mod character;
use character::crab::CrabPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "OxyMan".into(),
                    resolution: (480, 360).into(), // Native resolution (4:3)
                    // resolution: (1280, 720).into(), // 720p (16:9)
                    // resolution: (1920, 1080).into(), // 1080p (16:9)
                    // resolution: (2560, 1440).into(), // 1440p (16:9)
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(
                    Box::new(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }
                )),
                ..default()
            })
        )
        .add_plugins((
            ConfigPlugin,
            CameraPlugin,
            SpriteSheetPlugin,
            MazePlugin,
            CrabPlugin,
            // BirdPlugin,
            KeyboardPlugin,
            // MousePlugin,
        ))
        .run();
}
