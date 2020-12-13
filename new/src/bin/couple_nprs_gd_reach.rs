extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use stick_solo::act::switchable_nr_couple::SwitchableNRCouple;
use stick_solo::game::{
    base_plugins::BasePlugins,
    camera_plugin::CameraPlugin,
    goal_couple_plugin::{GoalCouple, GoalCouplePlugin},
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
    switchable_nr_couple_plugin::SwitchableNRCouplePlugin,
};
use stick_solo::plan::gradient_descent::*;
use stick_solo::plan::random_sampling::*;
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
        .add_resource(GoalQs(Array::zeros(2), Array::zeros(2)))
        .add_plugin(GoalCouplePlugin::new(GoalCouple(
            Vec2::new(0.2, -0.2),
            Vec2::new(0.5, -0.0),
        )))
        .add_plugins(BasePlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(SwitchableNRCouplePlugin::new(
            SwitchableNRCouple::new_left_pivot(
                Vec2::new(0.0, -0.1),
                &[0.3, 0.2],
                &[0.1, 0.1],
                &[(-inf, inf), (0.0, pi)],
                &[0.2, 0.3],
                &[0.1, 0.2],
                &[(-inf, inf), (0.0, pi)],
                0.01,
            ),
        ))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(genetic_solve_no_prior.system())
        .add_system(interpolate.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

struct GoalQs(Array1<f32>, Array1<f32>);

fn genetic_solve_no_prior(
    agent: Res<SwitchableNRCouple>,
    mut goal_qs: ResMut<GoalQs>,
    mut ticks: ResMut<Ticks>,
    goal_couple: ResMut<GoalCouple>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::W)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::S)
        || keyboard_input.pressed(KeyCode::D)
        || keyboard_input.pressed(KeyCode::T)
        || keyboard_input.pressed(KeyCode::G)
        || keyboard_input.pressed(KeyCode::F)
        || keyboard_input.pressed(KeyCode::H)
    {
        let (_, origin_left, ls, qs, q_clamps, pivoting_side) = agent.left().get_current_state();
        let (_min_loss, best_q) = no_prior_random_sample_optimizer(
            10_000,
            origin_left,
            ls,
            qs[0],
            pivoting_side,
            q_clamps,
            &goal_couple.0,
            |end, com, goal| {
                5.0 * (end.clone() - goal.clone()).length()
                    + com[1]
                    + (com[0] - (end[0] + goal[0]) / 2.0).abs()
            },
        );
        goal_qs.0 = best_q;
        let (origin_right, _) = get_end_verticex_and_com(origin_left, ls, &goal_qs.0);
        let (_, _, ls, qs, q_clamps, pivoting_side) = agent.right().get_current_state();
        let (_min_loss, best_q) = no_prior_random_sample_optimizer(
            10_000,
            &origin_right,
            ls,
            qs[0],
            pivoting_side,
            q_clamps,
            &goal_couple.1,
            |end, com, goal| {
                5.0 * (end.clone() - goal.clone()).length()
                    + com[1]
                    + (com[0] - (end[0] + goal[0]) / 2.0).abs()
            },
        );
        goal_qs.1 = best_q;
        ticks.0 = 0;
    }
}

fn interpolate(
    mut agent: ResMut<SwitchableNRCouple>,
    pause: Res<Pause>,
    mut ticks: ResMut<Ticks>,
    goal_qs: Res<GoalQs>,
    goal_couple: ResMut<GoalCouple>,
) {
    // Pause => pause everything
    if pause.0 {
        return;
    }
    {
        let (_, origin, ls, qs, _, _) = agent.left().get_current_state();

        let global_delta_qs = &goal_qs.0 - qs;

        let given_goal = goal_couple.0;
        let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) =
            gradient_descent(
                origin,
                ls,
                qs,
                &given_goal,
                EndControl::JacobianTranspose,
                COMXGoalType::PivotGoalMidpoint,
            );

        let alpha = 1.0 / (1.0 + ticks.0 as f32).powf(0.5);
        let beta = 1.0 - alpha;
        let gamma = 0.1;
        let delta = 0.1 / (1.0 + ticks.0 as f32).powf(1.0);
        agent.update(
            alpha * global_delta_qs
                + beta * take_end_to_given_goal
                + gamma * -push_com_x_from_its_goal
                + delta * -push_com_y_upward,
            arr1(&[0.0, 0.0]),
        );
    }
    {
        let (_, origin, ls, qs, _, _) = agent.right().get_current_state();

        let global_delta_qs = &goal_qs.1 - qs;

        let given_goal = goal_couple.1;
        let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) =
            gradient_descent(
                origin,
                ls,
                qs,
                &given_goal,
                EndControl::JacobianTranspose,
                COMXGoalType::PivotGoalMidpoint,
            );

        let alpha = 1.0 / (1.0 + ticks.0 as f32).powf(0.5);
        let beta = 1.0 - alpha;
        let gamma = 0.1;
        let delta = 0.1 / (1.0 + ticks.0 as f32).powf(1.0);
        agent.update(
            arr1(&[0.0, 0.0]),
            alpha * global_delta_qs
                + beta * take_end_to_given_goal
                + gamma * -push_com_x_from_its_goal
                + delta * -push_com_y_upward,
        );
    }
    ticks.0 += 1;
}
