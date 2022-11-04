extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use stick_solo::act::switchable_nr::{Side, SwitchableNR};
use stick_solo::game::{
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
};
use stick_solo::plan::gradient_descent::*;
use stick_solo::plan::random_sampling::*;
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
    let pi = std::f32::consts::PI;
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(GoalQs(Array::zeros(4)))
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
                        material: materials.add(Color::GREEN.into()),
                        ..default()
                    })
                    .insert(Goal);
            },
        )
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
                        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                            0.01 * 2.0,
                            0.01 * 2.0,
                        )))),
                        material: materials.add(Color::BLUE.into()),
                        ..default()
                    })
                    .insert(Vertex(0));
                for i in 0..n {
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                                0.01 * 2.0,
                                0.01 * 2.0,
                            )))),
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
        .insert_resource(SwitchableNR::new(
            Vec2::new(0.0, 0.1),
            &[0.2, 0.2, 0.2, 0.2],
            // &[-9.0, 0.0, -0.5, -0.5],
            // &[
            //     (-inf, inf),
            //     (-pi * 0.5, 0.0),
            //     (-pi, pi * 0.5),
            //     (-pi * 0.5, 0.0),
            // ],
            // Side::Right,
            &[-7.0, 0.1, 0.5, 0.5],
            &[
                (-inf, inf),
                (0.0, pi * 0.5),
                (-pi * 0.5, pi),
                (0.0, pi * 0.5),
            ],
            Side::Left,
        ))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(genetic_solve_no_prior)
        .run();
}

struct GoalQs(Array1<f32>);

fn genetic_solve_no_prior(
    mut goal_qs: ResMut<GoalQs>,
    mut transforms: Query<&mut Transform>,
    goal: Query<(Entity, &Goal)>,
    mut ticks: ResMut<Ticks>,
    keyboard_input: Res<Input<KeyCode>>,
    mut agent: ResMut<SwitchableNR>,
    pause: Res<Pause>,
    mut edge_query: Query<(Entity, &Edge)>,
    mut vertex_query: Query<(Entity, &Vertex)>,
    mut com_query: Query<(Entity, &CenterOfMass)>,
) {
    let (goal, _) = goal.single();
    let mut goal_transform = transforms.get_mut(goal).unwrap();
    if keyboard_input.pressed(KeyCode::W) {
        goal_transform.translation.y += 0.01;
    } else if keyboard_input.pressed(KeyCode::S) {
        goal_transform.translation.y -= 0.01;
    } else if keyboard_input.pressed(KeyCode::A) {
        goal_transform.translation.x -= 0.01;
    } else if keyboard_input.pressed(KeyCode::D) {
        goal_transform.translation.x += 0.01;
    }
    if keyboard_input.pressed(KeyCode::W)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::S)
        || keyboard_input.pressed(KeyCode::D)
    {
        let (_, origin, ls, qs, q_clamps, pivoting_side) = agent.get_current_state();
        let (_min_loss, best_q) = no_prior_random_sample_optimizer(
            10_000,
            origin,
            ls,
            qs[0],
            pivoting_side,
            q_clamps,
            &Vec2::new(goal_transform.translation.x, goal_transform.translation.y),
            |end: &Vec2, com: &Vec2, goal: &Vec2, origin: &Vec2| {
                5.0 * (end.clone() - goal.clone()).length()
                    + com[1]
                    + (com[0] - (origin[0] + goal[0]) / 2.0).abs()
            },
        );
        // println!("{:?}", min_loss);
        // println!("{:?}", best_q[0]);
        goal_qs.0 = best_q;
        ticks.0 = 0;
    }
    // Pause => pause everything
    if pause.0 {
        return;
    }
    let (_, origin, ls, qs, _, _) = agent.get_current_state();
    let global_delta_qs = &goal_qs.0 - qs;
    let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) = gradient_descent(
        origin,
        ls,
        qs,
        &Vec2::new(goal_transform.translation.x, goal_transform.translation.y),
        EndControl::JacobianTranspose,
        COMXGoalType::PivotGoalMidpoint,
    );

    let alpha = 1.0 / (1.0 + ticks.0 as f32).powf(0.5);
    let beta = 1.0 - alpha;
    let gamma = 0.1;
    let delta = 0.1 / (1.0 + ticks.0 as f32).powf(1.0);
    agent.update(
        alpha * global_delta_qs
            + beta * take_end_to_given_goal
            + gamma * -push_com_x_from_its_goal
            + delta * -push_com_y_upward,
    );

    let current_transforms = agent.pose_to_transforms();
    let (_, _, ls, _, _, _) = agent.get_current_state();
    for (entity, edge) in edge_query.iter_mut() {
        let (midpoint, angle) = current_transforms[edge.0];
        let mut transform = transforms.get_mut(entity).unwrap();
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.scale = Vec3::new(ls[edge.0], 0.01, 1.0);
        transform.rotation = Quat::from_rotation_z(angle);
    }
    let vertex_positions = agent.get_all_vertices();
    for (entity, idx) in vertex_query.iter_mut() {
        let mut transform = transforms.get_mut(entity).unwrap();
        transform.translation[0] = vertex_positions[idx.0][0];
        transform.translation[1] = vertex_positions[idx.0][1];
    }
    let com = agent.get_center_of_mass();
    for (entity, _) in com_query.iter_mut() {
        let mut transform = transforms.get_mut(entity).unwrap();
        transform.translation[0] = com[0];
        transform.translation[1] = com[1];
    }
    ticks.0 += 1;
}
