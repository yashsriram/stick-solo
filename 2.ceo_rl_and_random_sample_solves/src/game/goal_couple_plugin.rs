use crate::act::switchable_nr::SwitchableNR;
use bevy::prelude::*;

#[derive(Clone)]
pub struct GoalCouple(pub Vec2, pub Vec2);

pub struct GoalCouplePlugin {
    goal_couple: GoalCouple,
}

impl GoalCouplePlugin {
    pub fn new(goal_couple: GoalCouple) -> GoalCouplePlugin {
        GoalCouplePlugin { goal_couple }
    }
}

impl Plugin for GoalCouplePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.goal_couple.clone())
            .add_startup_system(init_vis)
            .add_system(interactive_goal_couple)
            .add_system(flush_transforms);
    }
}

#[derive(Component)]
struct Goal0Marker;
#[derive(Component)]
struct Goal1Marker;

fn init_vis(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    4.0 * SwitchableNR::GOAL_REACHED_SLACK,
                    4.0 * SwitchableNR::GOAL_REACHED_SLACK,
                )),
                color: Color::rgb(0.0, 1.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Goal0Marker);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    4.0 * SwitchableNR::GOAL_REACHED_SLACK,
                    4.0 * SwitchableNR::GOAL_REACHED_SLACK,
                )),
                color: Color::rgb(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Goal1Marker);
}

fn interactive_goal_couple(
    keyboard_input: Res<Input<KeyCode>>,
    mut goal_couple: ResMut<GoalCouple>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        goal_couple.0[1] += 0.01;
    } else if keyboard_input.pressed(KeyCode::S) {
        goal_couple.0[1] -= 0.01;
    } else if keyboard_input.pressed(KeyCode::A) {
        goal_couple.0[0] -= 0.01;
    } else if keyboard_input.pressed(KeyCode::D) {
        goal_couple.0[0] += 0.01;
    }

    if keyboard_input.pressed(KeyCode::I) {
        goal_couple.1[1] += 0.01;
    } else if keyboard_input.pressed(KeyCode::K) {
        goal_couple.1[1] -= 0.01;
    } else if keyboard_input.pressed(KeyCode::J) {
        goal_couple.1[0] -= 0.01;
    } else if keyboard_input.pressed(KeyCode::L) {
        goal_couple.1[0] += 0.01;
    }
}

fn flush_transforms(
    goal_couple: Res<GoalCouple>,
    mut goal_0_query: Query<(&Goal0Marker, &mut Transform)>,
    mut goal_1_query: Query<(&Goal1Marker, &mut Transform)>,
) {
    for (_, mut transform) in goal_0_query.iter_mut() {
        transform.translation[0] = goal_couple.0[0];
        transform.translation[1] = goal_couple.0[1];
    }
    for (_, mut transform) in goal_1_query.iter_mut() {
        transform.translation[0] = goal_couple.1[0];
        transform.translation[1] = goal_couple.1[1];
    }
}
