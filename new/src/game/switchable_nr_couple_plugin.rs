use crate::act::switchable_nr::*;
use crate::act::switchable_nr_couple::SwitchableNRCouple;
use bevy::prelude::*;

pub struct SwitchableNRCouplePlugin {
    agent: SwitchableNRCouple,
}

impl SwitchableNRCouplePlugin {
    pub fn new(agent: SwitchableNRCouple) -> SwitchableNRCouplePlugin {
        SwitchableNRCouplePlugin { agent }
    }
}

impl Plugin for SwitchableNRCouplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.agent.clone())
            .add_startup_system(init.system())
            .add_system(flush_transforms_left.system())
            .add_system(flush_transforms_right.system());
    }
}

struct Edge(usize);
struct Vertex(usize);
struct CenterOfMass;
#[derive(Default)]
struct Left;
#[derive(Default)]
struct Right;

fn init(
    mut commands: Commands,
    agent: Res<SwitchableNRCouple>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    fn init<T: Default + Sync + Component>(
        agent: &SwitchableNR,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let thickness = agent.thickness();
        let (n, _, ls, _, _, _) = agent.get_current_state();
        // Edges
        for i in 0..n {
            commands
                .spawn(SpriteComponents {
                    sprite: Sprite {
                        size: Vec2::new(ls[i], thickness),
                        resize_mode: SpriteResizeMode::Manual,
                    },
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    ..Default::default()
                })
                .with(Edge(i))
                .with(T::default());
        }
        // Vertices
        for i in 0..(n + 1) {
            commands
                .spawn(SpriteComponents {
                    sprite: Sprite {
                        size: Vec2::new(thickness * 2.0, thickness * 2.0),
                        resize_mode: SpriteResizeMode::Manual,
                    },
                    material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                    ..Default::default()
                })
                .with(Vertex(i))
                .with(T::default());
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
            .with(CenterOfMass)
            .with(T::default());
    }
    let (left, right) = (agent.left(), agent.right());
    init::<Left>(left, &mut commands, &mut materials);
    init::<Right>(right, &mut commands, &mut materials);
}

fn flush_transforms_left(
    agent: Res<SwitchableNRCouple>,
    mut edge_query: Query<(&Edge, &mut Sprite, &mut Transform, &Left)>,
    mut vertex_query: Query<(&Vertex, &mut Transform, &Left)>,
    mut com_query: Query<(&CenterOfMass, &mut Transform, &Left)>,
) {
    let (left, _) = (agent.left(), agent.right());
    let transforms = left.pose_to_transforms();
    let (_, _, ls, _, _, _) = left.get_current_state();
    for (edge, mut sprite, mut transform, _) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        sprite.size = Vec2::new(ls[edge.0], left.thickness());
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.rotation = Quat::from_rotation_z(angle);
    }
    let vertex_positions = left.get_all_vertices();
    for (idx, mut transform, _) in vertex_query.iter_mut() {
        transform.translation[0] = vertex_positions[idx.0][0];
        transform.translation[1] = vertex_positions[idx.0][1];
    }
    let com = left.get_center_of_mass();
    for (_, mut transform, _) in com_query.iter_mut() {
        transform.translation[0] = com[0];
        transform.translation[1] = com[1];
    }
}

fn flush_transforms_right(
    agent: Res<SwitchableNRCouple>,
    mut edge_query: Query<(&Edge, &mut Sprite, &mut Transform, &Right)>,
    mut vertex_query: Query<(&Vertex, &mut Transform, &Right)>,
    mut com_query: Query<(&CenterOfMass, &mut Transform, &Right)>,
) {
    let (_, right) = (agent.left(), agent.right());
    let transforms = right.pose_to_transforms();
    let (_, _, ls, _, _, _) = right.get_current_state();
    for (edge, mut sprite, mut transform, _) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        sprite.size = Vec2::new(ls[edge.0], right.thickness());
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.rotation = Quat::from_rotation_z(angle);
    }
    let vertex_positions = right.get_all_vertices();
    for (idx, mut transform, _) in vertex_query.iter_mut() {
        transform.translation[0] = vertex_positions[idx.0][0];
        transform.translation[1] = vertex_positions[idx.0][1];
    }
    let com = right.get_center_of_mass();
    for (_, mut transform, _) in com_query.iter_mut() {
        transform.translation[0] = com[0];
        transform.translation[1] = com[1];
    }
}
