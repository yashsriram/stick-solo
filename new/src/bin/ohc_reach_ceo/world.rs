use super::ceo::Reward;
use super::fcn::*;
use super::utils::{control, decode, encode, random_sample_solve, GoalQsCouple};
use bevy::prelude::*;
use ndarray::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use stick_solo::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use stick_solo::act::switchable_nr::PivotingSide;
use stick_solo::act::switchable_nr::SwitchableNR;
use stick_solo::game::goal_couple_plugin::GoalCouple;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct World {
    pub origin: Vec2,
    pub holding_ls: Vec<f32>,
    pub holding_q_clamps: Vec<(Option<f32>, Option<f32>)>,
    pub non_holding_ls: Vec<f32>,
    pub non_holding_q_clamps: Vec<(Option<f32>, Option<f32>)>,
    pub unscaled_relative_goal_region: (Vec2, Vec2),
}

impl World {
    fn sample_qs(q_clamps: &[(Option<f32>, Option<f32>)]) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        q_clamps
            .iter()
            .map(|(min, max)| {
                if *min == None || *max == None {
                    0.0
                } else {
                    rng.gen_range(min.unwrap(), max.unwrap())
                }
            })
            .collect()
    }

    pub fn sample_holding_qs(&self) -> Vec<f32> {
        World::sample_qs(&self.holding_q_clamps)
    }

    pub fn sample_non_holding_qs(&self) -> Vec<f32> {
        World::sample_qs(&self.non_holding_q_clamps)
    }

    fn get_q_clamps(q_clamps: &[(Option<f32>, Option<f32>)]) -> Vec<(f32, f32)> {
        let inf = f32::INFINITY;
        q_clamps
            .iter()
            .map(|(min, max)| match (min, max) {
                (None, None) => (-inf, inf),
                (None, Some(l)) => (-inf, *l),
                (Some(l), None) => (*l, inf),
                (Some(l1), Some(l2)) => (*l1, *l2),
            })
            .collect()
    }

    pub fn holding_q_clamps(&self) -> Vec<(f32, f32)> {
        World::get_q_clamps(&self.holding_q_clamps)
    }

    pub fn non_holding_q_clamps(&self) -> Vec<(f32, f32)> {
        World::get_q_clamps(&self.non_holding_q_clamps)
    }

    pub fn sample_goal(&self) -> Vec2 {
        let (min, max) = self.unscaled_relative_goal_region;
        let scale = self.holding_ls.iter().sum::<f32>() + self.non_holding_ls.iter().sum::<f32>();
        let (min, max) = (min * scale, max * scale);
        let diff = max - min;
        let rand_diff = Vec2::new(
            rand::random::<f32>() * diff[0],
            rand::random::<f32>() * diff[1],
        );
        self.origin + min + rand_diff
    }
}

impl Plugin for World {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.clone())
            .add_startup_system(init_vis.system());
    }
}

fn init_vis(
    mut commands: Commands,
    world: Res<World>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (min, max) = world.unscaled_relative_goal_region;
    let scale = world.holding_ls.iter().sum::<f32>() + world.non_holding_ls.iter().sum::<f32>();
    let (min, max) = (min * scale, max * scale);
    let midpoint = world.origin + (min + max) / 2.0;
    let diff = max - min;
    commands.spawn(SpriteComponents {
        sprite: Sprite {
            size: Vec2::new(diff[0], diff[1]),
            resize_mode: SpriteResizeMode::Manual,
        },
        transform: Transform::from_translation(Vec3::new(midpoint[0], midpoint[1], 0.0)),
        material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.02).into()),
        ..Default::default()
    });
}

impl Reward for World {
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
                PivotingSide::Left,
                self.origin,
                &self.holding_ls,
                &self.sample_holding_qs(),
                &self.holding_q_clamps(),
                &self.non_holding_ls,
                &self.sample_non_holding_qs(),
                &self.non_holding_q_clamps(),
                0.01,
            );
            let holding_origin = agent.holding().get_current_state().1.clone();
            let non_holding_goal = self.sample_goal();
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
                episode_reward -= dist * 2.0;
                // Non holding
                let last_vertex = agent.non_holding().get_last_vertex();
                let dist = (last_vertex - non_holding_goal).length();
                episode_reward -= dist * 10.0;
                // COM x
                let com = agent.get_center_of_mass();
                episode_reward -= (com[0] - (non_holding_goal[0] + holding_origin[0]) / 2.0).abs();
                // COM y
                let com = agent.get_center_of_mass();
                episode_reward -= com[1];
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
