use crate::act::switchable_nr::Side;
use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct World {
    pub holding_side: Side,
    pub origin: Vec2,
    pub holding_ls: Vec<f32>,
    pub holding_q_clamps: Vec<(Option<f32>, Option<f32>)>,
    pub non_holding_ls: Vec<f32>,
    pub non_holding_q_clamps: Vec<(Option<f32>, Option<f32>)>,
    pub unscaled_relative_goal_region: (Vec2, Vec2),
}

impl World {
    fn sample_qs(q_clamps: &[(Option<f32>, Option<f32>)]) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        q_clamps
            .iter()
            .map(|(min, max)| {
                if *min == None || *max == None {
                    0.0
                } else {
                    rng.gen_range(min.unwrap(), max.unwrap())
                }
            })
            .collect()
    }

    pub fn sample_holding_qs(&self) -> Vec<f32> {
        World::sample_qs(&self.holding_q_clamps)
    }

    pub fn sample_non_holding_qs(&self) -> Vec<f32> {
        World::sample_qs(&self.non_holding_q_clamps)
    }

    fn get_q_clamps(q_clamps: &[(Option<f32>, Option<f32>)]) -> Vec<(f32, f32)> {
        let inf = f32::INFINITY;
        q_clamps
            .iter()
            .map(|(min, max)| match (min, max) {
                (None, None) => (-inf, inf),
                (None, Some(l)) => (-inf, *l),
                (Some(l), None) => (*l, inf),
                (Some(l1), Some(l2)) => (*l1, *l2),
            })
            .collect()
    }

    pub fn holding_q_clamps(&self) -> Vec<(f32, f32)> {
        World::get_q_clamps(&self.holding_q_clamps)
    }

    pub fn non_holding_q_clamps(&self) -> Vec<(f32, f32)> {
        World::get_q_clamps(&self.non_holding_q_clamps)
    }

    pub fn sample_goal(&self) -> Vec2 {
        let (min, max) = self.unscaled_relative_goal_region;
        let scale = self.holding_ls.iter().sum::<f32>() + self.non_holding_ls.iter().sum::<f32>();
        let (min, max) = (min * scale, max * scale);
        let diff = max - min;
        let rand_diff = Vec2::new(
            rand::random::<f32>() * diff[0],
            rand::random::<f32>() * diff[1],
        );
        self.origin + min + rand_diff
    }
}

impl Plugin for World {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone())
            .add_startup_system(init_vis);
    }
}

fn init_vis(mut commands: Commands, world: Res<World>) {
    let (min, max) = world.unscaled_relative_goal_region;
    let scale = world.holding_ls.iter().sum::<f32>() + world.non_holding_ls.iter().sum::<f32>();
    let (min, max) = (min * scale, max * scale);
    let midpoint = world.origin + (min + max) / 2.0;
    let diff = max - min;
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(diff[0], diff[1])),
            color: Color::rgba(1.0, 0.0, 0.0, 0.02),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(midpoint[0], midpoint[1], 0.0)),
        ..Default::default()
    });
}
