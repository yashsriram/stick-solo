use super::switchable_nr::*;

use bevy::prelude::*;
use ndarray::prelude::*;

#[derive(Clone)]
pub struct SwitchableNRCouple {
    left: SwitchableNR,
    right: SwitchableNR,
}

impl SwitchableNRCouple {
    pub fn new_left_pivot(
        origin_left: Vec2,
        ls_left: &[f32],
        qs_left: &[f32],
        q_clamps_left: &[(f32, f32)],
        ls_right: &[f32],
        qs_right: &[f32],
        q_clamps_right: &[(f32, f32)],
        thickness: f32,
    ) -> Self {
        let left = SwitchableNR::new(
            origin_left,
            ls_left,
            qs_left,
            q_clamps_left,
            PivotingSide::Left,
            thickness,
        );
        let origin_right = left.get_last_vertex();
        let right = SwitchableNR::new(
            origin_right,
            ls_right,
            qs_right,
            q_clamps_right,
            PivotingSide::Left,
            thickness,
        );
        SwitchableNRCouple { left, right }
    }

    pub fn left(&self) -> &SwitchableNR {
        &self.left
    }

    pub fn right(&self) -> &SwitchableNR {
        &self.right
    }

    pub fn update(&mut self, left_delta_qs: Array1<f32>, right_delta_qs: Array1<f32>) {
        let (_, _, _, _, _, left_pivoting_side) = self.left.get_current_state();
        let (_, _, _, _, _, right_pivoting_side) = self.right.get_current_state();
        match (left_pivoting_side, right_pivoting_side) {
            (PivotingSide::Left, PivotingSide::Left) => {
                self.left.update(left_delta_qs);
                let origin_right = self.left.get_last_vertex();
                self.right.set_origin(origin_right);
                self.right.update(right_delta_qs);
            }
            (PivotingSide::Right, PivotingSide::Right) => panic!("Right pivot Not implemented"),
            (PivotingSide::Left, PivotingSide::Right) => panic!("Parallel Not implemented"),
            (PivotingSide::Right, PivotingSide::Left) => panic!("Flying Not implemented"),
        }
    }
}
