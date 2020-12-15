use super::*;
use crate::act::switchable_nr::Side;
use bevy::prelude::*;
use ndarray::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use rand::prelude::*;
use rayon::prelude::*;

fn get_q0_clamp(q0: f32, pivoting_side: &Side) -> (f32, f32) {
    let pi = std::f32::consts::PI;
    match pivoting_side {
        Side::Left => {
            let factor = (q0 + pi) / (2.0 * pi);
            let base = factor.floor() * 2.0 * pi;
            (base - pi * 5.0 / 6.0, base + pi * 2.0 / 6.0)
        }
        Side::Right => {
            let factor = q0 / (2.0 * pi);
            let base = factor.floor() * 2.0 * pi;
            (base + pi * 4.0 / 6.0, base + pi * 11.0 / 6.0)
        }
    }
}

pub fn no_prior_random_sample_optimizer<F>(
    num_samples: usize,
    origin: &Vec2,
    ls: &Array1<f32>,
    q0: f32,
    pivoting_side: &Side,
    q_clamps: &Array1<(f32, f32)>,
    goal: &Vec2,
    loss_fn: F,
) -> (f32, Array1<f32>)
where
    F: Fn(&Vec2, &Vec2, &Vec2) -> f32 + Send + Sync,
{
    let q0_clamp = get_q0_clamp(q0, pivoting_side);
    (0..num_samples)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let new_qs = q_clamps
                .iter()
                .enumerate()
                .map(|(i, clamp)| {
                    if i == 0 {
                        rng.gen_range(q0_clamp.0, q0_clamp.1)
                    } else {
                        rng.gen_range(clamp.0, clamp.1)
                    }
                })
                .collect::<Array1<f32>>();
            let (end, com) = get_end_verticex_and_com(origin, ls, &new_qs);
            (loss_fn(&end, &com, goal), new_qs)
        })
        .min_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
        .unwrap()
}

pub fn from_current_state_random_sample_optimizer<F>(
    num_samples: usize,
    q_mutation: f32,
    n: usize,
    origin: &Vec2,
    ls: &Array1<f32>,
    qs: &Array1<f32>,
    pivoting_side: &Side,
    q_clamps: &Array1<(f32, f32)>,
    goal: &Vec2,
    loss_fn: F,
) -> (f32, Array1<f32>)
where
    F: Fn(&Vec2, &Vec2, &Vec2) -> f32 + Send + Sync,
{
    let (q0_min, q0_max) = get_q0_clamp(qs[0], pivoting_side);
    (0..num_samples)
        .into_par_iter()
        .map(|_| {
            let mutation = Array::random(qs.len(), Uniform::new(-q_mutation, q_mutation));
            let mut new_qs = qs + &mutation;
            if new_qs[0] < q0_min {
                new_qs[0] = q0_min
            } else if new_qs[0] > q0_max {
                new_qs[0] = q0_max
            }
            for i in 1..n {
                let (min, max) = q_clamps[i];
                if new_qs[i] < min {
                    new_qs[i] = min
                } else if new_qs[i] > max {
                    new_qs[i] = max
                }
            }
            let (end, com) = get_end_verticex_and_com(origin, ls, &new_qs);
            (loss_fn(&end, &com, goal), new_qs)
        })
        .min_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
        .unwrap()
}
