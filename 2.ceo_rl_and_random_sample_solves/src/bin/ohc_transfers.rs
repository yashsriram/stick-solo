extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use std::{env, fs::File, io::BufReader};
use stick_solo::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use stick_solo::act::switchable_nr::*;
use stick_solo::game::{
    base_plugins::BasePlugins,
    camera_plugin::CameraPlugin,
    goal_couple_plugin::{GoalCouple, GoalCouplePlugin},
    one_holding_switchable_nr_couple_plugin::OneHoldingSwitchableNRCouplePlugin,
    path_plugin::{Path, PathPlugin},
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
};
use stick_solo::plan::cross_entropy_optimizing::experiment::Experiment;
use stick_solo::plan::cross_entropy_optimizing::fcn::*;
use stick_solo::plan::cross_entropy_optimizing::utils::{
    control, decode, encode, random_sample_solve, GoalQsCouple,
};

fn main() {
    let args = env::args();
    if args.len() != 3 {
        panic!("Bad cmd line parameters.");
    }
    // Load from file
    let args = args.collect::<Vec<String>>();
    let left_holding_exp: Experiment =
        serde_json::from_reader(BufReader::new(File::open(&args[1]).unwrap())).unwrap();
    let right_holding_exp: Experiment =
        serde_json::from_reader(BufReader::new(File::open(&args[2]).unwrap())).unwrap();
    println!("{:?}", left_holding_exp);
    println!("{:?}", right_holding_exp);

    // Visualize
    let world = left_holding_exp.world.clone();
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor {
            width: 2000,
            height: 1000,
            ..Default::default()
        })
        .add_plugins(BasePlugins)
        .add_plugin(CameraPlugin)
        .add_resource(FCNs {
            left_holding: left_holding_exp.fcn,
            right_holding: right_holding_exp.fcn,
        })
        .add_resource(GoalQsCouple(Array::zeros(0), Array::zeros(0)))
        .add_plugin(OneHoldingSwitchableNRCouplePlugin::new(
            OneHoldingSwitchableNRCouple::new(
                &world.holding_side,
                Vec2::new(0.0, -0.1),
                &world.holding_ls,
                &world.sample_holding_qs(),
                &world.holding_q_clamps(),
                &world.non_holding_ls,
                &world.sample_non_holding_qs(),
                &world.non_holding_q_clamps(),
                0.06,
            ),
        ))
        .add_plugin(GoalCouplePlugin::new(GoalCouple(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.5, 0.0),
        )))
        .add_plugin(PathPlugin::new(Path::default()))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_startup_system(initial_set_goal_qs_couple_system.system())
        .add_system(control_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

struct FCNs {
    left_holding: FCN,
    right_holding: FCN,
}

fn set_goal_qs_couple(
    agent: &OneHoldingSwitchableNRCouple,
    goal_qs_couple: &mut GoalQsCouple,
    goal_couple: &mut GoalCouple,
    fcns: &FCNs,
    path: &Path,
) {
    let non_holding_goal = path.0.front().unwrap().clone();
    let (_, holding_origin, _, _, _, holding_side) = agent.holding().get_current_state();
    // Network pipeline
    let (input, scale) = encode(&agent, &non_holding_goal);
    let forward_pass = match holding_side {
        Side::Left => fcns.left_holding.at(&input),
        Side::Right => fcns.right_holding.at(&input),
    };
    let holding_goal = decode(&forward_pass, scale, holding_origin.clone());
    // Setting GoalCouple and GoalQsCouple
    *goal_couple = GoalCouple(holding_goal, non_holding_goal);
    random_sample_solve(agent, goal_couple, goal_qs_couple);
}

fn initial_set_goal_qs_couple_system(
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut goal_qs_couple: ResMut<GoalQsCouple>,
    mut goal_couple: ResMut<GoalCouple>,
    path: ResMut<Path>,
    fcns: Res<FCNs>,
) {
    set_goal_qs_couple(&agent, &mut goal_qs_couple, &mut goal_couple, &fcns, &path);
}

fn control_system(
    mut agent: ResMut<OneHoldingSwitchableNRCouple>,
    pause: Res<Pause>,
    mut ticks: ResMut<Ticks>,
    mut goal_qs_couple: ResMut<GoalQsCouple>,
    mut goal_couple: ResMut<GoalCouple>,
    mut path: ResMut<Path>,
    fcns: Res<FCNs>,
) {
    if pause.0 {
        return;
    }
    // No more goals => pause everything
    if path.0.is_empty() {
        return;
    }
    let (_, holding_origin, _, _, _, holding_side) = agent.holding().get_current_state();
    let non_holding_goal = path.0.front().unwrap().clone();
    let have_to_match = match holding_side {
        Side::Left => non_holding_goal[0] - holding_origin[0] < -SwitchableNR::GOAL_REACHED_SLACK,
        Side::Right => non_holding_goal[0] - holding_origin[0] > SwitchableNR::GOAL_REACHED_SLACK,
    };
    if have_to_match {
        // Add current holding origin as goal
        path.0.push_front(holding_origin.clone());
        // Set GoalQsCouple
        set_goal_qs_couple(&agent, &mut goal_qs_couple, &mut goal_couple, &fcns, &path);
        return;
    }
    let non_holding_last = agent.non_holding().get_last_vertex();
    if (non_holding_goal - non_holding_last).length() < SwitchableNR::GOAL_REACHED_SLACK {
        // Switch pivot
        agent.switch_hold();
        // Remove one vertex from path
        path.0.pop_front();
        // Reset ticks
        ticks.0 = 0;
        // Set GoalQsCouple if more goals available
        if path.0.len() > 0 {
            set_goal_qs_couple(&agent, &mut goal_qs_couple, &mut goal_couple, &fcns, &path);
        }
        return;
    }
    control(&mut agent, &goal_qs_couple, &goal_couple, ticks.0);
    ticks.0 += 1;
}
