use bevy::prelude::*;
use ndarray::prelude::*;

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
