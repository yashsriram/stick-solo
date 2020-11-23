extern crate stick_solo;
use bevy::prelude::*;
use stick_solo::act::{Goal, NRAgent};
use stick_solo::vis::*;

fn main() {
    let pi = std::f32::consts::PI;
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(base_plugins::BasePlugins)
        .add_plugin(camera_plugin::CameraPlugin)
        .add_plugin(nr_agent_plugin::NRAgentPlugin::new(
            NRAgent::new(
                Vec2::new(0.0, 0.0),
                &[0.2, 0.2],
                &[pi / 2.0, -pi / 3.0],
                &[(pi * 1.0 / 3.0, pi * 2.0 / 3.0), (-pi / 2.0, 0.0)],
                0.01,
            ),
            // NRAgent::new(
            //     Vec2::new(0.0, 0.0),
            //     &[0.2, 0.2],
            //     &[-pi / 2.0, 0.0],
            //     &[(-pi * 5.0 / 12.0, pi * 1.0 / 12.0), (-pi, 0.0)],
            //     0.01,
            // ),
            Goal(Vec2::new(0.1, -0.1)),
        ))
        .add_plugin(status_bar_plugin::StatusBarPlugin)
        .add_system(control.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn control(goal: Res<Goal>, mut agent: ResMut<NRAgent>) {
    let (_, origin, ls, qs) = agent.get_current_state();
    let delta_qs = stick_solo::plan::jacobian_transpose(origin, ls, qs, &goal.0);
    agent.update(delta_qs);
}
