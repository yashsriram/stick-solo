use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup};

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "477f4532-fbc4-4faf-a5aa-da4fea8f22d2"]
pub struct SimpleMaterial {}

impl Material for SimpleMaterial {
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
