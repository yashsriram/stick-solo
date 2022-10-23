extern crate stick_solo;
use bevy::prelude::*;
use stick_solo::act::switchable_nr::{Side, SwitchableNR};
use stick_solo::game::{
    camera_plugin::CameraPlugin,
    goal_plugin::{Goal, GoalPlugin},
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
    switchable_nr_plugin::SwitchableNRPlugin,
};
use stick_solo::plan::gradient_descent::*;

fn main() {
    let inf = f32::INFINITY;
    let pi = std::f32::consts::PI;
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(RestTicks(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(SwitchableNRPlugin::new(SwitchableNR::new(
            Vec2::new(0.0, 0.0),
            &[0.2; 6],
            &[0.0; 6],
            &[(-inf, inf); 6],
            Side::Left,
            0.02,
        )))
        .add_plugin(GoalPlugin::new(Goal(Vec2::new(0.5, 0.2))))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(control)
        .run();
}

struct RestTicks(usize);

fn control(
    mut agent: ResMut<SwitchableNR>,
    pause: Res<Pause>,
    goal: Res<Goal>,
    mut ticks: ResMut<Ticks>,
    mut rest_ticks: ResMut<RestTicks>,
) {
    // Pause => pause everything
    if pause.0 {
        return;
    }
    let (_, origin, ls, qs, _, _) = agent.get_current_state();
    let given_goal = goal.0;

    let (take_end_to_given_goal, push_com_x_from_its_goal, _) = gradient_descent(
        origin,
        ls,
        qs,
        &given_goal,
        EndControl::JacobianTranspose,
        COMXGoalType::PivotGoalMidpoint,
    );

    agent.update(take_end_to_given_goal + -0.2 * push_com_x_from_its_goal);

    ticks.0 += 1;
    rest_ticks.0 += 1;
}
