use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct CustomPlugins;

impl PluginGroup for CustomPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(bevy::type_registry::TypeRegistryPlugin::default());
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(bevy::input::InputPlugin::default());
        group.add(bevy::window::WindowPlugin::default());
        group.add(bevy::asset::AssetPlugin::default());
        group.add(bevy::scene::ScenePlugin::default());
        group.add(bevy::render::RenderPlugin::default());
        group.add(bevy::sprite::SpritePlugin::default());
        group.add(bevy::pbr::PbrPlugin::default());
        group.add(bevy::ui::UiPlugin::default());
        group.add(bevy::text::TextPlugin::default());
        // group.add(bevy::audio::AudioPlugin::default());
        group.add(bevy::gltf::GltfPlugin::default());
        group.add(bevy::winit::WinitPlugin::default());
        group.add(bevy::wgpu::WgpuPlugin::default());
        #[cfg(feature = "bevy_gilrs")]
        group.add(bevy::gilrs::GilrsPlugin::default());
    }
}
