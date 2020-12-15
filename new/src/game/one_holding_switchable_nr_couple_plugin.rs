use crate::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use crate::act::switchable_nr::*;
use bevy::prelude::*;

pub struct OneHoldingSwitchableNRCouplePlugin {
    agent: OneHoldingSwitchableNRCouple,
}

impl OneHoldingSwitchableNRCouplePlugin {
    pub fn new(agent: OneHoldingSwitchableNRCouple) -> OneHoldingSwitchableNRCouplePlugin {
        OneHoldingSwitchableNRCouplePlugin { agent }
    }
}

impl Plugin for OneHoldingSwitchableNRCouplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.agent.clone())
            .add_startup_system(init_vis.system())
            .add_system(flush_transforms_com.system())
            .add_system(flush_transforms_original_holding.system())
            .add_system(flush_transforms_original_non_holding.system());
    }
}

struct Edge(usize);
struct Vertex(usize);
struct PartCenterOfMass;
struct TotalCenterOfMass;
#[derive(Default)]
struct OriginalHolding;
#[derive(Default)]
struct OriginalNonHolding;

fn init_vis(
    mut commands: Commands,
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    fn init<T: Default + Sync + Component>(
        agent: &SwitchableNR,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
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
                    material: materials.add(color.into()),
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
                    material: materials.add(color.into()),
                    ..Default::default()
                })
                .with(Vertex(i))
                .with(T::default());
        }
        // Center of mass
        commands
            .spawn(SpriteComponents {
                sprite: Sprite {
                    size: Vec2::new(thickness * 2.0, thickness * 2.0),
                    resize_mode: SpriteResizeMode::Manual,
                },
                material: materials.add(color.into()),
                ..Default::default()
            })
            .with(PartCenterOfMass)
            .with(T::default());
    }
    let (holding, non_holding) = (agent.holding(), agent.non_holding());
    let (_, holding_origin, _, _, _, _) = holding.get_current_state();
    // Original holding
    init::<OriginalHolding>(
        holding,
        &mut commands,
        &mut materials,
        Color::rgb(0.0, 1.0, 0.0),
    );
    // Original non-holding
    init::<OriginalNonHolding>(
        non_holding,
        &mut commands,
        &mut materials,
        Color::rgb(0.0, 0.0, 1.0),
    );
    // Origin
    commands.spawn(SpriteComponents {
        sprite: Sprite {
            size: Vec2::new(
                4.0 * SwitchableNR::GOAL_REACHED_SLACK,
                4.0 * SwitchableNR::GOAL_REACHED_SLACK,
            ),
            resize_mode: SpriteResizeMode::Manual,
        },
        transform: Transform::from_translation(Vec3::new(
            holding_origin[0],
            holding_origin[1],
            0.0,
        )),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        ..Default::default()
    });
    // COM
    commands
        .spawn(SpriteComponents {
            sprite: Sprite {
                size: Vec2::new(holding.thickness() * 2.0, holding.thickness() * 2.0),
                resize_mode: SpriteResizeMode::Manual,
            },
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .with(TotalCenterOfMass);
}

fn flush_transforms_original_holding(
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut edge_query: Query<(&Edge, &mut Sprite, &mut Transform, &OriginalHolding)>,
    mut vertex_query: Query<(&Vertex, &mut Transform, &OriginalHolding)>,
    mut com_query: Query<(&PartCenterOfMass, &mut Transform, &OriginalHolding)>,
) {
    let switchable_nr = agent.original_holding();
    let transforms = switchable_nr.pose_to_transforms();
    let (_, _, ls, _, _, _) = switchable_nr.get_current_state();
    for (edge, mut sprite, mut transform, _) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        sprite.size = Vec2::new(ls[edge.0], switchable_nr.thickness());
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.rotation = Quat::from_rotation_z(angle);
    }
    let vertex_positions = switchable_nr.get_all_vertices();
    for (idx, mut transform, _) in vertex_query.iter_mut() {
        transform.translation[0] = vertex_positions[idx.0][0];
        transform.translation[1] = vertex_positions[idx.0][1];
    }
    let com = switchable_nr.get_center_of_mass();
    for (_, mut transform, _) in com_query.iter_mut() {
        transform.translation[0] = com[0];
        transform.translation[1] = com[1];
    }
}

fn flush_transforms_original_non_holding(
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut edge_query: Query<(&Edge, &mut Sprite, &mut Transform, &OriginalNonHolding)>,
    mut vertex_query: Query<(&Vertex, &mut Transform, &OriginalNonHolding)>,
    mut com_query: Query<(&PartCenterOfMass, &mut Transform, &OriginalNonHolding)>,
) {
    let switchable_nr = agent.original_non_holding();
    let transforms = switchable_nr.pose_to_transforms();
    let (_, _, ls, _, _, _) = switchable_nr.get_current_state();
    for (edge, mut sprite, mut transform, _) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        sprite.size = Vec2::new(ls[edge.0], switchable_nr.thickness());
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.rotation = Quat::from_rotation_z(angle);
    }
    let vertex_positions = switchable_nr.get_all_vertices();
    for (idx, mut transform, _) in vertex_query.iter_mut() {
        transform.translation[0] = vertex_positions[idx.0][0];
        transform.translation[1] = vertex_positions[idx.0][1];
    }
    let com = switchable_nr.get_center_of_mass();
    for (_, mut transform, _) in com_query.iter_mut() {
        transform.translation[0] = com[0];
        transform.translation[1] = com[1];
    }
}

fn flush_transforms_com(
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut com_query: Query<(&TotalCenterOfMass, &mut Transform)>,
) {
    let com = agent.get_center_of_mass();
    for (_, mut transform) in com_query.iter_mut() {
        transform.translation[0] = com[0];
        transform.translation[1] = com[1];
    }
}
