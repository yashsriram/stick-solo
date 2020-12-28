pub mod cross_entropy_optimizing;
pub mod gradient_descent;
pub mod random_sampling;

use bevy::prelude::*;
use ndarray::prelude::*;

pub fn get_end_verticex_and_com(origin: &Vec2, ls: &Array1<f32>, qs: &Array1<f32>) -> (Vec2, Vec2) {
    let n = ls.len();
    let mut com = Vec2::zero();
    let mut e1 = origin.clone();
    let mut cumulative_rotation = 0f32;
    for i in 0..n {
        cumulative_rotation += qs[i];
        let e2 = e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * ls[i];
        com += ls[i] * (e1 + e2) / 2.0;
        e1 = e2;
    }
    (e1, com / ls.sum())
}

pub fn get_all_vertices_and_com(
    origin: &Vec2,
    ls: &Array1<f32>,
    qs: &Array1<f32>,
) -> (Vec<Vec2>, Vec2) {
    let n = ls.len();
    let mut com = Vec2::zero();
    let mut vertices = Vec::with_capacity(ls.len() + 1);
    vertices.push(origin.clone());
    let mut e1 = origin.clone();
    let mut cumulative_rotation = 0f32;
    for i in 0..n {
        cumulative_rotation += qs[i];
        let e2 = e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * ls[i];
        vertices.push(e2);
        com += ls[i] * (e1 + e2) / 2.0;
        e1 = e2;
    }
    (vertices, com / ls.sum())
}
