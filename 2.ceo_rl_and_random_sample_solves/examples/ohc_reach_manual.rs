extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use stick_solo::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use stick_solo::act::switchable_nr::Side;
use stick_solo::game::{
    camera_plugin::CameraPlugin,
    goal_couple_plugin::{GoalCouple, GoalCouplePlugin},
    one_holding_switchable_nr_couple_plugin::OneHoldingSwitchableNRCouplePlugin,
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
};
use stick_solo::plan::gradient_descent::*;
use stick_solo::plan::random_sampling::*;
use stick_solo::plan::*;

fn main() {
    let inf = f32::INFINITY;
    let pi = std::f32::consts::PI;
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .insert_resource(GoalQs(Array::zeros(2), Array::zeros(3)))
        .add_plugin(OneHoldingSwitchableNRCouplePlugin::new(
            OneHoldingSwitchableNRCouple::new(
                &Side::Right,
                Vec2::new(0.0, -0.1),
                &[0.2, 0.3],
                &[-0.1, -0.2],
                &[(-inf, inf), (-pi, 0.0)],
                &[0.2, 0.2, 0.1],
                &[-0.1, -0.1, -0.1],
                &[(-inf, inf), (-pi, 0.0), (-pi / 6.0, 0.0)],
                0.05,
            ),
        ))
        .add_plugin(GoalCouplePlugin::new(GoalCouple(
            Vec2::new(-0.2, -0.2),
            Vec2::new(-0.5, -0.0),
        )))
        // .add_resource(GoalQs(Array::zeros(3), Array::zeros(2)))
        // .add_plugin(OneHoldingSwitchableNRCouplePlugin::new(
        //     OneHoldingSwitchableNRCouple::new(
        //         &Side::Left,
        //         Vec2::new(0.0, -0.1),
        //         &[0.2, 0.2, 0.1],
        //         &[0.1, 0.1, 0.1],
        //         &[(-inf, inf), (0.0, pi), (0.0, pi / 6.0)],
        //         &[0.2, 0.3],
        //         &[0.1, 0.2],
        //         &[(-inf, inf), (0.0, pi)],
        //         0.01,
        //     ),
        // ))
        // .add_plugin(GoalCouplePlugin::new(GoalCouple(
        //     Vec2::new(0.2, -0.2),
        //     Vec2::new(0.5, -0.0),
        // )))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(random_sample_solve)
        .add_system(control)
        .run();
}

struct GoalQs(Array1<f32>, Array1<f32>);

fn random_sample_solve(
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut goal_qs: ResMut<GoalQs>,
    mut ticks: ResMut<Ticks>,
    goal_couple: ResMut<GoalCouple>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::W)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::S)
        || keyboard_input.pressed(KeyCode::D)
        || keyboard_input.pressed(KeyCode::I)
        || keyboard_input.pressed(KeyCode::K)
        || keyboard_input.pressed(KeyCode::J)
        || keyboard_input.pressed(KeyCode::L)
    {
        let (_, origin_holding, ls, qs, q_clamps, pivoting_side) =
            agent.holding().get_current_state();
        let loss_fn = |end: &Vec2, com: &Vec2, goal: &Vec2, origin: &Vec2| {
            5.0 * (end.clone() - goal.clone()).length()
                + com[1]
                + (com[0] - (origin[0] + goal[0]) / 2.0).abs()
        };
        let (_min_loss, best_q) = no_prior_random_sample_optimizer(
            10_000,
            origin_holding,
            ls,
            qs[0],
            pivoting_side,
            q_clamps,
            &goal_couple.0,
            loss_fn,
        );
        goal_qs.0 = best_q;
        let (origin_non_holding, _) = get_end_verticex_and_com(origin_holding, ls, &goal_qs.0);
        let (_, _, ls, qs, q_clamps, pivoting_side) = agent.non_holding().get_current_state();
        let (_min_loss, best_q) = no_prior_random_sample_optimizer(
            10_000,
            &origin_non_holding,
            ls,
            qs[0],
            pivoting_side,
            q_clamps,
            &goal_couple.1,
            loss_fn,
        );
        goal_qs.1 = best_q;
        ticks.0 = 0;
    }
}

fn control(
    mut agent: ResMut<OneHoldingSwitchableNRCouple>,
    pause: Res<Pause>,
    mut ticks: ResMut<Ticks>,
    goal_qs: Res<GoalQs>,
    goal_couple: ResMut<GoalCouple>,
) {
    // Pause => pause everything
    if pause.0 {
        return;
    }
    let holding_delta_qs = {
        let (_, origin, ls, qs, _, _) = agent.holding().get_current_state();
        let global_delta_qs = &goal_qs.0 - qs;
        let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) =
            gradient_descent(
                origin,
                ls,
                qs,
                &goal_couple.0,
                EndControl::JacobianTranspose,
                COMXGoalType::PivotGoalMidpoint,
            );
        let alpha = 1.0 / (1.0 + ticks.0 as f32).powf(0.5);
        let beta = 1.0 - alpha;
        let gamma = 0.1;
        let delta = 0.1 / (1.0 + ticks.0 as f32).powf(1.0);
        alpha * global_delta_qs
            + beta * take_end_to_given_goal
            + gamma * -push_com_x_from_its_goal
            + delta * -push_com_y_upward
    };
    let non_holding_delta_qs = {
        let (_, origin, ls, qs, _, _) = agent.non_holding().get_current_state();
        let global_delta_qs = &goal_qs.1 - qs;
        let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) =
            gradient_descent(
                origin,
                ls,
                qs,
                &goal_couple.1,
                EndControl::JacobianTranspose,
                COMXGoalType::PivotGoalMidpoint,
            );
        let alpha = 1.0 / (1.0 + ticks.0 as f32).powf(0.5);
        let beta = 1.0 - alpha;
        let gamma = 0.1;
        let delta = 0.1 / (1.0 + ticks.0 as f32).powf(1.0);
        alpha * global_delta_qs
            + beta * take_end_to_given_goal
            + gamma * -push_com_x_from_its_goal
            + delta * -push_com_y_upward
    };
    agent.update(holding_delta_qs, non_holding_delta_qs);
    ticks.0 += 1;
}
