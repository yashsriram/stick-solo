use bevy::prelude::*;
use nalgebra::DVector;

pub struct NRAgent {
    n: usize,
    origin: Vec2,
    l: DVector<f32>,
    q: DVector<f32>,
    thickness: f32,
}

impl NRAgent {
    pub fn new(origin: Vec2, l: Vec<f32>, q: Vec<f32>, thickness: f32) -> Self {
        assert_eq!(
            l.len(),
            q.len(),
            "Unequal number of lengths and joint angles arguments."
        );
        assert!(thickness > 0.0, "Non-positive thickness argument");
        NRAgent {
            n: l.len(),
            origin: origin,
            l: DVector::from_iterator(l.len(), l.into_iter()),
            q: DVector::from_iterator(q.len(), q.into_iter()),
            thickness: thickness,
        }
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn l(&self) -> &DVector<f32> {
        &self.l
    }

    pub fn q(&self) -> &DVector<f32> {
        &self.q
    }

    pub fn q_pluseq(&mut self, delta_q: &DVector<f32>) {
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
