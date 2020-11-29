use crate::act::switchable_nr::SwitchableNR;
use bevy::prelude::*;

pub struct SwitchableNRPlugin {
    agent: SwitchableNR,
}

impl SwitchableNRPlugin {
    pub fn new(agent: SwitchableNR) -> SwitchableNRPlugin {
        SwitchableNRPlugin { agent }
    }
}

impl Plugin for SwitchableNRPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.agent.clone())
            .add_startup_system(init.system())
            .add_system(flush_transforms.system());
    }
}

struct Edge(usize);
struct Vertex(usize);
struct CenterOfMass;

fn init(
    mut commands: Commands,
    agent: Res<SwitchableNR>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
    // Center of mass
    commands
        .spawn(SpriteComponents {
            sprite: Sprite {
                size: Vec2::new(0.04, 0.04),
                resize_mode: SpriteResizeMode::Manual,
            },
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .with(CenterOfMass);
}

fn flush_transforms(
    agent: Res<SwitchableNR>,
    mut edge_query: Query<(&Edge, &mut Sprite, &mut Transform)>,
    mut vertex_query: Query<(&Vertex, &mut Transform)>,
    mut com_query: Query<(&CenterOfMass, &mut Transform)>,
) {
    let transforms = agent.pose_to_transforms();
    let (_, _, ls, _) = agent.get_current_state();
    for (edge, mut sprite, mut transform) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        sprite.size = Vec2::new(ls[edge.0], agent.thickness());
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.rotation = Quat::from_rotation_z(angle);
    }
    let vertex_positions = agent.get_all_vertices();
    for (idx, mut transform) in vertex_query.iter_mut() {
        transform.translation[0] = vertex_positions[idx.0][0];
        transform.translation[1] = vertex_positions[idx.0][1];
    }
    let com = agent.get_center_of_mass();
    for (_, mut transform) in com_query.iter_mut() {
        transform.translation[0] = com[0];
        transform.translation[1] = com[1];
    }
}
