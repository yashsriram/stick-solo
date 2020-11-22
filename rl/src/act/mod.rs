use bevy::prelude::*;
use ndarray::prelude::*;

#[derive(Clone)]
pub struct Goal(pub Vec2);

#[derive(Clone)]
pub struct NRAgent {
    // State
    n: usize,
    origin: Vec2,
    ls: Array1<f32>,
    qs: Array1<f32>,
    // Control
    delta_qs: Array1<f32>,
    // Vis
    thickness: f32,
}

impl NRAgent {
    const MAX_DELTA_Q: f32 = 0.02;

    pub fn new(origin: Vec2, ls: &[f32], qs: &[f32], thickness: f32) -> Self {
        assert_eq!(
            ls.len(),
            qs.len(),
            "Unequal number of lengths and joint angles arguments."
        );
        for i in 0..ls.len() {
            assert!(ls[i] > 0.0, "Non-positive length argument.");
        }
        assert!(thickness > 0.0, "Non-positive thickness argument");
        NRAgent {
            n: ls.len(),
            origin: origin,
            ls: arr1(ls),
            qs: arr1(qs),
            delta_qs: Array1::<f32>::zeros((qs.len(),)),
            thickness: thickness,
        }
    }

    pub fn reset(&mut self, origin: Vec2, ls: &[f32], qs: &[f32]) {
        assert_eq!(
            ls.len(),
            qs.len(),
            "Unequal number of lengths and joint angles arguments."
        );
        for i in 0..ls.len() {
            assert!(ls[i] > 0.0, "Non-positive length argument.");
        }
        self.n = ls.len();
        self.origin = origin;
        self.ls = arr1(ls);
        self.qs = arr1(qs);
        self.delta_qs = Array1::<f32>::zeros((qs.len(),));
    }

    pub fn thickness(&self) -> f32 {
        self.thickness
    }

    pub fn get_current_state(&self) -> (usize, &Vec2, &Array1<f32>, &Array1<f32>) {
        (self.n, &self.origin, &self.ls, &self.qs)
    }

    pub fn get_current_control(&self) -> &Array1<f32> {
        &self.delta_qs
    }

    pub fn update(&mut self, control_delta_qs: Array1<f32>) {
        self.delta_qs = control_delta_qs;
        self.delta_qs.mapv_inplace(|e| {
            if e.abs() > Self::MAX_DELTA_Q {
                Self::MAX_DELTA_Q * e.signum()
            } else {
                e
            }
        });
        self.qs += &self.delta_qs;
    }

    pub fn get_last_vertex(&self) -> Vec2 {
        let mut e1 = self.origin;
        let mut cumulative_rotation = 0f32;
        for i in 0..self.n {
            cumulative_rotation += self.qs[i];
            let e2 =
                e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * self.ls[i];
            e1 = e2;
        }
        e1
    }

    pub fn get_all_vertices(&self) -> Vec<Vec2> {
        let mut vertices = vec![self.origin];
        let mut e1 = self.origin;
        let mut cumulative_rotation = 0f32;
        for i in 0..self.n {
            cumulative_rotation += self.qs[i];
            let e2 =
                e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * self.ls[i];
            vertices.push(e2);
            e1 = e2;
        }
        vertices
    }

    pub fn pose_to_transforms(&self) -> Vec<(Vec2, f32)> {
        let mut transforms = vec![];
        let mut e1 = self.origin;
        let mut cumulative_rotation = 0f32;
        for i in 0..self.n {
            cumulative_rotation += self.qs[i];
            let e2 =
                e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * self.ls[i];
            let midpoint = (e1 + e2) / 2.0;
            transforms.push((midpoint, cumulative_rotation));
            e1 = e2;
        }
        transforms
    }
}
