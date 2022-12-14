use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::collections::LinkedList;

#[derive(Clone)]
pub struct Path(pub LinkedList<Vec2>);

impl Default for Path {
    fn default() -> Self {
        Path({
            let pi = std::f32::consts::PI;
            let mut path = LinkedList::new();
            let parts = 10usize;
            for i in 0..parts {
                let theta = 2.0 * pi * (i as f32) / (parts as f32);
                path.push_back(Vec2::new(-100.0 + 100.0 * theta.cos(), 100.0 * theta.sin()));
            }
            let parts = 8usize;
            for i in 0..parts {
                let theta = 2.0 * pi * ((parts - i) as f32) / (parts as f32) + pi;
                path.push_back(Vec2::new(50.0 + 50. * theta.cos(), 50. * theta.sin()));
            }
            for i in 0..5 {
                path.push_back(Vec2::new(0.0, 20. * i as f32));
            }
            for i in 0..5 {
                path.push_back(Vec2::new(20. * i as f32, 80.0));
            }
            for i in (0..5).rev() {
                path.push_back(Vec2::new(80.0, 20. * i as f32));
            }
            for i in (0..5).rev() {
                path.push_back(Vec2::new(20. * i as f32, 0.0));
            }
            path
        })
    }
}

pub struct PathPlugin {
    path: Path,
}

impl PathPlugin {
    pub fn new(path: Path) -> PathPlugin {
        PathPlugin { path }
    }
}

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.path.clone())
            .add_startup_system(init_vis);
    }
}

#[derive(Component)]
struct Vertex(usize);

fn init_vis(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    path: Res<Path>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Vertices
    for (i, vertex) in path.0.iter().enumerate() {
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(4.0, 4.0))))
                    .into(),
                material: materials.add(Color::rgba(0.4, 0.4, 0.4, 0.4).into()),
                transform: Transform::from_translation(Vec3::new(vertex[0], vertex[1], 0.0)),
                ..default()
            })
            .insert(Vertex(i));
    }
    // Edges
    let vertices_vec = path.0.iter().map(|&v| v).collect::<Vec<Vec2>>();
    for (i, &vertex) in vertices_vec.iter().enumerate().skip(1) {
        let prev_vertex = vertices_vec[i - 1];
        let length = (vertex - prev_vertex).length();
        let transform = {
            let midpoint = (prev_vertex + vertex) / 2.0;
            let mut translation =
                Transform::from_translation(Vec3::new(midpoint[0], midpoint[1], 0.0));
            let angle = (prev_vertex[1] - vertex[1]).atan2(prev_vertex[0] - vertex[0]);
            translation.rotate(Quat::from_rotation_z(angle));
            translation
        };
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(length, 3.))))
                    .into(),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                transform,
                ..default()
            })
            .insert(Vertex(i));
    }
}
