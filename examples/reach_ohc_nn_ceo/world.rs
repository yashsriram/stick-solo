use ndarray::prelude::*;
use stick_solo::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use stick_solo::act::switchable_nr::SwitchableNR;
use stick_solo::game::goal_couple_plugin::GoalCouple;
use stick_solo::plan::cross_entropy_optimizing::ceo::Reward;
use stick_solo::plan::cross_entropy_optimizing::fcn::*;
use stick_solo::plan::cross_entropy_optimizing::utils::{
    control, decode, encode, random_sample_solve, GoalQsCouple,
};
use stick_solo::plan::cross_entropy_optimizing::world::World;

pub struct Wrapper(pub World);

impl Reward for Wrapper {
    fn average_reward(
        &self,
        fcn: &FCN,
        params: &Array1<f32>,
        num_episodes: usize,
        num_episode_ticks: usize,
    ) -> f32 {
        let mut cumulative_reward = 0.0;
        for _ in 0..num_episodes {
            // Spawn agent
            let mut agent = OneHoldingSwitchableNRCouple::new(
                &self.0.holding_side,
                self.0.origin,
                &self.0.holding_ls,
                &self.0.sample_holding_qs(),
                &self.0.holding_q_clamps(),
                &self.0.non_holding_ls,
                &self.0.sample_non_holding_qs(),
                &self.0.non_holding_q_clamps(),
            );
            let holding_origin = agent.holding().get_current_state().1.clone();
            let non_holding_goal = self.0.sample_goal();
            // Network pipeline
            let (input, scale) = encode(&agent, &non_holding_goal);
            let forward_pass = fcn.at_with(&input, params);
            let holding_goal = decode(&forward_pass, scale, holding_origin);
            // Setting GoalCouple and GoalQsCouple
            let goal_couple = GoalCouple(holding_goal, non_holding_goal);
            let mut goal_qs_couple = GoalQsCouple(Array::zeros(0), Array::zeros(0));
            random_sample_solve(&agent, &goal_couple, &mut goal_qs_couple);
            // Start calculating reward
            let mut episode_reward = 0.0;
            for ticks in 0..num_episode_ticks {
                // Apply control
                control(&mut agent, &goal_qs_couple, &goal_couple, ticks);
                // Holding
                let last_vertex = agent.holding().get_last_vertex();
                let dist = (last_vertex - holding_goal).length();
                episode_reward -= 2.0 * dist;
                // Non holding
                let last_vertex = agent.non_holding().get_last_vertex();
                let dist = (last_vertex - non_holding_goal).length();
                episode_reward -= 10.0 * dist;
                // COM y
                let com = agent.get_center_of_mass();
                episode_reward -= 5.0 * com[1];
                // COM x
                let com = agent.get_center_of_mass();
                episode_reward -= (com[0] - (non_holding_goal[0] + holding_origin[0]) / 2.0).abs();
            }
            // Holding
            let last_vertex = agent.holding().get_last_vertex();
            let dist = (last_vertex - holding_goal).length();
            if dist < SwitchableNR::GOAL_REACHED_SLACK {
                episode_reward += 500.0;
            }
            // Non holding
            let last_vertex = agent.non_holding().get_last_vertex();
            let dist = (last_vertex - non_holding_goal).length();
            if dist < SwitchableNR::GOAL_REACHED_SLACK {
                episode_reward += 1000.0;
            }

            cumulative_reward += episode_reward;
        }

        let average_reward = cumulative_reward / num_episodes as f32;
        average_reward
    }
}
