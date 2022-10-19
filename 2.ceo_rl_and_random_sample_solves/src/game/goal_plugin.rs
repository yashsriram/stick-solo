use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Clone)]
pub struct Goal(pub Vec2);

pub struct GoalPlugin {
    goal: Goal,
}

impl GoalPlugin {
    pub fn new(goal: Goal) -> GoalPlugin {
        GoalPlugin { goal }
    }
}

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.goal.clone())
            .add_startup_system(init_vis)
            .add_system(interactive_goal)
            .add_system(flush_transforms);
    }
}

#[derive(Component)]
struct GoalMarker;

fn init_vis(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.04, 0.04)))),
            material: materials.add(Color::GREEN.into()),
            ..default()
        })
        .insert(GoalMarker);
}

fn interactive_goal(keyboard_input: Res<Input<KeyCode>>, mut goal: ResMut<Goal>) {
    if keyboard_input.pressed(KeyCode::W) {
        goal.0[1] += 0.01;
    } else if keyboard_input.pressed(KeyCode::S) {
        goal.0[1] -= 0.01;
    } else if keyboard_input.pressed(KeyCode::A) {
        goal.0[0] -= 0.01;
    } else if keyboard_input.pressed(KeyCode::D) {
        goal.0[0] += 0.01;
    }
}

fn flush_transforms(goal: Res<Goal>, mut goal_query: Query<(&GoalMarker, &mut Transform)>) {
    for (_, mut transform) in goal_query.iter_mut() {
        transform.translation[0] = goal.0[0];
        transform.translation[1] = goal.0[1];
    }
}
