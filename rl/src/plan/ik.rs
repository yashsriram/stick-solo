use bevy::prelude::*;
use nalgebra::DMatrix;
use nalgebra::DVector;
use nalgebra::Vector2;

pub fn jt_step(a_i_0: &Vec<Vec2>, qs: &DVector<f32>, goal: &Vec2) -> DVector<f32> {
    // Free end coordinates
    let a_e_0 = *a_i_0.last().unwrap();
    // Building jacobian
    let mut jacobian = DMatrix::identity(2, qs.len());
    for i in 0..qs.len() {
        let a_ie_0 = a_e_0 - a_i_0[i];
        jacobian[(0, i)] = -a_ie_0[1];
        jacobian[(1, i)] = a_ie_0[0];
    }
    // Building delta_x
    let delta_x = Vector2::new(goal[0], goal[1]) - Vector2::new(a_e_0[0], a_e_0[1]);
    // Jacobian transpose
    let delta_q = jacobian.transpose() * delta_x;
    delta_q
}
