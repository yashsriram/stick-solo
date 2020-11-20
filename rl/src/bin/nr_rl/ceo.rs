use super::fcn::*;
use ndarray::prelude::*;
use ndarray::stack;
use ndarray_rand::rand_distr::{NormalError, StandardNormal};
use ndarray_rand::RandomExt;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

fn reward(ls: &[f32], fcn: &FCN, params: &Array1<f32>, num_episodes: usize) -> f32 {
    // let mut cumulative_reward = 0.0;
    // for _ in 0..num_episodes {
    //     // Set goal
    //     let goal_coordinates =
    //         Goal::in_region(self.goal_x_bounds, self.goal_y_bounds).coordinates();
    //     // Spawn agent
    //     let mut model = NRAgent::spawn_randomly(
    //         self.start_x_bounds,
    //         self.start_y_bounds,
    //         self.start_or_bounds,
    //         self.radius,
    //         goal_coordinates,
    //     );
    //     // Start calculating reward
    //     let mut episode_reward = 0.0;
    //     for tick in 0..self.num_episode_ticks {
    //         // Curr state
    //         let (x, y, or_in_rad) = model.scaled_state();
    //         // Control for curr state
    //         let control = fcn.at_with(&arr1(&[x, y, or_in_rad]), params);
    //         let (v, w) = (control[[0]], control[[1]]);
    //         // Apply control
    //         model.set_control(v, w);
    //         model.update(0.1).unwrap();
    //         // Next state
    //         let (x, y, or_in_rad) = model.scaled_state();
    //         // Makes agent orient towards goal
    //         let (x_hat, y_hat) = {
    //             let norm = (x * x + y * y).sqrt();
    //             (x / norm, y / norm)
    //         };
    //         let angular_deviation = ((x_hat - or_in_rad.cos()).powf(2.0)
    //             + (y_hat - or_in_rad.sin()).powf(2.0))
    //         .sqrt()
    //             * (1.0 / (1.0 + tick as f32));
    //         episode_reward -= angular_deviation;
    //         // Removes rotational jitter
    //         episode_reward -= w.abs();
    //         // Makes agent translate towards goal
    //         let dist = (x * x + y * y).sqrt();
    //         episode_reward -= dist * 30.0;
    //     }
    //     // Makes agent reach the goal at the end of episode
    //     let (x, y, _or_in_rad) = model.scaled_state();
    //     let final_dist = (x * x + y * y).sqrt();
    //     episode_reward += 200.0 * (-final_dist).exp();
    //     // Makes agent stop at the end of episode
    //     let (v, w) = model.control();
    //     episode_reward += 200.0 * (-v.abs()).exp() * (-final_dist).exp();
    //     episode_reward += 200.0 * (-w.abs()).exp() * (-final_dist).exp();

    //     cumulative_reward += episode_reward;
    // }

    // let average_reward = cumulative_reward / num_episodes as f32;
    // average_reward
    0.0
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CEO {
    pub generations: usize,
    pub batch_size: usize,
    pub num_evalation_samples: usize,
    pub elite_frac: f32,
    pub initial_std: f32,
    pub noise_factor: f32,
}

impl Default for CEO {
    fn default() -> CEO {
        CEO {
            generations: 300,
            batch_size: 50,
            num_evalation_samples: 300,
            elite_frac: 0.25,
            initial_std: 2.0,
            noise_factor: 2.0,
        }
    }
}

impl CEO {
    pub fn optimize(&self, ls: &[f32], fcn: &mut FCN) -> Result<Array1<f32>, NormalError> {
        let n_elite = (self.batch_size as f32 * self.elite_frac).round().floor() as usize;
        let mut noise_std = Array::from_elem((fcn.params().len(),), self.initial_std);
        for generation in 0..self.generations {
            let (sorted_th_means, mean_reward) = {
                let mut reward_th_mean_tuples = (0..self.batch_size)
                    .into_par_iter()
                    .map(|_| {
                        let randn_noise: Array1<f32> =
                            Array::random(fcn.params().len(), StandardNormal);
                        let scaled_randn_noise = randn_noise * &noise_std;
                        let perturbed_params = scaled_randn_noise + fcn.params();
                        (
                            reward(ls, fcn, &perturbed_params, self.num_evalation_samples),
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
                reward(ls, fcn, &fcn.params(), self.num_evalation_samples),
                noise_std.mean(),
            );
        }
        Ok(noise_std)
    }
}
