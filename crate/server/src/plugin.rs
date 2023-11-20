use bevy::app::{PluginGroup, PluginGroupBuilder};

/// This plugin group will add all the default plugins for a *Bevy* application:
/// * [`LogPlugin`](crate::log::LogPlugin)
/// * [`TaskPoolPlugin`](crate::core::TaskPoolPlugin)
/// * [`TypeRegistrationPlugin`](crate::core::TypeRegistrationPlugin)
/// * [`FrameCountPlugin`](crate::core::FrameCountPlugin)
/// * [`TimePlugin`](crate::time::TimePlugin)
/// * [`TransformPlugin`](crate::transform::TransformPlugin)
/// * [`HierarchyPlugin`](crate::hierarchy::HierarchyPlugin)
/// * [`DiagnosticsPlugin`](crate::diagnostic::DiagnosticsPlugin)
/// * [`InputPlugin`](crate::input::InputPlugin)
/// * [`WindowPlugin`](crate::window::WindowPlugin)
/// * [`AssetPlugin`](crate::asset::AssetPlugin) - with feature `bevy::asset`
/// * [`DebugAssetPlugin`](crate::asset::debug_asset_server::DebugAssetServerPlugin) - with feature `debug_asset_server`
/// * [`ScenePlugin`](crate::scene::ScenePlugin) - with feature `bevy::scene`
/// * [`WinitPlugin`](crate::winit::WinitPlugin) - with feature `bevy::winit`
/// * [`RenderPlugin`](crate::render::RenderPlugin) - with feature `bevy::render`
/// * [`ImagePlugin`](crate::render::texture::ImagePlugin) - with feature `bevy::render`
/// * [`PipelinedRenderingPlugin`](crate::render::pipelined_rendering::PipelinedRenderingPlugin) - with feature `bevy::render` when not targeting `wasm32`
/// * [`CorePipelinePlugin`](crate::core_pipeline::CorePipelinePlugin) - with feature `bevy::core_pipeline`
/// * [`SpritePlugin`](crate::sprite::SpritePlugin) - with feature `bevy::sprite`
/// * [`TextPlugin`](crate::text::TextPlugin) - with feature `bevy::text`
/// * [`UiPlugin`](crate::ui::UiPlugin) - with feature `bevy::ui`
/// * [`PbrPlugin`](crate::pbr::PbrPlugin) - with feature `bevy::pbr`
/// * [`GltfPlugin`](crate::gltf::GltfPlugin) - with feature `bevy::gltf`
/// * [`AudioPlugin`](crate::audio::AudioPlugin) - with feature `bevy::audio`
/// * [`GilrsPlugin`](crate::gilrs::GilrsPlugin) - with feature `bevy::gilrs`
/// * [`AnimationPlugin`](crate::animation::AnimationPlugin) - with feature `bevy::animation`
///
/// [`DefaultPlugins`] obeys *Cargo* *feature* flags. Users may exert control over this plugin group
/// by disabling `default-features` in their `Cargo.toml` and enabling only those features
/// that they wish to use.
///
/// [`DefaultPlugins`] contains all the plugins typically required to build
/// a *Bevy* application which includes a *window* and presentation components.
/// For *headless* cases – without a *window* or presentation, see [`MinimalPlugins`].
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(bevy::log::LogPlugin::default())
            .add(bevy::core::TaskPoolPlugin::default())
            .add(bevy::core::TypeRegistrationPlugin::default())
            .add(bevy::core::FrameCountPlugin::default())
            .add(bevy::time::TimePlugin::default())
            .add(bevy::transform::TransformPlugin::default())
            .add(bevy::hierarchy::HierarchyPlugin::default())
            .add(bevy::diagnostic::DiagnosticsPlugin::default())
            //.add(bevy::input::InputPlugin::default())
            //.add(bevy::window::WindowPlugin::default())
            .add(bevy::a11y::AccessibilityPlugin);

            group = group.add(bevy::asset::AssetPlugin::default());

        #[cfg(feature = "debug_asset_server")]
        {
            group = group.add(bevy::asset::debug_asset_server::DebugAssetServerPlugin::default());
        }

            group = group.add(bevy::scene::ScenePlugin::default());

            //group = group.add(bevy::winit::WinitPlugin::default());

          /*  group = group
                .add(bevy::render::RenderPlugin::default())
                // NOTE: Load this after renderer initialization so that it knows about the supported
                // compressed texture formats
                .add(bevy::render::texture::ImagePlugin::default());*/

           /* #[cfg(all(not(target_arch = "wasm32")))]
            {
                group = group
                    .add(bevy::render::pipelined_rendering::PipelinedRenderingPlugin::default());
            }*/

            group = group.add(bevy::core_pipeline::CorePipelinePlugin::default());

        #[cfg(feature = "bevy::sprite")]
        {
            group = group.add(bevy::sprite::SpritePlugin::default());
        }

        #[cfg(feature = "bevy::text")]
        {
            group = group.add(bevy::text::TextPlugin::default());
        }

        #[cfg(feature = "bevy::ui")]
        {
            group = group.add(bevy::ui::UiPlugin::default());
        }

            group = group.add(bevy::pbr::PbrPlugin::default());

        // NOTE: Load this after renderer initialization so that it knows about the supported
        // compressed texture formats
            group = group.add(bevy::gltf::GltfPlugin::default());

        #[cfg(feature = "bevy::audio")]
        {
            group = group.add(bevy::audio::AudioPlugin::default());
        }

        #[cfg(feature = "bevy::gilrs")]
        {
            group = group.add(bevy::gilrs::GilrsPlugin::default());
        }

        #[cfg(feature = "bevy::animation")]
        {
            group = group.add(bevy::animation::AnimationPlugin::default());
        }

        #[cfg(feature = "bevy::gizmos")]
        {
            group = group.add(bevy::gizmos::GizmoPlugin);
        }

        group
    }
}