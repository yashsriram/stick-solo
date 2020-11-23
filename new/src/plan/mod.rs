use bevy::prelude::*;
use ndarray::prelude::*;

fn get_all_vertices(origin: &Vec2, ls: &Array1<f32>, qs: &Array1<f32>) -> Vec<Vec2> {
    let mut vertices = vec![origin.clone()];
    let mut e1 = origin.clone();
    let mut cumulative_rotation = 0f32;
    for i in 0..qs.len() {
        cumulative_rotation += qs[i];
        let e2 = e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * ls[i];
        vertices.push(e2);
        e1 = e2;
    }
    vertices
}

pub fn jacobian_transpose(
    origin: &Vec2,
    ls: &Array1<f32>,
    qs: &Array1<f32>,
    goal: &Vec2,
) -> Array1<f32> {
    let a_i_0 = get_all_vertices(origin, ls, qs);
    // Free end coordinates
    let a_e_0 = *a_i_0.last().unwrap();
    // Building jacobian
    let mut jacobian = Array2::zeros((2, qs.len()));
    for i in 0..qs.len() {
        let a_ie_0 = a_e_0 - a_i_0[i];
        jacobian[(0, i)] = -a_ie_0[1];
        jacobian[(1, i)] = a_ie_0[0];
    }
    // Building delta_x
    let delta_x = arr1(&[goal[0], goal[1]]) - arr1(&[a_e_0[0], a_e_0[1]]);
    // Jacobian transpose
    let delta_q = jacobian.t().dot(&delta_x);
    // 2 x 1 -> 1 x 2
    let delta_q = arr1(&delta_q.iter().map(|&e| e).collect::<Vec<f32>>());
    delta_q
}
