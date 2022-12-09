extern crate stick_solo;
use bevy::prelude::*;
use stick_solo::act::switchable_nr::{Side, SwitchableNR};
use stick_solo::game::{
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
};
use stick_solo::plan::gradient_descent::*;
use stick_solo::AxesHuggingUnitSquare;

#[derive(Component)]
struct Goal;
#[derive(Component)]
struct Edge(usize);
#[derive(Component)]
struct Vertex(usize);
#[derive(Component)]
struct CenterOfMass;

fn main() {
    let inf = f32::INFINITY;
    App::new()
        .insert_resource(WindowDescriptor {
            canvas: Some("#interactive_example".to_string()),
            fit_canvas_to_parent: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
            commands.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(0.0, 0.0, 4.0),
                ..default()
            });
        })
        .add_startup_system(
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>| {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.04, 0.04)))),
                        transform: Transform::default().with_translation(Vec3::new(0.3, 0.4, 0.0)),
                        material: materials.add(Color::GREEN.into()),
                        ..default()
                    })
                    .insert(Goal);
            },
        )
        .insert_resource(SwitchableNR::new(
            Vec2::new(0.0, 0.0),
            &[0.2; 6],
            &[0.0; 6],
            &[(-inf, inf); 6],
            Side::Left,
        ))
        .add_startup_system(
            |mut commands: Commands,
             agent: Res<SwitchableNR>,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>| {
                let (n, _, ls, _, _, _) = agent.get_current_state();
                // Edges
                for i in 0..n {
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(AxesHuggingUnitSquare)),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::default().with_scale(Vec3::new(ls[i], 0.01, 1.0)),
                            ..default()
                        })
                        .insert(Edge(i));
                }
                // Vertices
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.02, 0.02)))),
                        material: materials.add(Color::BLUE.into()),
                        ..default()
                    })
                    .insert(Vertex(0));
                for i in 0..n {
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.02, 0.02)))),
                            material: materials.add(Color::BLUE.into()),
                            ..default()
                        })
                        .insert(Vertex(i + 1));
                }
                // Center of mass
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.04, 0.04)))),
                        material: materials.add(Color::RED.into()),
                        ..default()
                    })
                    .insert(CenterOfMass);
            },
        )
        .add_system(
            |agent: Res<SwitchableNR>,
             mut transforms_query: Query<&mut Transform>,
             mut edge_query: Query<(Entity, &Edge)>,
             mut vertex_query: Query<(Entity, &Vertex)>,
             mut com_query: Query<(Entity, &CenterOfMass)>| {
                let transforms = agent.pose_to_transforms();
                let (_, _, ls, _, _, _) = agent.get_current_state();
                for (entity, edge) in edge_query.iter_mut() {
                    let (midpoint, angle) = transforms[edge.0];
                    let mut transform = transforms_query.get_mut(entity).unwrap();
                    transform.translation[0] = midpoint[0];
                    transform.translation[1] = midpoint[1];
                    transform.scale = Vec3::new(ls[edge.0], 0.01, 1.0);
                    transform.rotation = Quat::from_rotation_z(angle);
                }
                let vertex_positions = agent.get_all_vertices();
                for (entity, idx) in vertex_query.iter_mut() {
                    let mut transform = transforms_query.get_mut(entity).unwrap();
                    transform.translation[0] = vertex_positions[idx.0][0];
                    transform.translation[1] = vertex_positions[idx.0][1];
                }
                let com = agent.get_center_of_mass();
                for (entity, _) in com_query.iter_mut() {
                    let mut transform = transforms_query.get_mut(entity).unwrap();
                    transform.translation[0] = com[0];
                    transform.translation[1] = com[1];
                }
            },
        )
        .add_system(
            |keyboard_input: Res<Input<KeyCode>>,
             mut goal_query: Query<(&Goal, &mut Transform)>| {
                let (_, mut transform) = goal_query.single_mut();
                if keyboard_input.pressed(KeyCode::W) {
                    transform.translation.y += 0.01;
                }
                if keyboard_input.pressed(KeyCode::S) {
                    transform.translation.y -= 0.01;
                }
                if keyboard_input.pressed(KeyCode::A) {
                    transform.translation.x -= 0.01;
                }
                if keyboard_input.pressed(KeyCode::D) {
                    transform.translation.x += 0.01;
                }
            },
        )
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(control)
        .run();
}

fn control(
    mut agent: ResMut<SwitchableNR>,
    pause: Res<Pause>,
    transforms: Query<&mut Transform>,
    goal: Query<(Entity, &Goal)>,
    mut ticks: ResMut<Ticks>,
) {
    // Pause => pause everything
    if pause.0 {
        return;
    }
    let (_, origin, ls, qs, _, _) = agent.get_current_state();
    let (goal, _) = goal.single();
    let goal_transform = transforms.get(goal).unwrap();

    let (take_end_to_given_goal, push_com_x_from_its_goal, _) = gradient_descent(
        origin,
        ls,
        qs,
        &Vec2::new(goal_transform.translation.x, goal_transform.translation.y),
        EndControl::JacobianTranspose,
        COMXGoalType::PivotGoalMidpoint,
    );

    agent.update(take_end_to_given_goal + -0.2 * push_com_x_from_its_goal);

    ticks.0 += 1;
}
