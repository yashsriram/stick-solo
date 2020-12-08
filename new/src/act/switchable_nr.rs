use bevy::prelude::*;
use ndarray::prelude::*;

#[derive(Clone)]
pub enum PivotingSide {
    Left,
    Right,
}

#[derive(Clone)]
pub struct SwitchableNR {
    // State
    n: usize,
    origin: Vec2,
    ls: Array1<f32>,
    qs: Array1<f32>,
    q_clamps: Array1<(f32, f32)>,
    pivoting_side: PivotingSide,
    // Control
    delta_qs: Array1<f32>,
    // Vis
    thickness: f32,
}

impl SwitchableNR {
    pub const GOAL_REACHED_SLACK: f32 = 0.01;
    const MAX_DELTA_Q: f32 = 0.02;

    pub fn new(
        origin: Vec2,
        ls: &[f32],
        qs: &[f32],
        q_clamps: &[(f32, f32)],
        pivoting_side: PivotingSide,
        thickness: f32,
    ) -> Self {
        assert!(ls.len() > 0, "Zero links argument.");
        assert_eq!(
            ls.len(),
            qs.len(),
            "Unequal number of lengths and joint angles arguments."
        );
        assert_eq!(
            ls.len(),
            q_clamps.len(),
            "Unequal number of lengths and joint angle clamps arguments."
        );
        for i in 0..ls.len() {
            assert!(ls[i] > 0.0, "Non-positive length argument.");
            assert!(
                q_clamps[i].0 < q_clamps[i].1,
                format!("Lower clamp greater than or equal to upper clamp.")
            );
            assert!(q_clamps[i].0 <= qs[i], "Disobidient q arguement.");
            assert!(qs[i] <= q_clamps[i].1, "Disobidient q arguement.");
        }
        assert!(
            q_clamps[0] == (-f32::INFINITY, f32::INFINITY),
            "First q clamp has to be (-inf, inf)."
        );
        assert!(thickness > 0.0, "Non-positive thickness argument.");
        SwitchableNR {
            n: ls.len(),
            origin: origin,
            ls: arr1(ls),
            qs: arr1(qs),
            q_clamps: arr1(q_clamps),
            pivoting_side: pivoting_side,
            delta_qs: Array1::<f32>::zeros((qs.len(),)),
            thickness: thickness,
        }
    }

    pub fn switch_pivot(&mut self) {
        self.origin = self.get_last_vertex();
        self.ls = arr1(&self.ls.to_vec().into_iter().rev().collect::<Vec<f32>>());
        // qs
        let qs_sum = self.qs.sum();
        let mut last_n_1_qs = self
            .qs
            .to_vec()
            .into_iter()
            .skip(1)
            .rev()
            .map(|e| -e)
            .collect::<Vec<f32>>();
        let mut qs = vec![qs_sum - std::f32::consts::PI];
        qs.append(&mut last_n_1_qs);
        self.qs = arr1(&qs);
        // q_clamps
        let mut last_n_1_q_clamps = self
            .q_clamps
            .to_vec()
            .into_iter()
            .skip(1)
            .rev()
            .map(|(min, max)| (-max, -min))
            .collect::<Vec<(f32, f32)>>();
        let mut q_clamps = vec![(-f32::INFINITY, f32::INFINITY)];
        q_clamps.append(&mut last_n_1_q_clamps);
        self.q_clamps = arr1(&q_clamps);
        self.delta_qs *= 0.0;
        // pivoting_side
        self.pivoting_side = match self.pivoting_side {
            PivotingSide::Left => PivotingSide::Right,
            PivotingSide::Right => PivotingSide::Left,
        };
    }

    pub fn thickness(&self) -> f32 {
        self.thickness
    }

    pub fn get_current_state(&self) -> (usize, &Vec2, &Array1<f32>, &Array1<f32>, &PivotingSide) {
        (
            self.n,
            &self.origin,
            &self.ls,
            &self.qs,
            &self.pivoting_side,
        )
    }

    pub fn get_current_control(&self) -> &Array1<f32> {
        &self.delta_qs
    }

    pub fn update(&mut self, control_delta_qs: Array1<f32>) {
        self.delta_qs = control_delta_qs;
        self.delta_qs.mapv_inplace(|delta_q| {
            if delta_q.abs() > Self::MAX_DELTA_Q {
                Self::MAX_DELTA_Q * delta_q.signum()
            } else {
                delta_q
            }
        });
        self.qs += &self.delta_qs;
        for i in 0..self.n {
            let (min, max) = self.q_clamps[i];
            if self.qs[i] < min {
                self.qs[i] = min
            } else if self.qs[i] > max {
                self.qs[i] = max
            }
        }
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
        let mut vertices = Vec::with_capacity(self.n + 1);
        vertices.push(self.origin);
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

    pub fn get_center_of_mass(&self) -> Vec2 {
        let mut com = Vec2::zero();
        let mut e1 = self.origin;
        let mut cumulative_rotation = 0f32;
        for i in 0..self.n {
            cumulative_rotation += self.qs[i];
            let e2 =
                e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * self.ls[i];
            com += self.ls[i] * (e1 + e2) / 2.0;
            e1 = e2;
        }
        com / self.ls.sum()
    }

    pub fn pose_to_transforms(&self) -> Vec<(Vec2, f32)> {
        let mut transforms = Vec::with_capacity(self.n);
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
