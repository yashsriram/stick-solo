extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use std::collections::LinkedList;
use stick_solo::act::switchable_nr::*;
use stick_solo::game::{
    base_plugins::BasePlugins,
    camera_plugin::CameraPlugin,
    path_plugin::{Path, PathPlugin},
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
    switchable_nr_plugin::SwitchableNRPlugin,
};
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
        .add_plugins(BasePlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(SwitchableNRPlugin::new(SwitchableNR::new(
            Vec2::new(-0.5, -0.1),
            &[0.2, 0.3, 0.3, 0.2],
            &[-2.0, 0.0, 2.0, 0.0],
            &[
                (-inf, inf),
                (0.0, pi * 0.5),
                (-pi * 0.5, pi),
                (0.0, pi * 0.5),
            ],
            PivotingSide::Left,
            0.01,
        )))
        .add_plugin(PathPlugin::new(Path({
            let mut path = LinkedList::new();
            path.push_back(Vec2::new(-0.6, 0.1));
            path.push_back(Vec2::new(-0.5, 0.1));
            path.push_back(Vec2::new(-0.1, 0.1));
            path.push_back(Vec2::new(0.3, 0.1));
            path.push_back(Vec2::new(-0.1, -0.3));
            path.push_back(Vec2::new(-0.1, -0.5));
            path.push_back(Vec2::new(-0.1, -0.7));
            path
        })))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(control.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
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
    let (_, origin, ls, qs, pivoting_side) = agent.get_current_state();
    let given_goal = path.0.front().unwrap().clone();
    let have_to_match = match pivoting_side {
        PivotingSide::Left => given_goal[0] - origin[0] < -SwitchableNR::GOAL_REACHED_SLACK,
        PivotingSide::Right => given_goal[0] - origin[0] > SwitchableNR::GOAL_REACHED_SLACK,
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

    let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) = ik(
        origin,
        ls,
        qs,
        &given_goal,
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
        2.0 * take_end_to_given_goal + -0.1 * push_com_x_from_its_goal + 1.0 * push_com_y_downward,
    );

    ticks.0 += 1;
}
