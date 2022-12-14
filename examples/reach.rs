extern crate stick_solo;
use bevy::asset::AssetServerSettings;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use ndarray::{Array, Array1};
use stick_solo::act::switchable_nr::{Side, SwitchableNR};
use stick_solo::game::{
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
};
use stick_solo::plan::gradient_descent::*;
use stick_solo::plan::random_sampling::{
    from_current_state_random_sample_optimizer, no_prior_random_sample_optimizer,
};
use stick_solo::AxesHuggingUnitSquare;

#[derive(Component)]
struct Goal;
#[derive(Component)]
struct Edge(usize);
#[derive(Component)]
struct CenterOfMass;

struct NPRSGoalQs(Array1<f32>);
struct CSRSGoalQs(Array1<f32>);

enum Algorithm {
    GradientDescent,
    NoPriorRandomSampleWithGradientDescent,
    CurrentStateRandomSampleWithGradientDescent,
}

fn main() {
    let inf = f32::INFINITY;
    let pi = std::f32::consts::PI;
    let mut app = App::new();
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.insert_resource(AssetServerSettings {
            asset_folder: "static/assets".to_string(),
            ..default()
        });
    }
    app.insert_resource(WindowDescriptor {
        width: 500.,
        height: 500.,
        canvas: Some("#interactive_example".to_string()),
        fit_canvas_to_parent: true,
        ..default()
    })
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins)
    .add_plugin(StatusBarPlugin)
    .add_plugin(PausePlugin)
    .insert_resource(SwitchableNR::new(
        Vec2::new(0., 0.),
        &[64.; 4],
        &[0.; 4],
        &[
            (-inf, inf),
            (0.0, pi * 0.5),
            (-pi * 0.5, pi),
            (0.0, pi * 0.5),
        ],
        Side::Left,
    ))
    .insert_resource(NPRSGoalQs(Array::zeros(4)))
    .insert_resource(CSRSGoalQs(Array::zeros(4)))
    .insert_resource(Algorithm::GradientDescent)
    .add_startup_system(init)
    .add_system(place_goal)
    .add_system(control)
    .add_system(flush_transforms)
    .add_system(change_algorithm)
    .run();
}

fn init(
    mut commands: Commands,
    agent: Res<SwitchableNR>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(8., 8.))))
                .into(),
            material: materials.add(Color::GREEN.into()),
            ..default()
        })
        .insert(Goal);
    let (n, _, ls, _, _, _) = agent.get_current_state();
    // Edges
    for i in 0..n {
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(AxesHuggingUnitSquare { width: 15. }))
                    .into(),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::default().with_scale(Vec3::new(ls[i], 1., 1.)),
                ..default()
            })
            .insert(Edge(i));
    }
    // Center of mass
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(5., 5.))))
                .into(),
            material: materials.add(Color::RED.into()),
            ..default()
        })
        .insert(CenterOfMass);
}

fn place_goal(
    mouse_button_input: Res<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
    agent: Res<SwitchableNR>,
    mut nprs_goal_qs: ResMut<NPRSGoalQs>,
    mut csrs_goal_qs: ResMut<CSRSGoalQs>,
    mut transforms: Query<&mut Transform>,
    goal: Query<(Entity, &Goal)>,
    mut ticks: ResMut<Ticks>,
    algorithm: Res<Algorithm>,
) {
    let (goal, _) = goal.single();
    let mut goal_transform = transforms.get_mut(goal).unwrap();
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.primary_mut();
        if let Some(cursor) = window.physical_cursor_position() {
            let w = window.physical_width();
            let h = window.physical_height();
            let (x_hat, y_hat) = (
                cursor.x as f32 - w as f32 / 2.,
                cursor.y as f32 - h as f32 / 2.,
            );
            info!("{:?}", (w, h, cursor.x, cursor.y));
            info!("{:?}", (x_hat, y_hat));
            let scale_factor = window.scale_factor() as f32;
            goal_transform.translation.x = x_hat / scale_factor;
            goal_transform.translation.y = y_hat / scale_factor;

            let (n, origin, ls, qs, q_clamps, pivoting_side) = agent.get_current_state();

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
            nprs_goal_qs.0 = best_q;

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
            csrs_goal_qs.0 = best_q;

            ticks.0 = 0;
        }
    }
}

fn control(
    mut agent: ResMut<SwitchableNR>,
    pause: Res<Pause>,
    transforms: Query<&mut Transform>,
    goal: Query<(Entity, &Goal)>,
    mut ticks: ResMut<Ticks>,
    nprs_goal_qs: Res<NPRSGoalQs>,
    csrs_goal_qs: Res<CSRSGoalQs>,
    algorithm: Res<Algorithm>,
) {
    // Pause => pause everything
    if pause.0 {
        return;
    }
    let (_, origin, ls, qs, _, _) = agent.get_current_state();

    let (goal, _) = goal.single();
    let goal_transform = transforms.get(goal).unwrap();
    let given_goal = Vec2::new(goal_transform.translation.x, goal_transform.translation.y);

    let (take_end_to_given_goal, push_com_x_from_its_goal, _) = gradient_descent(
        origin,
        ls,
        qs,
        &given_goal,
        EndControl::JacobianTranspose,
        COMXGoalType::PivotGoalMidpoint,
    );

    match *algorithm {
        Algorithm::GradientDescent => {
            agent.update(take_end_to_given_goal + -0.2 * push_com_x_from_its_goal);
        }
        Algorithm::NoPriorRandomSampleWithGradientDescent => {
            let alpha = 1.0 / (1.0 + ticks.0 as f32).powf(0.5);
            let beta = 1.0 - alpha;
            let gamma = 0.1;
            let delta = 0.1 / (1.0 + ticks.0 as f32).powf(1.0);

            let global_delta_qs = &nprs_goal_qs.0 - qs;
            agent.update(
                global_delta_qs, // + beta * take_end_to_given_goal
                                 // + gamma * -push_com_x_from_its_goal
                                 // + delta * -push_com_y_upward,
            );
        }
        Algorithm::CurrentStateRandomSampleWithGradientDescent => {
            let alpha = 1.0 / (1.0 + ticks.0 as f32).powf(0.5);
            let beta = 1.0 - alpha;
            let gamma = 0.1;
            let delta = 0.1 / (1.0 + ticks.0 as f32).powf(1.0);

            let global_delta_qs = &csrs_goal_qs.0 - qs;
            agent.update(
                global_delta_qs, // + beta * take_end_to_given_goal
                                 // + gamma * -push_com_x_from_its_goal
                                 // + delta * -push_com_y_upward,
            );
        }
    }

    ticks.0 += 1;
}

fn flush_transforms(
    agent: Res<SwitchableNR>,
    mut transforms_query: Query<&mut Transform>,
    mut edge_query: Query<(Entity, &Edge)>,
    mut com_query: Query<(Entity, &CenterOfMass)>,
) {
    let transforms = agent.pose_to_transforms();
    for (entity, edge) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        let mut transform = transforms_query.get_mut(entity).unwrap();
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.rotation = Quat::from_rotation_z(angle);
    }
    let com = agent.get_center_of_mass();
    for (entity, _) in com_query.iter_mut() {
        let mut transform = transforms_query.get_mut(entity).unwrap();
        transform.translation[0] = com[0];
        transform.translation[1] = com[1];
    }
}

fn change_algorithm(input: Res<Input<KeyCode>>, mut algorithm: ResMut<Algorithm>) {
    if input.just_pressed(KeyCode::Key1) {
        *algorithm = Algorithm::GradientDescent;
    }
    if input.just_pressed(KeyCode::Key2) {
        *algorithm = Algorithm::NoPriorRandomSampleWithGradientDescent;
    }
    if input.just_pressed(KeyCode::Key3) {
        *algorithm = Algorithm::CurrentStateRandomSampleWithGradientDescent;
    }
}
