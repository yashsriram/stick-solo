use bevy::prelude::*;
use ndarray::prelude::*;

pub fn jt_step(a_i_0: &Vec<Vec2>, qs: &Array1<f32>, goal: &Vec2) -> Array1<f32> {
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
