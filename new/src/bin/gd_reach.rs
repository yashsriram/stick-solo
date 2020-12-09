extern crate stick_solo;
use bevy::prelude::*;
use stick_solo::act::switchable_nr::{PivotingSide, SwitchableNR};
use stick_solo::game::{
    base_plugins::BasePlugins,
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
        .add_plugin(SwitchableNRPlugin::new(SwitchableNR::new(
            Vec2::new(0.0, -0.1),
            &[0.2, 0.2, 0.2, 0.2],
            &[0.0, 0.0, 2.0, 0.0],
            &[
                (-inf, inf),
                (0.0, pi * 0.5),
                (-pi * 0.5, pi),
                (0.0, pi * 0.5),
            ],
            PivotingSide::Left,
            0.01,
        )))
        .add_plugin(GoalPlugin::new(Goal(Vec2::new(0.5, 0.5))))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(control.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
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
