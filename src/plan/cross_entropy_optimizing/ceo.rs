use super::fcn::*;
use ndarray::{prelude::*, stack};
use ndarray_rand::{
    rand_distr::{NormalError, StandardNormal},
    RandomExt,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

pub trait Reward {
    fn average_reward(
        &self,
        fcn: &FCN,
        params: &Array1<f32>,
        num_episodes: usize,
        num_episode_ticks: usize,
    ) -> f32;
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
    pub fn optimize(
        &self,
        fcn: &mut FCN,
        reward: &(dyn Reward + std::marker::Sync),
    ) -> Result<(f32, Array1<f32>), NormalError> {
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
                            reward.average_reward(
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
                reward.average_reward(
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
