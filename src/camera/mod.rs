use bevy::prelude::*;

use crate::system::config::Config;

#[derive(Component)]
pub struct MainCamera;

#[derive(Default)]
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    window: Query<&Window>,
    config: Res<Config>,
) {
    let Ok(window) = window.single() else 
        { return; };

    let (ww, wh) = (
        (window.width() - config.scaled_size_f32()) / 2.0, 
        (window.height() - config.scaled_size_f32()) / 2.0
    );

    *clear_color = ClearColor(Color::srgb_u8(20, 16, 16));
    commands.spawn((
        Msaa::Off,
        MainCamera,
        Transform::from_xyz(ww, -wh, 0.),
        Camera2d::default(),
    ));
}
