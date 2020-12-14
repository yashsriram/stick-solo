use bevy::prelude::*;
use ndarray::prelude::*;
use stick_solo::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use stick_solo::game::goal_couple_plugin::GoalCouple;
use stick_solo::plan::gradient_descent::*;
use stick_solo::plan::random_sampling::*;
use stick_solo::plan::*;

pub struct GoalQsCouple(pub Array1<f32>, pub Array1<f32>);

pub fn encode(agent: &OneHoldingSwitchableNRCouple, non_holding_goal: &Vec2) -> (Array1<f32>, f32) {
    let (_, holding_origin, holding_ls, _, _, _) = agent.holding().get_current_state();
    let (_, _, non_holding_ls, _, _, _) = agent.non_holding().get_current_state();
    let relative_goal = non_holding_goal.clone() - holding_origin.clone();

    let scale = holding_ls.sum();
    let holding_ls = holding_ls / scale;
    let non_holding_ls = non_holding_ls / scale;
    let relative_goal = relative_goal / scale;

    (
        arr1(&[
            holding_ls[0],
            holding_ls[1],
            holding_ls[2],
            non_holding_ls[0],
            non_holding_ls[1],
            relative_goal[0],
            relative_goal[1],
        ]),
        scale,
    )
}

pub fn random_sample_solve(
    agent: &OneHoldingSwitchableNRCouple,
    goal_couple: &GoalCouple,
    goal_qs_couple: &mut GoalQsCouple,
) {
    let (_, origin_holding, ls, qs, q_clamps, pivoting_side) = agent.holding().get_current_state();
    let (_min_loss, best_q) = no_prior_random_sample_optimizer(
        10_000,
        origin_holding,
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
    goal_qs_couple.0 = best_q;
    let (origin_non_holding, _) = get_end_verticex_and_com(origin_holding, ls, &goal_qs_couple.0);
    let (_, _, ls, qs, q_clamps, pivoting_side) = agent.non_holding().get_current_state();
    let (_min_loss, best_q) = no_prior_random_sample_optimizer(
        10_000,
        &origin_non_holding,
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
    goal_qs_couple.1 = best_q;
}

pub fn control(
    agent: &mut OneHoldingSwitchableNRCouple,
    goal_qs_couple: &GoalQsCouple,
    goal_couple: &GoalCouple,
    ticks: usize,
) {
    let holding_delta_qs = {
        let (_, origin, ls, qs, _, _) = agent.holding().get_current_state();
        let global_delta_qs = &goal_qs_couple.0 - qs;
        let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) =
            gradient_descent(
                origin,
                ls,
                qs,
                &goal_couple.0,
                EndControl::JacobianTranspose,
                COMXGoalType::PivotGoalMidpoint,
            );
        let alpha = 1.0 / (1.0 + ticks as f32).powf(0.5);
        let beta = 1.0 - alpha;
        let gamma = 0.1;
        let delta = 0.1 / (1.0 + ticks as f32).powf(1.0);
        alpha * global_delta_qs
            + beta * take_end_to_given_goal
            + gamma * -push_com_x_from_its_goal
            + delta * -push_com_y_upward
    };
    let non_holding_delta_qs = {
        let (_, origin, ls, qs, _, _) = agent.non_holding().get_current_state();
        let global_delta_qs = &goal_qs_couple.1 - qs;
        let (take_end_to_given_goal, push_com_x_from_its_goal, push_com_y_upward) =
            gradient_descent(
                origin,
                ls,
                qs,
                &goal_couple.1,
                EndControl::JacobianTranspose,
                COMXGoalType::PivotGoalMidpoint,
            );
        let alpha = 1.0 / (1.0 + ticks as f32).powf(0.5);
        let beta = 1.0 - alpha;
        let gamma = 0.1;
        let delta = 0.1 / (1.0 + ticks as f32).powf(1.0);
        alpha * global_delta_qs
            + beta * take_end_to_given_goal
            + gamma * -push_com_x_from_its_goal
            + delta * -push_com_y_upward
    };
    agent.update(holding_delta_qs, non_holding_delta_qs);
}
