extern crate stick_solo;
mod world;

use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use ndarray::prelude::*;
use std::{env, fs::File, io::BufReader};
use stick_solo::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use stick_solo::act::switchable_nr::Side;
use stick_solo::game::{
    goal_couple_plugin::{GoalCouple, GoalCouplePlugin},
    one_holding_switchable_nr_couple_plugin::OneHoldingSwitchableNRCouplePlugin,
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
};
use stick_solo::plan::cross_entropy_optimizing::ceo::CEO;
use stick_solo::plan::cross_entropy_optimizing::experiment::Experiment;
use stick_solo::plan::cross_entropy_optimizing::fcn::*;
use stick_solo::plan::cross_entropy_optimizing::utils::{
    control, decode, encode, random_sample_solve, GoalQsCouple,
};
use stick_solo::plan::cross_entropy_optimizing::world::World;
use world::Wrapper;

fn main() {
    let args = env::args();
    let exp = if args.len() == 1 {
        // Optimize
        let pi = std::f32::consts::PI;
        let wrapper = Wrapper(World {
            holding_side: Side::Left,
            origin: Vec2::new(0.0, 0.0),
            holding_ls: vec![0.2, 0.2],
            holding_q_clamps: vec![(None, None), (Some(-pi), Some(-0.0))],
            non_holding_ls: vec![0.2, 0.2],
            non_holding_q_clamps: vec![(None, None), (Some(-pi), Some(-0.0))],
            unscaled_relative_goal_region: (Vec2::new(-0.8, -0.8), Vec2::new(0.1, 0.8)),
        });
        // let wrapper = Wrapper(World {
        //     holding_side: Side::Left,
        //     origin: Vec2::new(0.0, 0.0),
        //     holding_ls: vec![0.2, 0.2],
        //     holding_q_clamps: vec![(None, None), (Some(0.0), Some(pi))],
        //     non_holding_ls: vec![0.2, 0.2],
        //     non_holding_q_clamps: vec![(None, None), (Some(0.0), Some(pi))],
        //     unscaled_relative_goal_region: (Vec2::new(-0.1, -0.8), Vec2::new(0.8, 0.8)),
        // });
        let mut fcn = FCN::new(vec![
            (
                wrapper.0.holding_ls.len() + wrapper.0.non_holding_ls.len() + 2,
                Activation::Linear,
            ),
            (16, Activation::LeakyReLu(0.1)),
            (16, Activation::LeakyReLu(0.1)),
            (2, Activation::Linear),
        ]);
        let ceo = CEO {
            generations: 500,
            batch_size: 50,
            num_episodes: 20,
            num_episode_ticks: 200,
            elite_frac: 0.25,
            initial_std: 1.0,
            noise_factor: 1.0,
            ..Default::default()
        };
        let (mean_reward, _th_std) = ceo.optimize(&mut fcn, &wrapper).unwrap();
        let exp = Experiment {
            fcn: fcn,
            ceo: ceo,
            world: wrapper.0,
        };
        // Save
        use chrono::{Datelike, Timelike, Utc};
        let now = Utc::now();
        serde_json::to_writer_pretty(
            &File::create(format!(
                "{}-{}:{}@{:.2}.json",
                now.month(),
                now.day(),
                now.num_seconds_from_midnight(),
                mean_reward
            ))
            .unwrap(),
            &exp,
        )
        .unwrap();
        exp
    } else {
        if args.len() != 2 {
            panic!("Bad cmd line parameters.");
        }
        // Load from file
        let args = args.collect::<Vec<String>>();
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);
        let exp: Experiment = serde_json::from_reader(reader).unwrap();
        exp
    };
    println!("{:?}", exp);

    // Visualize
    let world = exp.world.clone();
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(AssetServerSettings {
            asset_folder: "static/assets".to_string(),
            watch_for_changes: false,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
            commands.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(0.0, 0.0, 4.0),
                ..default()
            });
        })
        .add_plugin(exp.world)
        .insert_resource(exp.fcn)
        .insert_resource(GoalQsCouple(Array::zeros(0), Array::zeros(0)))
        .add_plugin(OneHoldingSwitchableNRCouplePlugin::new(
            OneHoldingSwitchableNRCouple::new(
                &world.holding_side,
                world.origin,
                &world.holding_ls,
                &world.sample_holding_qs(),
                &world.holding_q_clamps(),
                &world.non_holding_ls,
                &world.sample_non_holding_qs(),
                &world.non_holding_q_clamps(),
            ),
        ))
        .add_plugin(GoalCouplePlugin::new(GoalCouple(
            Vec2::new(0.0, 0.0),
            world.sample_goal(),
        )))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_startup_system(initial_set_goal_qs_couple_system)
        .add_system(interactive_set_goal_qs_couple_system)
        .add_system(control_system)
        .run();
}

fn set_goal_qs_couple(
    agent: &OneHoldingSwitchableNRCouple,
    goal_qs_couple: &mut GoalQsCouple,
    goal_couple: &mut GoalCouple,
    fcn: &FCN,
) {
    let holding_origin = agent.holding().get_current_state().1.clone();
    let non_holding_goal = goal_couple.1;
    // Network pipeline
    let (input, scale) = encode(&agent, &non_holding_goal);
    let forward_pass = fcn.at(&input);
    let holding_goal = decode(&forward_pass, scale, holding_origin);
    // Setting GoalCouple and GoalQsCouple
    *goal_couple = GoalCouple(holding_goal, non_holding_goal);
    random_sample_solve(agent, goal_couple, goal_qs_couple);
}

fn initial_set_goal_qs_couple_system(
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut goal_qs_couple: ResMut<GoalQsCouple>,
    mut goal_couple: ResMut<GoalCouple>,
    fcn: Res<FCN>,
) {
    set_goal_qs_couple(&agent, &mut goal_qs_couple, &mut goal_couple, &fcn);
}

fn interactive_set_goal_qs_couple_system(
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut goal_qs_couple: ResMut<GoalQsCouple>,
    mut ticks: ResMut<Ticks>,
    mut goal_couple: ResMut<GoalCouple>,
    fcn: Res<FCN>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::I)
        || keyboard_input.pressed(KeyCode::K)
        || keyboard_input.pressed(KeyCode::J)
        || keyboard_input.pressed(KeyCode::L)
    {
        set_goal_qs_couple(&agent, &mut goal_qs_couple, &mut goal_couple, &fcn);
        ticks.0 = 0;
    }
}

fn control_system(
    mut agent: ResMut<OneHoldingSwitchableNRCouple>,
    pause: Res<Pause>,
    mut ticks: ResMut<Ticks>,
    goal_qs_couple: Res<GoalQsCouple>,
    goal_couple: ResMut<GoalCouple>,
) {
    if pause.0 {
        return;
    }
    control(&mut agent, &goal_qs_couple, &goal_couple, ticks.0);
    ticks.0 += 1;
}

use ndarray::prelude::*;
use stick_solo::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use stick_solo::act::switchable_nr::SwitchableNR;
use stick_solo::game::goal_couple_plugin::GoalCouple;
use stick_solo::plan::cross_entropy_optimizing::ceo::Reward;
use stick_solo::plan::cross_entropy_optimizing::fcn::*;
use stick_solo::plan::cross_entropy_optimizing::utils::{
    control, decode, encode, random_sample_solve, GoalQsCouple,
};
use stick_solo::plan::cross_entropy_optimizing::world::World;

pub struct Wrapper(pub World);

impl Reward for Wrapper {
    fn average_reward(
        &self,
        fcn: &FCN,
        params: &Array1<f32>,
        num_episodes: usize,
        num_episode_ticks: usize,
    ) -> f32 {
        let mut cumulative_reward = 0.0;
        for _ in 0..num_episodes {
            // Spawn agent
            let mut agent = OneHoldingSwitchableNRCouple::new(
                &self.0.holding_side,
                self.0.origin,
                &self.0.holding_ls,
                &self.0.sample_holding_qs(),
                &self.0.holding_q_clamps(),
                &self.0.non_holding_ls,
                &self.0.sample_non_holding_qs(),
                &self.0.non_holding_q_clamps(),
            );
            let holding_origin = agent.holding().get_current_state().1.clone();
            let non_holding_goal = self.0.sample_goal();
            // Network pipeline
            let (input, scale) = encode(&agent, &non_holding_goal);
            let forward_pass = fcn.at_with(&input, params);
            let holding_goal = decode(&forward_pass, scale, holding_origin);
            // Setting GoalCouple and GoalQsCouple
            let goal_couple = GoalCouple(holding_goal, non_holding_goal);
            let mut goal_qs_couple = GoalQsCouple(Array::zeros(0), Array::zeros(0));
            random_sample_solve(&agent, &goal_couple, &mut goal_qs_couple);
            // Start calculating reward
            let mut episode_reward = 0.0;
            for ticks in 0..num_episode_ticks {
                // Apply control
                control(&mut agent, &goal_qs_couple, &goal_couple, ticks);
                // Holding
                let last_vertex = agent.holding().get_last_vertex();
                let dist = (last_vertex - holding_goal).length();
                episode_reward -= 2.0 * dist;
                // Non holding
                let last_vertex = agent.non_holding().get_last_vertex();
                let dist = (last_vertex - non_holding_goal).length();
                episode_reward -= 10.0 * dist;
                // COM y
                let com = agent.get_center_of_mass();
                episode_reward -= 5.0 * com[1];
                // COM x
                let com = agent.get_center_of_mass();
                episode_reward -= (com[0] - (non_holding_goal[0] + holding_origin[0]) / 2.0).abs();
            }
            // Holding
            let last_vertex = agent.holding().get_last_vertex();
            let dist = (last_vertex - holding_goal).length();
            if dist < SwitchableNR::GOAL_REACHED_SLACK {
                episode_reward += 500.0;
            }
            // Non holding
            let last_vertex = agent.non_holding().get_last_vertex();
            let dist = (last_vertex - non_holding_goal).length();
            if dist < SwitchableNR::GOAL_REACHED_SLACK {
                episode_reward += 1000.0;
            }

            cumulative_reward += episode_reward;
        }

        let average_reward = cumulative_reward / num_episodes as f32;
        average_reward
    }
}
