use bevy::prelude::*;
use nalgebra::DVector;

pub struct NRAgentPlugin;

impl Plugin for NRAgentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(NRAgentStateRes::new(
            Vec2::new(0.0, 0.0),
            vec![0.2, 0.2, 0.2, 0.2],
            vec![0.5, -0.1, -0.6, -0.1],
            0.01,
        ))
        .add_resource(GoalRes(Vec2::new(0.5, 0.0)))
        .add_startup_system(init.system())
        .add_system(interactive_goal.system())
        .add_system(control.system())
        .add_system(flush_transforms.system());
    }
}

struct GoalRes(Vec2);
struct GoalMarkerCom;

struct NRAgentStateRes {
    n: usize,
    origin: Vec2,
    l: DVector<f32>,
    q: DVector<f32>,
    thickness: f32,
}

impl NRAgentStateRes {
    fn new(origin: Vec2, l: Vec<f32>, q: Vec<f32>, thickness: f32) -> Self {
        assert_eq!(
            l.len(),
            q.len(),
            "Unequal number of lengths and joint angles arguments."
        );
        assert!(thickness > 0.0, "Non-positive thickness argument");
        NRAgentStateRes {
            n: l.len(),
            origin: origin,
            l: DVector::from_iterator(l.len(), l.into_iter()),
            q: DVector::from_iterator(q.len(), q.into_iter()),
            thickness: thickness,
        }
    }

    fn pose_to_transforms(&self) -> Vec<(Vec2, f32)> {
        let mut transforms = vec![];
        let mut e1 = self.origin;
        let mut cumulative_rotation = 0f32;
        for i in 0..self.n {
            cumulative_rotation += self.q[i];
            let e2 =
                e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * self.l[i];
            let midpoint = (e1 + e2) / 2.0;
            transforms.push((midpoint, cumulative_rotation));
            e1 = e2;
        }
        transforms
    }

    fn get_vertices(&self) -> Vec<Vec2> {
        let mut vertices = vec![self.origin];
        let mut e1 = self.origin;
        let mut cumulative_rotation = 0f32;
        for i in 0..self.n {
            cumulative_rotation += self.q[i];
            let e2 =
                e1 + Vec2::new(cumulative_rotation.cos(), cumulative_rotation.sin()) * self.l[i];
            vertices.push(e2);
            e1 = e2;
        }
        vertices
    }
}

struct Edge(usize);
struct Vertex(usize);

fn init(
    mut commands: Commands,
    agent_state: Res<NRAgentStateRes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Edges
    for i in 0..agent_state.n {
        commands
            .spawn(SpriteComponents {
                sprite: Sprite {
                    size: Vec2::new(agent_state.l[i], agent_state.thickness),
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
                size: Vec2::new(agent_state.thickness * 2.0, agent_state.thickness * 2.0),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(Vertex(0));
    for i in 0..agent_state.n {
        commands
            .spawn(SpriteComponents {
                sprite: Sprite {
                    size: Vec2::new(agent_state.thickness * 2.0, agent_state.thickness * 2.0),
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
                size: Vec2::new(agent_state.thickness * 4.0, agent_state.thickness * 4.0),
                resize_mode: SpriteResizeMode::Manual,
            },
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..Default::default()
        })
        .with(GoalMarkerCom);
}

fn interactive_goal(keyboard_input: Res<Input<KeyCode>>, mut goal: ResMut<GoalRes>) {
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

fn control(goal: Res<GoalRes>, mut agent_state: ResMut<NRAgentStateRes>) {
    let mut delta_q = super::ik::jt_step(&agent_state.get_vertices(), &agent_state.q, &goal.0);
    delta_q *= 0.1;
    agent_state.q += delta_q;
}

fn flush_transforms(
    goal: Res<GoalRes>,
    agent_state: Res<NRAgentStateRes>,
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
    let vertex_positions = agent_state.get_vertices();
    for (vertex, mut transform) in vertex_query.iter_mut() {
        transform.translation[0] = vertex_positions[vertex.0][0];
        transform.translation[1] = vertex_positions[vertex.0][1];
    }
    for (_, mut transform) in goal_query.iter_mut() {
        transform.translation[0] = goal.0[0];
        transform.translation[1] = goal.0[1];
    }
}
