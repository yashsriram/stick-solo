extern crate sticksolo;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::keyboard::KeyCode,
    prelude::*,
};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use sticksolo::plugins::*;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(base_plugins::BasePlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(init_camera.system())
        .add_startup_system(init_fps.system())
        .add_plugin(nr_agent_plugin::NRAgentPlugin)
        .add_system(fps_update_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn init_camera(mut commands: Commands) {
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

fn init_fps(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextComponents {
        style: Style::default(),
        text: Text {
            value: "FPS:".to_string(),
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            style: TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..Default::default()
            },
        },
        ..Default::default()
    });
}

fn fps_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}
