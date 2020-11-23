use crate::act::NR;
use bevy::prelude::*;

pub struct NRPlugin {
    agent: NR,
}

impl NRPlugin {
    pub fn new(agent: NR) -> NRPlugin {
        NRPlugin { agent }
    }
}

impl Plugin for NRPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.agent.clone())
            .add_startup_system(init.system())
            .add_system(flush_transforms.system());
    }
}

struct Edge(usize);
struct Vertex(usize);

fn init(mut commands: Commands, agent: Res<NR>, mut materials: ResMut<Assets<ColorMaterial>>) {
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
}

fn flush_transforms(
    agent_state: Res<NR>,
    mut edge_query: Query<(&Edge, &mut Transform)>,
    mut vertex_query: Query<(&Vertex, &mut Transform)>,
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
}
