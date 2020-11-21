extern crate stick_solo;

mod ceo;
mod fcn;

use bevy::prelude::*;
use ceo::CEO;
use fcn::*;
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;
use stick_solo::act::{Goal, NRAgent};
use stick_solo::vis::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Experiment {
    fcn: FCN,
    ceo: CEO,
}

fn main() {
    let ls = [0.2, 0.2, 0.2, 0.2];
    let args = env::args();
    let exp = if args.len() == 1 {
        let mut fcn = FCN::new(vec![
            (2 + ls.len() + ls.len() + 2, Activation::Linear),
            (5, Activation::LeakyReLu(0.1)),
            (5, Activation::LeakyReLu(0.1)),
            (5, Activation::LeakyReLu(0.1)),
            (5, Activation::LeakyReLu(0.1)),
            (ls.len(), Activation::Linear),
        ]);
        let ceo = CEO {
            generations: 500,
            batch_size: 100,
            num_evalation_samples: 1,
            num_episode_ticks: 100,
            elite_frac: 0.25,
            initial_std: 2.0,
            noise_factor: 2.0,
            ..Default::default()
        };
        let (mean_reward, _th_std) = ceo.optimize(&ls, &mut fcn).unwrap();
        let exp = Experiment { fcn: fcn, ceo: ceo };
        // Save
        use chrono::{Datelike, Timelike, Utc};
        let now = Utc::now();
        serde_json::to_writer(
            &File::create(format!(
                "{}-{}:{}@{:2}.json",
                now.day(),
                now.month(),
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
            NRAgent::new(Vec2::new(0.0, 0.0), &ls, &[0.5, -0.1, -0.6, -0.1], 0.01),
            Goal(Vec2::new(0.5, 0.0)),
        ))
        .add_plugin(fps_plugin::FPSPlugin)
        .add_system(control.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn control(goal: Res<Goal>, mut agent: ResMut<NRAgent>, fcn: Res<FCN>) {
    let (_, origin, ls, qs) = agent.get_current_state();
    // let delta_qs = stick_solo::plan::jacobian_transpose(origin, ls, qs, &goal.0);
    // agent.update(delta_qs);
    let mut input = vec![origin[0], origin[1]];
    input.append(&mut ls.to_vec());
    input.append(&mut qs.to_vec());
    input.push(goal.0[0]);
    input.push(goal.0[1]);
    // Control for curr state
    let mut delta_qs = fcn.at(&arr1(&input));
    let delta_qs_norm = delta_qs.mapv(|e| e * e).sum().sqrt();
    if delta_qs_norm > 0.1 {
        delta_qs = delta_qs / delta_qs_norm * 0.1;
    }
    // Apply control
    agent.update(delta_qs);
}
