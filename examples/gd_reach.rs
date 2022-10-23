extern crate stick_solo;
use bevy::prelude::*;
use stick_solo::act::switchable_nr::{Side, SwitchableNR};
use stick_solo::game::{
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
    switchable_nr_plugin::SwitchableNRPlugin,
};
use stick_solo::plan::gradient_descent::*;

#[derive(Component)]
struct Goal;

fn main() {
    let inf = f32::INFINITY;
    let pi = std::f32::consts::PI;
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
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
        .add_plugin(SwitchableNRPlugin::new(SwitchableNR::new(
            Vec2::new(0.0, 0.0),
            &[0.2; 6],
            &[0.0; 6],
            &[(-inf, inf); 6],
            Side::Left,
            0.02,
        )))
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
