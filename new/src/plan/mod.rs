use bevy::prelude::*;
use ndarray::prelude::*;

pub enum EndControl {
    JacobianTranspose,
    PseudoInverse,
}

pub enum COMXGoalType {
    Pivot,
    PivotGoalMidpoint,
}

fn get_all_vertices_and_com(
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

fn jacobian_transpose(a_i_0: &Vec<Vec2>, goal: &Vec2) -> Array1<f32> {
    let n = a_i_0.len() - 1;
    // Free end coordinates
    let a_e_0 = *a_i_0.last().unwrap();
    // Building jacobian
    let mut jacobian = Array2::zeros((2, n));
    for i in 0..n {
        let a_ie_0 = a_e_0 - a_i_0[i];
        jacobian[(0, i)] = -a_ie_0[1];
        jacobian[(1, i)] = a_ie_0[0];
    }
    // Building delta_x
    let delta_x = arr1(&[goal[0], goal[1]]) - arr1(&[a_e_0[0], a_e_0[1]]);
    // Jacobian transpose
    let delta_q = jacobian.t().dot(&delta_x);
    delta_q
}

fn com_x_sqr_gradient_with_qs(vertices: &Vec<Vec2>, com_x: f32) -> Array1<f32> {
    // len(vertices) = n + 1
    // Calculate y_1 + y_2 + y_3 + ... y_(n-1) + (y_n / 2); y_0 = 0 anyway so include it for cleaner code
    let sum_y_i = vertices.iter().map(|vertex| vertex[1]).sum::<f32>();
    let last_y_i = vertices.last().unwrap()[1];
    let second_term = sum_y_i - (last_y_i / 2.0);
    // Calculate negative gradient of x_c ^ 2 w.r.t. q_i
    let n = vertices.len() - 1;
    let nf = n as f32;
    let mut delta_q_prev = (2.0 * com_x / nf) * second_term;
    let mut delta_q = Vec::with_capacity(n);
    delta_q.push(delta_q_prev);
    for i in 1..n {
        // Actual value
        let delta_q_curr =
            delta_q_prev - (2.0 * com_x / nf) * vertices[i][1] * (nf - (i as f32) + 0.5);
        // Discounted responsibility for sending com_x to origin
        let delta_q_curr = delta_q_curr / (i as f32);
        delta_q_prev = delta_q_curr;
        delta_q.push(delta_q_prev);
    }
    // delta_q corresponds to negative gradient, so take a negative
    let delta_q = -arr1(&delta_q);
    delta_q
}

fn com_y_gradient_with_qs(vertices: &Vec<Vec2>) -> Array1<f32> {
    // len(vertices) = n + 1
    // Calculate x_1 + x_2 + x_3 + ... x_(n-1) + (x_n / 2); x_0 = 0 anyway so include it for cleaner code
    let sum_x_i = vertices.iter().map(|vertex| vertex[0]).sum::<f32>();
    let last_x_i = vertices.last().unwrap()[0];
    let second_term = sum_x_i - (last_x_i / 2.0);
    // Calculate negative gradient of y_c w.r.t. q_i
    let n = vertices.len() - 1;
    let nf = n as f32;
    let mut delta_q_prev = (1.0 / nf) * second_term;
    let mut delta_q = Vec::with_capacity(n);
    delta_q.push(delta_q_prev);
    for i in 1..n {
        // Actual value
        let delta_q_curr = delta_q_prev - (1.0 / nf) * vertices[i][0] * (nf - (i as f32) + 0.5);
        let delta_q_curr = delta_q_curr / (i as f32);
        delta_q_prev = delta_q_curr;
        delta_q.push(delta_q_prev);
    }
    // delta_q corresponds to positive gradient, so directly return
    arr1(&delta_q)
}

pub fn ik(
    origin: &Vec2,
    ls: &Array1<f32>,
    qs: &Array1<f32>,
    goal: &Vec2,
    end_control: EndControl,
    com_x_goal_type: COMXGoalType,
) -> (Array1<f32>, Array1<f32>, Array1<f32>) {
    let (vertices, com) = get_all_vertices_and_com(origin, ls, qs);
    let take_end_to_given_goal = match end_control {
        EndControl::JacobianTranspose => jacobian_transpose(&vertices, &goal),
        EndControl::PseudoInverse => Array1::<f32>::zeros(qs.len()),
    };
    // Shift origin to first vertex
    let origin = vertices[0];
    let vertices = vertices
        .iter()
        .map(|&vertex| vertex - origin)
        .collect::<Vec<Vec2>>();
    // Set com_x goal
    let com_x = com[0];
    let com_x_goal = match com_x_goal_type {
        COMXGoalType::Pivot => origin[0],
        COMXGoalType::PivotGoalMidpoint => (origin[0] + goal[0]) / 2.0,
    };
    let push_com_x_from_its_goal = com_x_sqr_gradient_with_qs(&vertices, com_x - com_x_goal);
    // Make com_y go downward
    let push_com_y_upward = com_y_gradient_with_qs(&vertices);
    (
        take_end_to_given_goal,
        push_com_x_from_its_goal,
        push_com_y_upward,
    )
}
