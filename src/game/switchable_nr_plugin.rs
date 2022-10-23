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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let thickness = agent.thickness();
    let (n, _, ls, _, _, _) = agent.get_current_state();
    // Edges
    for i in 0..n {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)))),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::default().with_scale(Vec3::new(ls[i], thickness, 1.0)),
                ..default()
            })
            .insert(Edge(i));
    }
    // Vertices
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                thickness * 2.0,
                thickness * 2.0,
            )))),
            material: materials.add(Color::BLUE.into()),
            ..default()
        })
        .insert(Vertex(0));
    for i in 0..n {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                    thickness * 2.0,
                    thickness * 2.0,
                )))),
                material: materials.add(Color::BLUE.into()),
                ..default()
            })
            .insert(Vertex(i + 1));
    }
    // Center of mass
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.04, 0.04)))),
            material: materials.add(Color::RED.into()),
            ..default()
        })
        .insert(CenterOfMass);
}

fn flush_transforms(
    agent: Res<SwitchableNR>,
    mut transforms_query: Query<&mut Transform>,
    mut edge_query: Query<(Entity, &Edge)>,
    mut vertex_query: Query<(Entity, &Vertex)>,
    mut com_query: Query<(Entity, &CenterOfMass)>,
) {
    let transforms = agent.pose_to_transforms();
    let (_, _, ls, _, _, _) = agent.get_current_state();
    for (entity, edge) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        let mut transform = transforms_query.get_mut(entity).unwrap();
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.scale = Vec3::new(ls[edge.0], agent.thickness(), 1.0);
        transform.rotation = Quat::from_rotation_z(angle);
    }
    let vertex_positions = agent.get_all_vertices();
    for (entity, idx) in vertex_query.iter_mut() {
        let mut transform = transforms_query.get_mut(entity).unwrap();
        transform.translation[0] = vertex_positions[idx.0][0];
        transform.translation[1] = vertex_positions[idx.0][1];
    }
    let com = agent.get_center_of_mass();
    for (entity, _) in com_query.iter_mut() {
        let mut transform = transforms_query.get_mut(entity).unwrap();
        transform.translation[0] = com[0];
        transform.translation[1] = com[1];
    }
}
