use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FlyCameraPlugin)
            .add_startup_system(init_camera.system());
    }
}

pub fn init_camera(mut commands: Commands) {
    commands
        .spawn(UiCameraComponents::default())
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
            ..Default::default()
        })
        .with(FlyCamera {
            key_up: KeyCode::Up,
            key_down: KeyCode::Down,
            key_left: KeyCode::Left,
            key_right: KeyCode::Right,
            key_forward: KeyCode::Plus,
            key_backward: KeyCode::Minus,
            sensitivity: 0.0,
            ..Default::default()
        });
}
