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
    fn build(&self, app: &mut App) {
        app.insert_resource(self.agent.clone())
            .add_startup_system(init_vis)
            .add_system(flush_transforms);
    }
}

#[derive(Component)]
struct Edge(usize);
#[derive(Component)]
struct Vertex(usize);
#[derive(Component)]
struct CenterOfMass;

fn init_vis(
    mut commands: Commands,
    agent: Res<SwitchableNR>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let thickness = agent.thickness();
    let (n, _, ls, _, _, _) = agent.get_current_state();
    // Edges
    for i in 0..n {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(ls[i], thickness)),
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Edge(i));
    }
    // Vertices
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(thickness * 2.0, thickness * 2.0)),
                color: Color::rgb(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Vertex(0));
    for i in 0..n {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(thickness * 2.0, thickness * 2.0)),
                    color: Color::rgb(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Vertex(i + 1));
    }
    // Center of mass
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.04, 0.04)),
                color: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CenterOfMass);
}

fn flush_transforms(
    agent: Res<SwitchableNR>,
    mut edge_query: Query<(&Edge, &mut Sprite, &mut Transform)>,
    mut vertex_query: Query<(&Vertex, &mut Transform)>,
    mut com_query: Query<(&CenterOfMass, &mut Transform)>,
) {
    let transforms = agent.pose_to_transforms();
    let (_, _, ls, _, _, _) = agent.get_current_state();
    for (edge, mut sprite, mut transform) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        sprite.custom_size = Some(Vec2::new(ls[edge.0], agent.thickness()));
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
