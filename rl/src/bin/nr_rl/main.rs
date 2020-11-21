extern crate stick_solo;

mod ceo;
mod encode;
mod fcn;
mod reward;

use bevy::prelude::*;
use ceo::CEO;
use encode::generate_input;
use fcn::*;
use reward::NRAgentReward;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::BufReader};
use stick_solo::act::{Goal, NRAgent};
use stick_solo::vis::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Experiment {
    fcn: FCN,
    ceo: CEO,
    reward: NRAgentReward,
}

fn main() {
    let args = env::args();
    let exp = if args.len() == 1 {
        // Optimize
        let reward = NRAgentReward {
            origin: Vec2::zero(),
            ls: vec![0.2, 0.2, 0.2, 0.4],
            qs: vec![0.5, 0.5, 0.5, 0.0],
            goal: Vec2::new(0.2, -0.5),
        };
        let mut fcn = FCN::new(vec![
            (reward.qs.len() * 2 + 2, Activation::Linear),
            (16, Activation::LeakyReLu(0.1)),
            (16, Activation::Sigmoid),
            (8, Activation::LeakyReLu(0.1)),
            (8, Activation::Sigmoid),
            (reward.qs.len(), Activation::Linear),
        ]);
        let ceo = CEO {
            generations: 500,
            batch_size: 50,
            num_episodes: 1,
            num_episode_ticks: 500,
            elite_frac: 0.1,
            initial_std: 2.0,
            noise_factor: 2.0,
            ..Default::default()
        };
        let (mean_reward, _th_std) = ceo.optimize(&mut fcn, &reward).unwrap();
        let exp = Experiment {
            fcn: fcn,
            ceo: ceo,
            reward: reward,
        };
        // Save
        use chrono::{Datelike, Timelike, Utc};
        let now = Utc::now();
        serde_json::to_writer(
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
    App::build()
        .add_resource(exp.fcn)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(base_plugins::BasePlugins)
        .add_plugin(camera_plugin::CameraPlugin)
        .add_plugin(nr_agent_plugin::NRAgentPlugin::new(
            NRAgent::new(exp.reward.origin, &exp.reward.ls, &exp.reward.qs, 0.01),
            Goal(exp.reward.goal),
        ))
        .add_plugin(status_bar_plugin::StatusBarPlugin)
        .add_system(control.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn control(goal: Res<Goal>, mut agent: ResMut<NRAgent>, fcn: Res<FCN>) {
    let delta_qs = fcn.at(&generate_input(agent.get_current_state(), &goal.0));
    println!("{:?}", delta_qs);
    agent.update(delta_qs);
}
