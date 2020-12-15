extern crate stick_solo;
use bevy::prelude::*;
use rand::prelude::*;
use rand_distr::Normal;
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
            &[-2.0, 0.0, 2.0, 0.0],
            &[
                (-inf, inf),
                (0.0, pi * 0.5),
                (-pi * 0.5, pi),
                (0.0, pi * 0.5),
            ],
            Side::Left,
            0.01,
        )))
        .add_plugin(PathPlugin::new(Path({
            let mut path = LinkedList::new();
            // path.push_back(Vec2::new(-0.6, 0.1));
            // path.push_back(Vec2::new(-0.5, 0.1));
            // path.push_back(Vec2::new(-0.1, 0.1));
            // path.push_back(Vec2::new(0.3, 0.1));
            // path.push_back(Vec2::new(-0.1, -0.3));
            // path.push_back(Vec2::new(-0.2, -0.5));
            // path.push_back(Vec2::new(-0.3, -0.7));
            // path.push_back(Vec2::new(-0.3, -0.9));
            // path.push_back(Vec2::new(-0.3, -1.1));
            // path.push_back(Vec2::new(-0.3, -1.1));
            // path.push_back(Vec2::new(-0.2, -1.3));
            // path.push_back(Vec2::new(0.1, -1.5));
            // path.push_back(Vec2::new(0.5, -1.5));
            // path.push_back(Vec2::new(0.7, -1.3));
            // path.push_back(Vec2::new(0.7, -1.15));
            // path.push_back(Vec2::new(0.7, -1.0));
            let parts = 7usize;
            for i in 0..parts {
                let theta = 2.0 * pi * (i as f32) / (parts as f32);
                path.push_back(Vec2::new(-1.0 + theta.cos(), theta.sin()) * 0.5);
            }
            for i in 0..parts {
                let theta = 2.0 * pi * ((parts - i) as f32) / (parts as f32) + pi;
                path.push_back(Vec2::new(1.0 + theta.cos(), theta.sin()) * 0.5);
            }
            path
        })))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(control.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

struct RestTicks(usize);

fn control(
    mut agent: ResMut<SwitchableNR>,
    mut path: ResMut<Path>,
    pause: Res<Pause>,
    mut ticks: ResMut<Ticks>,
    mut rest_ticks: ResMut<RestTicks>,
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
        rest_ticks.0 = 0;
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
            1.0 * diff_y.abs()
        }
    }
    let rnd: f32 = thread_rng().sample(Normal::new(0.0, 3.0).unwrap());
    let beta = 0.03 / take_end_to_given_goal.mapv(|e| e * e).sum().sqrt();
    let com = agent.get_center_of_mass();
    let origin = origin.clone();
    agent.update(if rest_ticks.0 < 40 {
        -1.0 * push_com_x_from_its_goal + -5.0 * push_com_y_upward
    } else {
        beta * rnd * rnd.signum() * take_end_to_given_goal
            + -0.2 * push_com_x_from_its_goal
            + -downward_push_coeff(&com, origin) * push_com_y_upward
    });

    ticks.0 += 1;
    rest_ticks.0 += 1;
}
