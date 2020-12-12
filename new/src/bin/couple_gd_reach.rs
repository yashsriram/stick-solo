extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use stick_solo::act::switchable_nr_couple::SwitchableNRCouple;
use stick_solo::game::{
    base_plugins::BasePlugins,
    camera_plugin::CameraPlugin,
    goal_plugin::{Goal, GoalPlugin},
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
    switchable_nr_couple_plugin::SwitchableNRCouplePlugin,
};
use stick_solo::plan::gradient_descent::*;

fn main() {
    let inf = f32::INFINITY;
    let pi = std::f32::consts::PI;
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor {
            width: 2000,
            height: 1000,
            ..Default::default()
        })
        .add_resource(RestTicks(0))
        .add_plugins(BasePlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(SwitchableNRCouplePlugin::new(
            SwitchableNRCouple::new_left_pivot(
                Vec2::new(0.0, -0.1),
                &[0.3, 0.2],
                &[0.1, 0.1],
                &[(-inf, inf), (0.0, pi * 0.5)],
                &[0.2, 0.3],
                &[0.1, 0.2],
                &[(-inf, inf), (0.0, pi * 0.5)],
                0.01,
            ),
        ))
        .add_plugin(GoalPlugin::new(Goal(Vec2::new(0.5, 0.5))))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(control.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

struct RestTicks(usize);

fn control(
    mut agent: ResMut<SwitchableNRCouple>,
    pause: Res<Pause>,
    goal: Res<Goal>,
    mut ticks: ResMut<Ticks>,
    mut rest_ticks: ResMut<RestTicks>,
) {
    // Pause => pause everything
    if pause.0 {
        return;
    }
    let (_, origin, ls, qs, _, _) = agent.left().get_current_state();
    let (left_take_end_to_given_goal, left_push_com_x_from_its_goal, _) = gradient_descent(
        origin,
        ls,
        qs,
        &goal.0,
        EndControl::JacobianTranspose,
        COMXGoalType::PivotGoalMidpoint,
    );

    let (_, origin, ls, qs, _, _) = agent.right().get_current_state();
    let (right_take_end_to_given_goal, right_push_com_x_from_its_goal, _) = gradient_descent(
        origin,
        ls,
        qs,
        &goal.0,
        EndControl::JacobianTranspose,
        COMXGoalType::PivotGoalMidpoint,
    );

    agent.update(
        left_take_end_to_given_goal + -0.2 * left_push_com_x_from_its_goal,
        right_take_end_to_given_goal + -0.2 * right_push_com_x_from_its_goal,
    );

    ticks.0 += 1;
    rest_ticks.0 += 1;
}
