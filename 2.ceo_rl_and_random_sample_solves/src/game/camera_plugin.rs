use bevy::prelude::*;
// use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(FlyCameraPlugin)
            .add_startup_system(init_camera);
    }
}

pub fn init_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
        // .with(FlyCamera {
        //     key_up: KeyCode::Up,
        //     key_down: KeyCode::Down,
        //     key_left: KeyCode::Left,
        //     key_right: KeyCode::Right,
        //     key_forward: KeyCode::Plus,
        //     key_backward: KeyCode::Minus,
        //     sensitivity: 0.0,
        //     ..Default::default()
        // })
}
