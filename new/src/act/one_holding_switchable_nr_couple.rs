use super::switchable_nr::*;

use bevy::prelude::*;
use ndarray::prelude::*;

#[derive(Clone)]
pub struct OneHoldingSwitchableNRCouple {
    holding: SwitchableNR,
    non_holding: SwitchableNR,
    is_holding_as_initialized: bool,
}

impl OneHoldingSwitchableNRCouple {
    pub fn new_left_holding(
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
        OneHoldingSwitchableNRCouple {
            holding: left,
            non_holding: right,
            is_holding_as_initialized: true,
        }
    }

    pub fn new_right_holding(
        origin_right: Vec2,
        ls_right: &[f32],
        qs_right: &[f32],
        q_clamps_right: &[(f32, f32)],
        ls_left: &[f32],
        qs_left: &[f32],
        q_clamps_left: &[(f32, f32)],
        thickness: f32,
    ) -> Self {
        let right = SwitchableNR::new(
            origin_right,
            ls_right,
            qs_right,
            q_clamps_right,
            PivotingSide::Right,
            thickness,
        );
        let origin_left = right.get_last_vertex();
        let left = SwitchableNR::new(
            origin_left,
            ls_left,
            qs_left,
            q_clamps_left,
            PivotingSide::Right,
            thickness,
        );
        OneHoldingSwitchableNRCouple {
            holding: right,
            non_holding: left,
            is_holding_as_initialized: true,
        }
    }

    pub fn original_holding(&self) -> &SwitchableNR {
        if self.is_holding_as_initialized {
            &self.holding
        } else {
            &self.non_holding
        }
    }

    pub fn original_non_holding(&self) -> &SwitchableNR {
        if self.is_holding_as_initialized {
            &self.non_holding
        } else {
            &self.holding
        }
    }

    pub fn holding(&self) -> &SwitchableNR {
        &self.holding
    }

    pub fn non_holding(&self) -> &SwitchableNR {
        &self.non_holding
    }

    pub fn update(&mut self, holding_delta_qs: Array1<f32>, non_holding_delta_qs: Array1<f32>) {
        self.holding.update(holding_delta_qs);
        let origin_non_holding = self.holding.get_last_vertex();
        self.non_holding.set_origin(origin_non_holding);
        self.non_holding.update(non_holding_delta_qs);
    }
}
