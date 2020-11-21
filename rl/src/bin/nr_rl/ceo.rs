use super::fcn::*;
use bevy::prelude::*;
use ndarray::prelude::*;
use ndarray::stack;
use ndarray_rand::rand_distr::{NormalError, StandardNormal};
use ndarray_rand::RandomExt;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use stick_solo::act::NRAgent;

pub fn generate_input(
    state: (usize, &Vec2, &Array1<f32>, &Array1<f32>),
    goal: &Vec2,
) -> Array1<f32> {
    let (_, origin, ls, qs) = state;
    let scale = (goal.clone() - origin.clone()).length();
    let scaled_goal = (goal.clone() - origin.clone()) / scale;
    let scaled_ls = ls / scale;

    fn get_all_vertices(ls: &Array1<f32>, qs: &Array1<f32>) -> Vec<f32> {
        let mut vertices = vec![];
        let mut e1 = Vec2::zero();
        let mut cumulative_rotation = 0f32;
        for i in 0..qs.len() {
            cumulative_rotation += qs[i];
            let e2 = e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * ls[i];
            vertices.push(e2[0]);
            vertices.push(e2[1]);
            e1 = e2;
        }
        vertices
    }

    let mut input = Vec::with_capacity(ls.len() * 2 + 2);
    input.append(&mut get_all_vertices(&scaled_ls, qs));
    input.push(scaled_goal[0]);
    input.push(scaled_goal[1]);
    arr1(&input)
}

fn reward(
    ls: &[f32],
    fcn: &FCN,
    params: &Array1<f32>,
    num_episodes: usize,
    num_episode_ticks: usize,
) -> f32 {
    let mut cumulative_reward = 0.0;
    for _ in 0..num_episodes {
        // Set goal
        let goal = Vec2::new(0.2, 0.0);
        // Spawn agent
        let mut agent = NRAgent::new(Vec2::new(0.0, 0.0), ls, &[0.5, 0.5, 0.5, 0.5], 1.0);
        // Start calculating reward
        let mut episode_reward = 0.0;
        for _tick in 0..num_episode_ticks {
            let input = generate_input(agent.get_current_state(), &goal);
            let delta_qs = fcn.at_with(&input, params);
            let delta_qs_norm = delta_qs.mapv(|e| e * e).sum().sqrt();
            // let prev_delta_qs = agent.get_current_control();
            // let delta_delta_qs_norm = (delta_qs.clone() - prev_delta_qs)
            //     .mapv(|e| e * e)
            //     .sum()
            //     .sqrt();
            // Apply control
            agent.update(delta_qs);
            // Makes agent translate towards goal
            let last_vertex = agent.get_last_vertex();
            let dist = (last_vertex - goal).length();
            episode_reward -= dist * 100.0;
            // Penalize huge controls
            episode_reward -= delta_qs_norm;
            // Penalize huge difference in controls
            // episode_reward -= delta_delta_qs_norm;
        }
        // Makes agent reach the goal at the end of episode
        let last_vertex = agent.get_last_vertex();
        let final_dist = (last_vertex - goal).length();
        episode_reward += 1000.0 * (-final_dist).exp();
        // Makes agent stop at the end of episode
        let delta_qs = agent.get_current_control();
        let delta_qs_norm = delta_qs.mapv(|e| e * e).sum().sqrt();
        episode_reward += 10000.0 * (-delta_qs_norm).exp() * (-final_dist).exp();

        cumulative_reward += episode_reward;
    }

    let average_reward = cumulative_reward / num_episodes as f32;
    average_reward
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CEO {
    pub generations: usize,
    pub batch_size: usize,
    pub num_episodes: usize,
    pub num_episode_ticks: usize,
    pub elite_frac: f32,
    pub initial_std: f32,
    pub noise_factor: f32,
}

impl Default for CEO {
    fn default() -> CEO {
        CEO {
            generations: 300,
            batch_size: 50,
            num_episodes: 300,
            num_episode_ticks: 500,
            elite_frac: 0.25,
            initial_std: 2.0,
            noise_factor: 2.0,
        }
    }
}

impl CEO {
    pub fn optimize(&self, ls: &[f32], fcn: &mut FCN) -> Result<(f32, Array1<f32>), NormalError> {
        let n_elite = (self.batch_size as f32 * self.elite_frac).round().floor() as usize;
        let mut noise_std = Array::from_elem((fcn.params().len(),), self.initial_std);
        let mut latest_mean_reward = 0.0;
        for generation in 0..self.generations {
            let (sorted_th_means, mean_reward) = {
                let mut reward_th_mean_tuples = (0..self.batch_size)
                    .into_par_iter()
                    // .into_iter()
                    .map(|_| {
                        let randn_noise: Array1<f32> =
                            Array::random(fcn.params().len(), StandardNormal);
                        let scaled_randn_noise = randn_noise * &noise_std;
                        let perturbed_params = scaled_randn_noise + fcn.params();
                        (
                            reward(
                                ls,
                                fcn,
                                &perturbed_params,
                                self.num_episodes,
                                self.num_episode_ticks,
                            ),
                            perturbed_params,
                        )
                    })
                    .collect::<Vec<(f32, Array1<f32>)>>();
                reward_th_mean_tuples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                reward_th_mean_tuples.reverse();
                let (rewards, sorted_th_means): (Vec<_>, Vec<_>) =
                    reward_th_mean_tuples.into_iter().unzip();
                (
                    sorted_th_means,
                    rewards.iter().sum::<f32>() / rewards.len() as f32,
                )
            };
            let elite_ths = sorted_th_means
                .iter()
                .take(n_elite)
                .map(|th| th.slice(s![..]))
                .collect::<Vec<ArrayView1<f32>>>();
            let elite_ths = stack(Axis(0), &elite_ths)
                .unwrap()
                .into_shape((n_elite, fcn.params().len()))
                .unwrap();
            fcn.set_params(elite_ths.mean_axis(Axis(0)).unwrap());
            noise_std = elite_ths.std_axis(Axis(0), 0.0);
            noise_std += self.noise_factor / (generation + 1) as f32;
            println!(
                "generation={} mean_reward={:?} reward_with_current_th={:?}, th_std_mean={:?}",
                generation + 1,
                mean_reward,
                reward(
                    ls,
                    fcn,
                    &fcn.params(),
                    self.num_episodes,
                    self.num_episode_ticks
                ),
                noise_std.mean(),
            );
            latest_mean_reward = mean_reward;
        }
        Ok((latest_mean_reward, noise_std))
    }
}
