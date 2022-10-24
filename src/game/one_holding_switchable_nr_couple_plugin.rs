use crate::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use crate::act::switchable_nr::*;
use crate::AxesHuggingUnitSquare;
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
    fn build(&self, app: &mut App) {
        app.insert_resource(self.agent.clone())
            .add_startup_system(init_vis)
            .add_system(flush_transforms_com)
            .add_system(flush_transforms_body)
            .add_system(flush_transforms_original_holding)
            .add_system(flush_transforms_original_non_holding);
    }
}

#[derive(Component)]
struct Edge(usize);
#[derive(Component)]
struct Vertex(usize);
#[derive(Component)]
struct PartCenterOfMass;
#[derive(Component)]
struct TotalCenterOfMass;
#[derive(Component, Default)]
struct OriginalHolding;
#[derive(Component, Default)]
struct OriginalNonHolding;
#[derive(Component)]
struct Body(f32);

fn init_vis(
    mut commands: Commands,
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    fn init<T: Default + Sync + Component>(
        agent: &SwitchableNR,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        _color: Color,
        asset_server: &AssetServer,
    ) {
        let (n, _, ls, _, _, _) = agent.get_current_state();
        // Edges
        for i in 0..n {
            let texture_handle = asset_server.load("sprites/bone.png");
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(AxesHuggingUnitSquare)),
                    material: materials.add(texture_handle.into()),
                    transform: Transform::default().with_scale(Vec3::new(ls[i], 0.05, 1.0)),
                    ..default()
                })
                .insert(Edge(i))
                .insert(T::default());
        }
        // Vertices
        // for i in 0..(n + 1) {
        //     commands
        //         .spawn(SpriteBundle {
        //             sprite: Sprite {
        //                 custom_size: Vec2::new(thickness * 2.0, thickness * 2.0),
        //             },
        //             ..Default::default()
        //         })
        //         .with(Vertex(i))
        //         .with(T::default());
        // }
        // Center of mass
        // commands
        //     .spawn(SpriteBundle {
        //         sprite: Sprite {
        //             custom_size: Vec2::new(thickness * 2.0, thickness * 2.0),
        //         },
        //         ..Default::default()
        //     })
        //     .with(PartCenterOfMass)
        //     .with(T::default());
    }
    let (holding, non_holding) = (agent.holding(), agent.non_holding());
    let (_, holding_origin, _, _, _, _) = holding.get_current_state();
    // Original holding
    init::<OriginalHolding>(
        holding,
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::rgb(0.0, 1.0, 0.0),
        &asset_server,
    );
    // Original non-holding
    init::<OriginalNonHolding>(
        non_holding,
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::rgb(0.0, 0.0, 1.0),
        &asset_server,
    );
    // Origin
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)))),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::default()
            .with_scale(Vec3::new(
                4.0 * SwitchableNR::GOAL_REACHED_SLACK,
                4.0 * SwitchableNR::GOAL_REACHED_SLACK,
                1.0,
            ))
            .with_translation(Vec3::new(holding_origin[0], holding_origin[1], 0.0)),
        ..default()
    });
    // COM
    // commands
    //     .spawn(SpriteBundle {
    //         sprite: Sprite {
    //             custom_size: Vec2::new(holding.thickness() * 2.0, holding.thickness() * 2.0),
    //         },
    //         material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
    //         ..Default::default()
    //     })
    //     .with(TotalCenterOfMass);
    // Body
    let (n1, _, ls1, _, _, _) = agent.holding().get_current_state();
    let (n2, _, ls2, _, _, _) = agent.non_holding().get_current_state();
    let len = (ls1.sum() + ls2.sum()) / (n1 + n2) as f32 * 1.5;
    // Face
    let texture_handle = asset_server.load("sprites/skull.png");
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)))),
            material: materials.add(texture_handle.into()),
            transform: Transform::default()
                .with_scale(Vec3::new(len / 4.0, len / 4.0, 1.0))
                .with_translation(Vec3::new(0.0, 0.0, 0.01)),
            ..default()
        })
        .insert(Body(len / 6.0));
}

fn flush_transforms_original_holding(
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut transforms_query: Query<&mut Transform>,
    mut edge_query: Query<(Entity, &Edge, &OriginalHolding)>,
    mut vertex_query: Query<(Entity, &Vertex, &OriginalHolding)>,
    mut com_query: Query<(Entity, &PartCenterOfMass, &OriginalHolding)>,
) {
    let switchable_nr = agent.original_holding();
    let transforms = switchable_nr.pose_to_transforms();
    let (_, _, ls, _, _, _) = switchable_nr.get_current_state();
    for (entity, edge, _) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        let mut transform = transforms_query.get_mut(entity).unwrap();
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.rotation = Quat::from_rotation_z(angle);
        transform.scale = Vec3::new(ls[edge.0], 0.05, 1.0);
    }
    let vertex_positions = switchable_nr.get_all_vertices();
    for (entity, idx, _) in vertex_query.iter_mut() {
        let mut transform = transforms_query.get_mut(entity).unwrap();
        transform.translation[0] = vertex_positions[idx.0][0];
        transform.translation[1] = vertex_positions[idx.0][1];
    }
    let com = switchable_nr.get_center_of_mass();
    for (entity, _, _) in com_query.iter_mut() {
        let mut transform = transforms_query.get_mut(entity).unwrap();
        transform.translation[0] = com[0];
        transform.translation[1] = com[1];
    }
}

fn flush_transforms_original_non_holding(
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut transforms_query: Query<&mut Transform>,
    mut edge_query: Query<(Entity, &Edge, &OriginalNonHolding)>,
    mut vertex_query: Query<(Entity, &Vertex, &OriginalNonHolding)>,
    mut com_query: Query<(Entity, &PartCenterOfMass, &OriginalNonHolding)>,
) {
    let switchable_nr = agent.original_non_holding();
    let transforms = switchable_nr.pose_to_transforms();
    let (_, _, ls, _, _, _) = switchable_nr.get_current_state();
    for (entity, edge, _) in edge_query.iter_mut() {
        let (midpoint, angle) = transforms[edge.0];
        let mut transform = transforms_query.get_mut(entity).unwrap();
        transform.translation[0] = midpoint[0];
        transform.translation[1] = midpoint[1];
        transform.rotation = Quat::from_rotation_z(angle);
        transform.scale = Vec3::new(ls[edge.0], 0.05, 1.0);
    }
    let vertex_positions = switchable_nr.get_all_vertices();
    for (entity, idx, _) in vertex_query.iter_mut() {
        let mut transform = transforms_query.get_mut(entity).unwrap();
        transform.translation[0] = vertex_positions[idx.0][0];
        transform.translation[1] = vertex_positions[idx.0][1];
    }
    let com = switchable_nr.get_center_of_mass();
    for (entity, _, _) in com_query.iter_mut() {
        let mut transform = transforms_query.get_mut(entity).unwrap();
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

fn flush_transforms_body(
    agent: Res<OneHoldingSwitchableNRCouple>,
    mut body_query: Query<(&Body, &mut Transform)>,
) {
    let holding_end = agent.holding().get_last_vertex();
    for (body, mut transform) in body_query.iter_mut() {
        transform.translation[0] = holding_end[0];
        transform.translation[1] = holding_end[1] + body.0;
    }
}
