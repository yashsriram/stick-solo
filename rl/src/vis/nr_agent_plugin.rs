use crate::act::{Goal, NRAgent};
use bevy::prelude::*;

pub struct NRAgentPlugin {
    agent: NRAgent,
    goal: Vec2,
}

impl NRAgentPlugin {
    pub fn new(agent: NRAgent, goal: Vec2) -> NRAgentPlugin {
        NRAgentPlugin { agent, goal }
    }
}

impl Plugin for NRAgentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.agent.clone())
            .add_resource(Goal(self.goal))
            .add_startup_system(init.system())
            .add_system(interactive_goal.system())
            .add_system(flush_transforms.system());
    }
}

struct GoalMarkerCom;

struct Edge(usize);
struct Vertex(usize);

fn init(mut commands: Commands, agent: Res<NRAgent>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let thickness = agent.thickness();
    let (n, _, ls, _) = agent.get_current_state();
    // Edges
    for i in 0..n {
        commands
            .spawn(SpriteComponents {
                sprite: Sprite {
                    size: Vec2::new(ls[i], thickness),
                    resize_mode: SpriteResizeMode::Manual,
                },
                material: materials
                    .add(Color::rgb(rand::random(), rand::random(), rand::random()).into()),
                ..Default::default()
            })
            .with(Edge(i));
    }
    // Vertices
    commands
        .spawn(SpriteComponents {
            sprite: Sprite {
                size: Vec2::new(thickness * 2.0, thickness * 2.0),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(Vertex(0));
    for i in 0..n {
        commands
            .spawn(SpriteComponents {
                sprite: Sprite {
                    size: Vec2::new(thickness * 2.0, thickness * 2.0),
                    resize_mode: SpriteResizeMode::Manual,
                },
                ..Default::default()
            })
            .with(Vertex(i + 1));
    }
    // Goal
    commands
        .spawn(SpriteComponents {
            sprite: Sprite {
                size: Vec2::new(thickness * 4.0, thickness * 4.0),
                resize_mode: SpriteResizeMode::Manual,
            },
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..Default::default()
        })
        .with(GoalMarkerCom);
}

fn flush_transforms(
    goal: Res<Goal>,
    agent_state: Res<NRAgent>,
    mut edge_query: Query<(&Edge, &mut Transform)>,
    mut vertex_query: Query<(&Vertex, &mut Transform)>,
    mut goal_query: Query<(&GoalMarkerCom, &mut Transform)>,
) {
    let transforms = agent_state.pose_to_transforms();
    for (edge, mut transform) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.rotation = Quat::from_rotation_z(angle);
    }
    let vertex_positions = agent_state.get_all_vertices();
    for (idx, mut transform) in vertex_query.iter_mut() {
        transform.translation[0] = vertex_positions[idx.0][0];
        transform.translation[1] = vertex_positions[idx.0][1];
    }
    for (_, mut transform) in goal_query.iter_mut() {
        transform.translation[0] = goal.0[0];
        transform.translation[1] = goal.0[1];
    }
}

fn interactive_goal(keyboard_input: Res<Input<KeyCode>>, mut goal: ResMut<Goal>) {
    if keyboard_input.pressed(KeyCode::W) {
        goal.0[1] += 0.01;
    } else if keyboard_input.pressed(KeyCode::S) {
        goal.0[1] -= 0.01;
    } else if keyboard_input.pressed(KeyCode::A) {
        goal.0[0] -= 0.01;
    } else if keyboard_input.pressed(KeyCode::D) {
        goal.0[0] += 0.01;
    }
}
