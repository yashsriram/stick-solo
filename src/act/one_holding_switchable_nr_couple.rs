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
    pub fn new(
        holding_side: &Side,
        origin_holding: Vec2,
        ls_holding: &[f32],
        qs_holding: &[f32],
        q_clamps_holding: &[(f32, f32)],
        ls_non_holding: &[f32],
        qs_non_holding: &[f32],
        q_clamps_non_holding: &[(f32, f32)],
    ) -> Self {
        let holding = SwitchableNR::new(
            origin_holding,
            ls_holding,
            qs_holding,
            q_clamps_holding,
            holding_side.clone(),
        );
        let origin_non_holding = holding.get_last_vertex();
        let non_holding = SwitchableNR::new(
            origin_non_holding,
            ls_non_holding,
            qs_non_holding,
            q_clamps_non_holding,
            holding_side.clone(),
        );
        OneHoldingSwitchableNRCouple {
            holding,
            non_holding,
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

    pub fn get_center_of_mass(&self) -> Vec2 {
        let m1 = self.holding().get_total_mass();
        let com1 = self.holding().get_center_of_mass();
        let m2 = self.non_holding().get_total_mass();
        let com2 = self.non_holding().get_center_of_mass();

        (m1 * com1 + m2 * com2) / (m1 + m2)
    }

    pub fn update(&mut self, holding_delta_qs: Array1<f32>, non_holding_delta_qs: Array1<f32>) {
        self.holding.update(holding_delta_qs);
        let origin_non_holding = self.holding.get_last_vertex();
        self.non_holding.set_origin(origin_non_holding);
        self.non_holding.update(non_holding_delta_qs);
    }

    pub fn switch_hold(&mut self) {
        // Switch pivot
        self.non_holding.switch_pivot();
        self.holding.switch_pivot();
        // Swap
        let prev_holding = self.holding.clone();
        self.holding = self.non_holding.clone();
        self.non_holding = prev_holding;
        self.is_holding_as_initialized = !self.is_holding_as_initialized;
    }
}
