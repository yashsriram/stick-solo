extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use rand::prelude::*;
use rayon::prelude::*;
use stick_solo::act::switchable_nr::{PivotingSide, SwitchableNR};
use stick_solo::game::{
    base_plugins::BasePlugins,
    camera_plugin::CameraPlugin,
    goal_plugin::{Goal, GoalPlugin},
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
    switchable_nr_plugin::SwitchableNRPlugin,
};
use stick_solo::plan::*;

fn main() {
    let inf = f32::INFINITY;
    let pi = std::f32::consts::PI;
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor {
            width: 2000,
            height: 1000,
            ..Default::default()
        })
        .add_plugins(BasePlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(SwitchableNRPlugin::new(SwitchableNR::new(
            Vec2::new(0.0, -0.1),
            &[0.2, 0.2, 0.2, 0.2],
            &[1.0, 0.5, 0.0, 0.5],
            &[
                (-inf, inf),
                (0.0, pi * 0.5),
                (-pi * 0.5, pi),
                (0.0, pi * 0.5),
            ],
            PivotingSide::Left,
            0.01,
        )))
        .add_plugin(GoalPlugin::new(Goal(Vec2::new(0.1, -0.5))))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(genetic_solve_from_current_state.system())
        // .add_system(interpolate.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn genetic_solve_no_prior(mut agent: ResMut<SwitchableNR>, goal: Res<Goal>, pause: Res<Pause>) {
    if pause.0 {
        return;
    }
    let inf = f32::INFINITY;
    let pi = std::f32::consts::PI;
    let (n, origin, ls, _, q_clamps, _) = agent.get_current_state();
    let mut losses = (0..10000usize)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let new_qs = q_clamps
                .iter()
                .map(|clamp| {
                    if clamp == &(-inf, inf) {
                        rng.gen_range(-pi, pi)
                    } else {
                        rng.gen_range(clamp.0, clamp.1)
                    }
                })
                .collect::<Array1<f32>>();
            let (end, com) = get_end_verticex_and_com(origin, ls, &new_qs);
            (
                10.0 * (end - goal.0).length()
                    + com[1]
                    + (com[0] - (end[0] + goal.0[0]) / 2.0).powf(2.0),
                new_qs,
            )
        })
        .collect::<Vec<(f32, Array1<f32>)>>();
    losses.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("{:?}", losses[0].0);
    agent.update_qs(losses[0].1.clone());
}

fn genetic_solve_from_current_state(
    mut agent: ResMut<SwitchableNR>,
    goal: Res<Goal>,
    pause: Res<Pause>,
) {
    if pause.0 {
        return;
    }
    let (n, origin, ls, qs, q_clamps, _) = agent.get_current_state();
    let q_mutation = 2.0f32;
    let mut losses = (0..10000usize)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let mutation = Array::random(qs.len(), Uniform::new(-q_mutation, q_mutation));
            let mut new_qs = qs + &mutation;
            for i in 0..n {
                let (min, max) = q_clamps[i];
                if new_qs[i] < min {
                    new_qs[i] = min
                } else if new_qs[i] > max {
                    new_qs[i] = max
                }
            }
            let (end, com) = get_end_verticex_and_com(origin, ls, &new_qs);
            (
                10.0 * (end - goal.0).length()
                    + com[1]
                    + (com[0] - (end[0] + goal.0[0]) / 2.0).powf(2.0),
                new_qs,
            )
        })
        .collect::<Vec<(f32, Array1<f32>)>>();
    losses.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("{:?}", losses[0].0);
    agent.update_qs(losses[0].1.clone());
}

// fn interpolate(
//     mut agent: ResMut<SwitchableNR>,
//     pause: Res<Pause>,
//     goal: Res<Goal>,
//     mut ticks: ResMut<Ticks>,
// ) {
//     // Pause => pause everything
//     if pause.0 {
//         return;
//     }
//     let (_, origin, ls, qs, q_clamps) = agent.get_current_state();
//     let given_goal = goal.0;

//     let (take_end_to_given_goal, push_com_x_from_its_goal, _) = ik(
//         origin,
//         ls,
//         qs,
//         &given_goal,
//         EndControl::JacobianTranspose,
//         COMXGoalType::PivotGoalMidpoint,
//     );

//     agent.update(take_end_to_given_goal + -0.2 * push_com_x_from_its_goal);

//     ticks.0 += 1;
// }
