extern crate stick_solo;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use stick_solo::act::switchable_nr::*;
use stick_solo::game::{
    path_plugin::{Path, PathPlugin},
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
};
use stick_solo::plan::gradient_descent::*;
use stick_solo::AxesHuggingUnitSquare;

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
        .insert_resource(WindowDescriptor {
            width: 500.0,
            height: 500.0,
            canvas: Some("#interactive_example".to_string()),
            fit_canvas_to_parent: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(Camera2dBundle::default());
        })
        .insert_resource(SwitchableNR::new(
            Vec2::new(0.0, -0.1),
            &[64.; 4],
            &[-2.0, 0.0, 2.0, 0.0],
            &[
                (-inf, inf),
                (0.0, pi * 0.5),
                (-pi * 0.5, pi),
                (0.0, pi * 0.5),
            ],
            Side::Left,
        ))
        .add_plugin(PathPlugin::new(Path::default()))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_startup_system(
            |mut commands: Commands,
             agent: Res<SwitchableNR>,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<ColorMaterial>>| {
                let (n, _, ls, _, _, _) = agent.get_current_state();
                // Edges
                for i in 0..n {
                    commands
                        .spawn_bundle(MaterialMesh2dBundle {
                            mesh: meshes.add(Mesh::from(AxesHuggingUnitSquare)).into(),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::default().with_scale(Vec3::new(ls[i], 10., 1.0)),
                            ..default()
                        })
                        .insert(Edge(i));
                }
                // Vertices
                commands
                    .spawn_bundle(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Quad::new(Vec2::new(5.0, 5.0))))
                            .into(),
                        material: materials.add(Color::BLUE.into()),
                        ..default()
                    })
                    .insert(Vertex(0));
                for i in 0..n {
                    commands
                        .spawn_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Quad::new(Vec2::new(5.0, 5.0))))
                                .into(),
                            material: materials.add(Color::BLUE.into()),
                            ..default()
                        })
                        .insert(Vertex(i + 1));
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

fn control(
    mut agent: ResMut<SwitchableNR>,
    mut path: ResMut<Path>,
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
    let (_, origin, ls, qs, _, pivoting_side) = agent.get_current_state();
    let given_goal = path.0.front().unwrap().clone();
    let have_to_match = match pivoting_side {
        Side::Left => given_goal[0] - origin[0] < -SwitchableNR::GOAL_REACHED_SLACK,
        Side::Right => given_goal[0] - origin[0] > SwitchableNR::GOAL_REACHED_SLACK,
    };
    if have_to_match {
        path.0.push_front(origin.clone());
        return;
    }
    let last = agent.get_last_vertex();
    if (given_goal - last).length() < SwitchableNR::GOAL_REACHED_SLACK {
        agent.switch_pivot();
        path.0.pop_front();
        return;
    }

    let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) = gradient_descent(
        origin,
        ls,
        qs,
        &given_goal,
        EndControl::JacobianTranspose,
        COMXGoalType::PivotGoalMidpoint,
    );

    fn downward_push_coeff(com: &Vec2, origin: Vec2) -> f32 {
        let diff_y = com[1] - origin[1];
        if diff_y < 0.0 {
            0.0
        } else {
            0.1 * diff_y.abs()
        }
    }
    let beta = 0.03 / take_end_to_given_goal.mapv(|e| e * e).sum().sqrt();
    let com = agent.get_center_of_mass();
    let origin = origin.clone();
    agent.update(
        beta * take_end_to_given_goal
            + -0.2 * push_com_x_from_its_goal
            + -downward_push_coeff(&com, origin) * push_com_y_upward,
    );

    ticks.0 += 1;
}
