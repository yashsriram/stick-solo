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
            .add_startup_system(init.system());
    }
}

struct Vertex(usize);

fn init(mut commands: Commands, path: Res<Path>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for (i, vertex) in path.0.iter().enumerate() {
        commands
            .spawn(SpriteComponents {
                sprite: Sprite {
                    size: Vec2::new(0.04, 0.04),
                    resize_mode: SpriteResizeMode::Manual,
                },
                material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
                transform: Transform::from_translation(Vec3::new(vertex[0], vertex[1], 0.0)),
                ..Default::default()
            })
            .with(Vertex(i));
    }
}
