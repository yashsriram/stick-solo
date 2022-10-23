extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use stick_solo::act::switchable_nr::{Side, SwitchableNR};
use stick_solo::game::{
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
    switchable_nr_plugin::SwitchableNRPlugin,
};
use stick_solo::plan::gradient_descent::*;
use stick_solo::plan::random_sampling::*;

#[derive(Component)]
struct Goal;

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
        .add_system(
            |keyboard_input: Res<Input<KeyCode>>,
             mut goal_query: Query<(&Goal, &mut Transform)>| {
                for (_, mut transform) in goal_query.iter_mut() {
                    if keyboard_input.pressed(KeyCode::W) {
                        transform.translation.y += 0.01;
                    } else if keyboard_input.pressed(KeyCode::S) {
                        transform.translation.y -= 0.01;
                    } else if keyboard_input.pressed(KeyCode::A) {
                        transform.translation.x -= 0.01;
                    } else if keyboard_input.pressed(KeyCode::D) {
                        transform.translation.x += 0.01;
                    }
                }
            },
        )
        .add_plugin(SwitchableNRPlugin::new(SwitchableNR::new(
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
            0.01,
        )))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(genetic_solve_from_current_state)
        .add_system(interpolate)
        .run();
}

struct GoalQs(Array1<f32>);

fn genetic_solve_from_current_state(
    agent: Res<SwitchableNR>,
    mut goal_qs: ResMut<GoalQs>,
    transforms: Query<&mut Transform>,
    goal: Query<(Entity, &Goal)>,
    mut ticks: ResMut<Ticks>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::W)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::S)
        || keyboard_input.pressed(KeyCode::D)
    {
        let (n, origin, ls, qs, q_clamps, pivoting_side) = agent.get_current_state();
        let (goal, _) = goal.single();
        let goal_transform = transforms.get(goal).unwrap();
        let (_min_loss, best_q) = from_current_state_random_sample_optimizer(
            10_000,
            2.0,
            n,
            origin,
            ls,
            qs,
            pivoting_side,
            q_clamps,
            &Vec2::new(goal_transform.translation.x, goal_transform.translation.y),
            |end: &Vec2, com: &Vec2, goal: &Vec2, origin: &Vec2| {
                5.0 * (end.clone() - goal.clone()).length()
                    + 5.0 * com[1]
                    + (com[0] - (origin[0] + goal[0]) / 2.0).abs()
            },
        );
        // println!("{:?}", min_loss);
        // println!("{:?}", best_q[0]);
        goal_qs.0 = best_q;
        ticks.0 = 0;
    }
}

fn interpolate(
    mut agent: ResMut<SwitchableNR>,
    pause: Res<Pause>,
    transforms: Query<&mut Transform>,
    goal: Query<(Entity, &Goal)>,
    mut ticks: ResMut<Ticks>,
    goal_qs: Res<GoalQs>,
) {
    // Pause => pause everything
    if pause.0 {
        return;
    }
    let (_, origin, ls, qs, _, _) = agent.get_current_state();

    let global_delta_qs = &goal_qs.0 - qs;

    let (goal, _) = goal.single();
    let goal_transform = transforms.get(goal).unwrap();
    let given_goal = Vec2::new(goal_transform.translation.x, goal_transform.translation.y);
    let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) = gradient_descent(
        origin,
        ls,
        qs,
        &given_goal,
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

    ticks.0 += 1;
}
