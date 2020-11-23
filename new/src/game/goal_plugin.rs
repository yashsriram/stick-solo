use crate::act::Goal;
use bevy::prelude::*;

pub struct GoalPlugin {
    goal: Goal,
}

impl GoalPlugin {
    pub fn new(goal: Goal) -> GoalPlugin {
        GoalPlugin { goal }
    }
}

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.goal.clone())
            .add_startup_system(init.system())
            .add_system(interactive_goal.system())
            .add_system(flush_transforms.system());
    }
}

struct GoalMarker;

fn init(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(SpriteComponents {
            sprite: Sprite {
                size: Vec2::new(0.04, 0.04),
                resize_mode: SpriteResizeMode::Manual,
            },
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..Default::default()
        })
        .with(GoalMarker);
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
