extern crate stick_solo;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
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
            width: 500.0,
            height: 500.0,
            canvas: Some("#interactive_example".to_string()),
            fit_canvas_to_parent: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .insert_resource(SwitchableNR::new(
            Vec2::new(0.0, 0.0),
            &[32.; 6],
            &[0.0; 6],
            &[(-inf, inf); 6],
            Side::Left,
        ))
        .add_startup_system(init)
        .add_system(control)
        .add_system(place_goal)
        .add_system(flush_transforms)
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
                .add(Mesh::from(shape::Quad::new(Vec2::new(5., 5.))))
                .into(),
            material: materials.add(Color::BLUE.into()),
            ..default()
        })
        .insert(Vertex(0));
    for i in 0..n {
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(5., 5.))))
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
}

fn place_goal(
    mouse_button_input: Res<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
    mut goal_query: Query<(&Goal, &mut Transform)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.primary_mut();
        if let Some(cursor) = window.physical_cursor_position() {
            let w = window.physical_width();
            let h = window.physical_height();
            let (x_hat, y_hat) = (
                cursor.x as f32 - w as f32 / 2.0,
                cursor.y as f32 - h as f32 / 2.0,
            );
            info!("{:?}", (w, h, cursor.x, cursor.y));
            info!("{:?}", (x_hat, y_hat));
            let scale_factor = window.scale_factor() as f32;
            let (_, mut transform) = goal_query.single_mut();
            transform.translation.x = x_hat / scale_factor;
            transform.translation.y = y_hat / scale_factor;
        }
    }
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

fn flush_transforms(
    agent: Res<SwitchableNR>,
    mut transforms_query: Query<&mut Transform>,
    mut edge_query: Query<(Entity, &Edge)>,
    mut vertex_query: Query<(Entity, &Vertex)>,
    mut com_query: Query<(Entity, &CenterOfMass)>,
) {
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
}
