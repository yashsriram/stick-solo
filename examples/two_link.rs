use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;
use stick_solo::game::status_bar_plugin::StatusBarPlugin;
use stick_solo::game::viz::SimpleMaterial;
use stick_solo::AxesHuggingUnitSquare;

#[derive(Component)]
struct Goal;
#[derive(Component)]
struct Link(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(MaterialPlugin::<SimpleMaterial>::default())
        .add_startup_system(
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<SimpleMaterial>>| {
                commands.spawn_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                });
                let thickness = 0.04;
                let lengths = [0.435, 0.7324];
                let thetas = [0.0, FRAC_PI_2];
                let mut origin = Vec3::ZERO;
                for (idx, (len, theta)) in lengths
                    .into_iter()
                    .zip(thetas.into_iter())
                    .into_iter()
                    .enumerate()
                {
                    commands
                        .spawn_bundle(MaterialMeshBundle {
                            mesh: meshes.add(Mesh::from(AxesHuggingUnitSquare)),
                            transform: Transform::default()
                                .with_scale(Vec3::new(len, thickness, 1.0))
                                .with_rotation(Quat::from_rotation_z(theta))
                                .with_translation(origin),
                            material: materials.add(SimpleMaterial {}),
                            ..default()
                        })
                        .insert(Link(idx));
                    origin += Quat::from_rotation_z(theta) * (Vec3::X * len);
                }
                commands
                    .spawn_bundle(MaterialMeshBundle {
                        mesh: meshes.add(Mesh::from(AxesHuggingUnitSquare)),
                        transform: Transform::default()
                            .with_scale(Vec3::new(thickness, thickness, 1.0)),
                        material: materials.add(SimpleMaterial {}),
                        ..default()
                    })
                    .insert(Goal);
            },
        )
        .add_system(
            |keyboard_input: Res<Input<KeyCode>>,
             mut goal_query: Query<(&Goal, &mut Transform)>| {
                let (_, mut transform) = goal_query.single_mut();
                if keyboard_input.pressed(KeyCode::W) {
                    transform.translation.y += 0.01;
                } else if keyboard_input.pressed(KeyCode::S) {
                    transform.translation.y -= 0.01;
                } else if keyboard_input.pressed(KeyCode::A) {
                    transform.translation.x -= 0.01;
                } else if keyboard_input.pressed(KeyCode::D) {
                    transform.translation.x += 0.01;
                }
            },
        )
        .add_plugin(StatusBarPlugin)
        .run();
}
