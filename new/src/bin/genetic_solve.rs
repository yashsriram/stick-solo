extern crate stick_solo;
use bevy::prelude::*;
use ndarray::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use rand::prelude::*;
use rayon::prelude::*;
use stick_solo::act::NR;
use stick_solo::game::{
    base_plugins::BasePlugins,
    camera_plugin::CameraPlugin,
    goal_plugin::{Goal, GoalPlugin},
    nr_plugin::NRPlugin,
    pause_plugin::Pause,
    pause_plugin::PausePlugin,
    status_bar_plugin::{StatusBarPlugin, Ticks},
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
        .add_plugin(NRPlugin::new(NR::new(
            Vec2::new(0.0, -0.1),
            &[0.2, 0.2, 0.2, 0.2],
            &[1.0, 1.0, 2.0, 0.0],
            &[
                (-pi, pi / 2.0),
                (0.0, pi * 0.5),
                (-pi * 0.5, pi),
                (0.0, pi * 0.5),
            ],
            0.01,
        )))
        .add_plugin(GoalPlugin::new(Goal(Vec2::new(0.1, -0.5))))
        .add_plugin(StatusBarPlugin)
        .add_plugin(PausePlugin)
        .add_system(genetic_solve_multi_gen.system())
        // .add_system(interpolate.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn genetic_solve_single_gen(mut agent: ResMut<NR>, goal: Res<Goal>) {
    let (n, origin, ls, _, q_clamps) = agent.get_current_state();
    let mut tries = (0..10000usize)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let new_qs = q_clamps
                .iter()
                .map(|clamp| rng.gen_range(clamp.0, clamp.1))
                .collect::<Array1<f32>>();
            let mut e1 = *origin;
            let mut cumulative_rotation = 0f32;
            for i in 0..n {
                cumulative_rotation += new_qs[i];
                let e2 =
                    e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * ls[i];
                e1 = e2;
            }
            ((e1 - goal.0).length(), new_qs)
        })
        .collect::<Vec<(f32, Array1<f32>)>>();
    tries.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("{:?}", tries[0].0);
    agent.update_qs(tries[0].1.clone());
}

fn genetic_solve_multi_gen(mut agent: ResMut<NR>, goal: Res<Goal>) {
    let (n, origin, ls, qs, q_clamps) = agent.get_current_state();
    let q_mutation = 2.0f32;
    let mut tries = (0..10000usize)
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
            let mut e1 = *origin;
            let mut cumulative_rotation = 0f32;
            for i in 0..n {
                cumulative_rotation += new_qs[i];
                let e2 =
                    e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * ls[i];
                e1 = e2;
            }
            ((e1 - goal.0).length(), new_qs)
        })
        .collect::<Vec<(f32, Array1<f32>)>>();
    tries.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("{:?}", tries[0].0);
    agent.update_qs(tries[0].1.clone());
}

// fn interpolate(
//     mut agent: ResMut<NR>,
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
