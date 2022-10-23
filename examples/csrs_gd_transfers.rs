extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use stick_solo::act::switchable_nr::*;
use stick_solo::game::{
    path_plugin::{Path, PathPlugin},
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
};
use stick_solo::plan::gradient_descent::*;
use stick_solo::plan::random_sampling::*;

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
        .insert_resource(SwitchableNR::new(
            Vec2::new(0.0, -0.1),
            &[0.2, 0.2, 0.2, 0.2],
            &[-2.0, 0.0, 2.0, 0.0],
            &[
                (-inf, inf),
                (0.0, pi * 0.5),
                (-pi * 0.5, pi),
                (0.0, pi * 0.5),
            ],
            Side::Left,
            0.01,
        ))
        .insert_resource(GoalQs(Array::zeros(4)))
        .add_plugin(PathPlugin::new(Path::default()))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_startup_system(set_first_goal)
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
                            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)))),
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
        .add_system(control)
        .run();
}

struct GoalQs(Array1<f32>);

fn set_first_goal(agent: ResMut<SwitchableNR>, path: ResMut<Path>, mut goal_qs: ResMut<GoalQs>) {
    let (n, origin, ls, qs, q_clamps, pivoting_side) = agent.get_current_state();
    let loss_fn = |end: &Vec2, com: &Vec2, goal: &Vec2, origin: &Vec2| {
        5.0 * (end.clone() - goal.clone()).length()
            + 5.0 * com[1]
            + (com[0] - (origin[0] + goal[0]) / 2.0).abs()
    };
    let (_min_loss, best_q) = from_current_state_random_sample_optimizer(
        10_000,
        3.0,
        n,
        origin,
        ls,
        qs,
        pivoting_side,
        q_clamps,
        &path.0.front().unwrap().clone(),
        loss_fn,
    );
    goal_qs.0 = best_q;
}

fn control(
    mut agent: ResMut<SwitchableNR>,
    mut path: ResMut<Path>,
    mut goal_qs: ResMut<GoalQs>,
    pause: Res<Pause>,
    mut ticks: ResMut<Ticks>,
) {
    // Pause => pause everything
    if pause.0 {
        return;
    }
    // No more goals => pause everything
    if path.0.is_empty() {
        return;
    }
    let loss_fn = |end: &Vec2, com: &Vec2, goal: &Vec2, origin: &Vec2| {
        5.0 * (end.clone() - goal.clone()).length()
            + 5.0 * com[1]
            + (com[0] - (origin[0] + goal[0]) / 2.0).abs()
    };
    let (_, origin, ls, qs, _, pivoting_side) = agent.get_current_state();
    let given_goal = path.0.front().unwrap().clone();
    let have_to_match = match pivoting_side {
        Side::Left => given_goal[0] - origin[0] < -SwitchableNR::GOAL_REACHED_SLACK,
        Side::Right => given_goal[0] - origin[0] > SwitchableNR::GOAL_REACHED_SLACK,
    };
    if have_to_match {
        path.0.push_front(origin.clone());

        let (n, origin, ls, qs, q_clamps, pivoting_side) = agent.get_current_state();
        let (_min_loss, best_q) = from_current_state_random_sample_optimizer(
            10_000,
            3.0,
            n,
            origin,
            ls,
            qs,
            pivoting_side,
            q_clamps,
            &path.0.front().unwrap().clone(),
            loss_fn,
        );
        goal_qs.0 = best_q;

        return;
    }
    let last = agent.get_last_vertex();
    if (given_goal - last).length() < SwitchableNR::GOAL_REACHED_SLACK {
        agent.switch_pivot();
        path.0.pop_front();

        ticks.0 = 0;

        if path.0.len() > 0 {
            let (n, origin, ls, qs, q_clamps, pivoting_side) = agent.get_current_state();
            let (_min_loss, best_q) = from_current_state_random_sample_optimizer(
                10_000,
                3.0,
                n,
                origin,
                ls,
                qs,
                pivoting_side,
                q_clamps,
                &path.0.front().unwrap().clone(),
                loss_fn,
            );
            goal_qs.0 = best_q;
        }
        return;
    }

    let global_delta_qs = &goal_qs.0 - qs;

    let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) = gradient_descent(
        origin,
        ls,
        qs,
        &given_goal,
        EndControl::JacobianTranspose,
        COMXGoalType::PivotGoalMidpoint,
    );

    let alpha = 1.0 / (1.0 + ticks.0 as f32).powf(0.8);
    let beta = 0.01 / take_end_to_given_goal.mapv(|e| e * e).sum().sqrt();
    let gamma = 0.1;
    let delta = 0.1 / (1.0 + ticks.0 as f32).powf(1.0);
    agent.update(
        alpha * global_delta_qs
            + beta * take_end_to_given_goal
            + gamma * -push_com_x_from_its_goal
            + delta * -push_com_y_upward,
    );

    ticks.0 += 1;
}
