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
use stick_solo::plan::gradient_descent::*;
use stick_solo::plan::random_sampling::*;

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
        .add_resource(GoalQs(Array::zeros(4)))
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
            let parts = 8usize;
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
        .add_startup_system(set_first_goal.system())
        .add_system(control.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

struct GoalQs(Array1<f32>);

fn set_first_goal(agent: ResMut<SwitchableNR>, path: ResMut<Path>, mut goal_qs: ResMut<GoalQs>) {
    let (n, origin, ls, qs, q_clamps, pivoting_side) = agent.get_current_state();
    let loss_fn = |end: &Vec2, com: &Vec2, goal: &Vec2, origin: &Vec2| {
        5.0 * (end.clone() - goal.clone()).length()
            + 5.0 * com[1]
            + (com[0] - (origin[0] + goal[0]) / 2.0).abs()
    };
    let (_min_loss, best_q) = from_current_state_random_sample_optimizer(
        10_000,
        3.0,
        n,
        origin,
        ls,
        qs,
        pivoting_side,
        q_clamps,
        &path.0.front().unwrap().clone(),
        loss_fn,
    );
    goal_qs.0 = best_q;
}

fn control(
    mut agent: ResMut<SwitchableNR>,
    mut path: ResMut<Path>,
    mut goal_qs: ResMut<GoalQs>,
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
    let loss_fn = |end: &Vec2, com: &Vec2, goal: &Vec2, origin: &Vec2| {
        5.0 * (end.clone() - goal.clone()).length()
            + 5.0 * com[1]
            + (com[0] - (origin[0] + goal[0]) / 2.0).abs()
    };
    let (_, origin, ls, qs, _, pivoting_side) = agent.get_current_state();
    let given_goal = path.0.front().unwrap().clone();
    let have_to_match = match pivoting_side {
        Side::Left => given_goal[0] - origin[0] < -SwitchableNR::GOAL_REACHED_SLACK,
        Side::Right => given_goal[0] - origin[0] > SwitchableNR::GOAL_REACHED_SLACK,
    };
    if have_to_match {
        path.0.push_front(origin.clone());

        let (n, origin, ls, qs, q_clamps, pivoting_side) = agent.get_current_state();
        let (_min_loss, best_q) = from_current_state_random_sample_optimizer(
            10_000,
            3.0,
            n,
            origin,
            ls,
            qs,
            pivoting_side,
            q_clamps,
            &path.0.front().unwrap().clone(),
            loss_fn,
        );
        goal_qs.0 = best_q;

        return;
    }
    let last = agent.get_last_vertex();
    if (given_goal - last).length() < SwitchableNR::GOAL_REACHED_SLACK {
        agent.switch_pivot();
        path.0.pop_front();

        ticks.0 = 0;

        if path.0.len() > 0 {
            let (n, origin, ls, qs, q_clamps, pivoting_side) = agent.get_current_state();
            let (_min_loss, best_q) = from_current_state_random_sample_optimizer(
                10_000,
                3.0,
                n,
                origin,
                ls,
                qs,
                pivoting_side,
                q_clamps,
                &path.0.front().unwrap().clone(),
                loss_fn,
            );
            goal_qs.0 = best_q;
        }
        return;
    }

    let global_delta_qs = &goal_qs.0 - qs;

    let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) = gradient_descent(
        origin,
        ls,
        qs,
        &given_goal,
        EndControl::JacobianTranspose,
        COMXGoalType::PivotGoalMidpoint,
    );

    // {
    //     fn downward_push_coeff(com: &Vec2, origin: &Vec2) -> f32 {
    //         let diff_y = com[1] - origin[1];
    //         if diff_y < 0.0 {
    //             0.0
    //         } else {
    //             1.0 * diff_y.abs()
    //         }
    //     }
    //     let rnd: f32 = thread_rng().sample(Normal::new(0.0, 3.0).unwrap());
    //     2.0 * rnd * rnd.signum() * take_end_to_given_goal
    //         + -0.2 * push_com_x_from_its_goal
    //         + -downward_push_coeff(&agent.get_center_of_mass(), origin) * push_com_y_upward
    // };
    let alpha = 1.0 / (1.0 + ticks.0 as f32).powf(0.8);
    let beta = 0.01 / take_end_to_given_goal.mapv(|e| e * e).sum().sqrt();
    let gamma = 0.1;
    let delta = 0.1 / (1.0 + ticks.0 as f32).powf(1.0);
    agent.update(
        alpha * global_delta_qs
            + beta * take_end_to_given_goal
            + gamma * -push_com_x_from_its_goal
            + delta * -push_com_y_upward,
    );

    ticks.0 += 1;
}
