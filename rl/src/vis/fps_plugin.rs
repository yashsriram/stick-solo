use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FPSPlugin;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(init_fps.system())
            .add_system(fps_update_system.system());
    }
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
        if let Some(fps) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.average())
        {
            if let Some(frame_count) = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                .and_then(|frame_count| frame_count.average())
            {
                text.value = format!("FRAME: {}, FPS: {:.2}", frame_count, fps,);
            }
        }
    }
}
