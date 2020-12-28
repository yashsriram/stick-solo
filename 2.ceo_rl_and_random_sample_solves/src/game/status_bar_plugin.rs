use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct Ticks(pub usize);

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Ticks(0))
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(init_fps_vis.system())
            .add_system(fps_update_system.system());
    }
}

fn init_fps_vis(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn fps_update_system(
    diagnostics: Res<Diagnostics>,
    ticks: Res<Ticks>,
    mut query: Query<&mut Text>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.average())
        {
            text.value = format!("TICKS: {}, FPS: {:.2}", ticks.0, fps,);
        }
    }
}
