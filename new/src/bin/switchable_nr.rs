extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use stick_solo::act::{switchable_nr::SwitchableNR, Goal};
use stick_solo::game::{pause_plugin::Pause, status_bar_plugin::Ticks, *};
use stick_solo::plan::*;

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
        .add_plugins(base_plugins::BasePlugins)
        .add_plugin(camera_plugin::CameraPlugin)
        .add_plugin(switchable_nr_plugin::SwitchableNRPlugin::new(
            SwitchableNR::new(
                Vec2::new(-0.5, -0.1),
                &[0.2, 0.3, 0.3, 0.2],
                &[-2.0, 0.0, 2.0, 0.0],
                &[
                    (-inf, inf),
                    (0.0, pi * 0.5),
                    (-pi * 0.5, pi),
                    (0.0, pi * 0.5),
                ],
                0.01,
            ),
        ))
        .add_plugin(goal_plugin::GoalPlugin::new(Goal(Vec2::new(0.1, -0.1))))
        .add_plugin(status_bar_plugin::StatusBarPlugin)
        .add_plugin(pause_plugin::PausePlugin)
        .add_system(control.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn control(
    mut agent: ResMut<SwitchableNR>,
    mut goal: ResMut<Goal>,
    pause: Res<Pause>,
    mut ticks: ResMut<Ticks>,
) {
    if pause.0 {
        return;
    }

    let (_, origin, ls, qs) = agent.get_current_state();
    let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) = ik(
        origin,
        ls,
        qs,
        &goal.0,
        EndControl::JacobianTranspose,
        COMXGoalType::PivotGoalMidpoint,
    );
    let com = agent.get_center_of_mass();
    let push_com_y_downward = if com[1] > origin[1] {
        -push_com_y_upward
    } else {
        Array1::<f32>::zeros(qs.len())
    };
    agent.update(
        2.0 * take_end_to_given_goal + -0.2 * push_com_x_from_its_goal + 1.0 * push_com_y_downward,
    );

    let last = agent.get_last_vertex();
    if (goal.0 - last).length() < 0.01 {
        agent.switch_pivot();
        goal.0[1] -= 0.1;
        goal.0[0] -= (rand::random::<f32>() - 0.5) * 0.2;
    }

    ticks.0 += 1;
}
