extern crate stick_solo;

mod encode;
mod world;

use bevy::prelude::*;
use encode::encode;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::BufReader};
use stick_solo::{
    act::{Goal, NRAgent},
    plan::{ceo::CEO, fcn::*},
    vis::nr_agent_plugin::Pause,
    vis::status_bar_plugin::Ticks,
    vis::*,
};
use world::World;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Experiment {
    fcn: FCN,
    ceo: CEO,
    world: World,
}

fn main() {
    let args = env::args();
    let exp = if args.len() == 1 {
        // Optimize
        let pi = std::f32::consts::PI;
        let world = World {
            origin: Vec2::zero(),
            ls: vec![0.4, 0.4],
            qs: vec![(0.0, 2.0 * pi), (0.0, 2.0 * pi)],
            goal: (Vec2::new(-0.0, -0.0), Vec2::new(0.5, 0.5)),
        };
        let mut fcn = FCN::new(vec![
            (world.qs.len() * 2 + 2, Activation::Linear),
            (12, Activation::LeakyReLu(0.1)),
            (12, Activation::Sigmoid),
            (8, Activation::LeakyReLu(0.1)),
            (8, Activation::Sigmoid),
            (world.qs.len(), Activation::Linear),
        ]);
        let ceo = CEO {
            generations: 400,
            batch_size: 50,
            num_episodes: 20,
            num_episode_ticks: 300,
            elite_frac: 0.25,
            initial_std: 3.0,
            noise_factor: 3.0,
            ..Default::default()
        };
        let (mean_reward, _th_std) = ceo.optimize(&mut fcn, &world).unwrap();
        let exp = Experiment {
            fcn: fcn,
            ceo: ceo,
            world: world,
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
    App::build()
        .add_resource(exp.fcn)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(exp.world.clone())
        .add_plugins(base_plugins::BasePlugins)
        .add_plugin(camera_plugin::CameraPlugin)
        .add_plugin(nr_agent_plugin::NRAgentPlugin::new(
            NRAgent::new(
                exp.world.origin,
                &exp.world.ls,
                &exp.world.sample_qs(),
                0.01,
            ),
            Goal(exp.world.sample_goal()),
        ))
        .add_plugin(status_bar_plugin::StatusBarPlugin)
        .add_system(control.system())
        .add_system(reset.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn reset(
    keyboard_input: Res<Input<KeyCode>>,
    world: Res<World>,
    mut agent: ResMut<NRAgent>,
    mut goal: ResMut<Goal>,
    mut ticks: ResMut<Ticks>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        agent.reset(world.origin, &world.ls, &world.sample_qs());
        goal.0 = world.sample_goal();
        ticks.0 = 0;
    }
}

fn control(
    goal: Res<Goal>,
    mut agent: ResMut<NRAgent>,
    fcn: Res<FCN>,
    pause: Res<Pause>,
    mut ticks: ResMut<Ticks>,
) {
    if pause.0 {
        return;
    }
    let delta_qs = fcn.at(&encode(agent.get_current_state(), &goal.0));
    println!("{:?}", delta_qs);
    agent.update(delta_qs);
    ticks.0 += 1;
}
