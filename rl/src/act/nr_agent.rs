use bevy::prelude::*;
use ndarray::prelude::*;

pub struct NRAgent {
    n: usize,
    origin: Vec2,
    l: Array1<f32>,
    q: Array1<f32>,
    thickness: f32,
}

impl NRAgent {
    pub fn new(origin: Vec2, l: &[f32], q: &[f32], thickness: f32) -> Self {
        assert_eq!(
            l.len(),
            q.len(),
            "Unequal number of lengths and joint angles arguments."
        );
        for i in 0..l.len() {
            assert!(l[i] > 0.0, "Non-zero length argument.");
        }
        assert!(thickness > 0.0, "Non-positive thickness argument");
        NRAgent {
            n: l.len(),
            origin: origin,
            l: arr1(l),
            q: arr1(q),
            thickness: thickness,
        }
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn l(&self) -> &Array1<f32> {
        &self.l
    }

    pub fn q(&self) -> &Array1<f32> {
        &self.q
    }

    pub fn q_pluseq(&mut self, delta_q: &Array1<f32>) {
        self.q += delta_q;
    }

    pub fn thickness(&self) -> f32 {
        self.thickness
    }

    pub fn pose_to_transforms(&self) -> Vec<(Vec2, f32)> {
        let mut transforms = vec![];
        let mut e1 = self.origin;
        let mut cumulative_rotation = 0f32;
        for i in 0..self.n {
            cumulative_rotation += self.q[i];
            let e2 =
                e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * self.l[i];
            let midpoint = (e1 + e2) / 2.0;
            transforms.push((midpoint, cumulative_rotation));
            e1 = e2;
        }
        transforms
    }

    pub fn get_vertices(&self) -> Vec<Vec2> {
        let mut vertices = vec![self.origin];
        let mut e1 = self.origin;
        let mut cumulative_rotation = 0f32;
        for i in 0..self.n {
            cumulative_rotation += self.q[i];
            let e2 =
                e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * self.l[i];
            vertices.push(e2);
            e1 = e2;
        }
        vertices
    }
}
