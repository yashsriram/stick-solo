use crate::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use crate::game::goal_couple_plugin::GoalCouple;
use crate::plan::gradient_descent::*;
use crate::plan::random_sampling::*;
use crate::plan::*;
use bevy::prelude::*;
use ndarray::prelude::*;

pub struct GoalQsCouple(pub Array1<f32>, pub Array1<f32>);

pub fn encode(agent: &OneHoldingSwitchableNRCouple, non_holding_goal: &Vec2) -> (Array1<f32>, f32) {
    let (_, holding_origin, holding_ls, _, _, _) = agent.holding().get_current_state();
    let (_, _, non_holding_ls, _, _, _) = agent.non_holding().get_current_state();
    let relative_goal = non_holding_goal.clone() - holding_origin.clone();

    // Scaling
    let scale = holding_ls.sum() + non_holding_ls.sum();
    let scaled_holding_ls = holding_ls / scale;
    let scaled_non_holding_ls = non_holding_ls / scale;
    let scaled_relative_goal = relative_goal / scale;

    let mut encoding = Array1::zeros(scaled_holding_ls.len() + scaled_non_holding_ls.len() + 2);
    for (i, &l) in scaled_holding_ls.iter().enumerate() {
        encoding[i] = l;
    }
    for (i, &l) in scaled_non_holding_ls.iter().enumerate() {
        encoding[scaled_holding_ls.len() + i] = l;
    }
    encoding[scaled_holding_ls.len() + scaled_non_holding_ls.len()] = scaled_relative_goal[0];
    encoding[scaled_holding_ls.len() + scaled_non_holding_ls.len() + 1] = scaled_relative_goal[1];
    (encoding, scale)
}

pub fn decode(forward_pass: &Array1<f32>, scale: f32, holding_origin: Vec2) -> Vec2 {
    Vec2::new(forward_pass[0], forward_pass[1]) * scale + holding_origin
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
