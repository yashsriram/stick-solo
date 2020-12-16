use crate::act::switchable_nr::SwitchableNR;
use bevy::prelude::*;
use std::collections::LinkedList;

#[derive(Clone)]
pub struct Path(pub LinkedList<Vec2>);

pub struct PathPlugin {
    path: Path,
}

impl PathPlugin {
    pub fn new(path: Path) -> PathPlugin {
        PathPlugin { path }
    }
}

impl Plugin for PathPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.path.clone())
            .add_startup_system(init_vis.system());
    }
}

struct Vertex(usize);

fn init_vis(mut commands: Commands, path: Res<Path>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Vertices
    for (i, vertex) in path.0.iter().enumerate() {
        commands
            .spawn(SpriteComponents {
                sprite: Sprite {
                    size: Vec2::new(
                        4.0 * SwitchableNR::GOAL_REACHED_SLACK,
                        4.0 * SwitchableNR::GOAL_REACHED_SLACK,
                    ),
                    resize_mode: SpriteResizeMode::Manual,
                },
                material: materials.add(Color::rgba(0.4, 0.4, 0.4, 0.4).into()),
                transform: Transform::from_translation(Vec3::new(vertex[0], vertex[1], 0.0)),
                ..Default::default()
            })
            .with(Vertex(i));
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
            .spawn(SpriteComponents {
                sprite: Sprite {
                    size: Vec2::new(length, 0.005),
                    resize_mode: SpriteResizeMode::Manual,
                },
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                transform: transform,
                ..Default::default()
            })
            .with(Vertex(i));
    }
}
