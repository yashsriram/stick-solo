use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct Ticks(pub usize);

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Ticks(0))
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(init_fps_vis)
            .add_system(fps_update_system);
    }
}

fn init_fps_vis(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "FPS:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        );
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
            text.sections[0].value = format!("TICKS: {}, FPS: {:.2}", ticks.0, fps,);
        }
    }
}
