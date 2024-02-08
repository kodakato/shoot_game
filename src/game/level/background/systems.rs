use bevy::core_pipeline::fullscreen_vertex_shader::FULLSCREEN_SHADER_HANDLE;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{
    PrimitiveState, RenderPipelineDescriptor, SpecializedMeshPipelineError,
};
use bevy::sprite::Material2dKey;
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use bevy_tiling_background::{
    BackgroundMaterial, BackgroundMovementScale, CustomBackgroundImageBundle, ScrollingBackground,
    SetImageRepeatingExt, TilingBackgroundPlugin,
};
pub fn setup_background (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let backgorund_image = asset_server.load("sprites/stars.png");
    commands.spawn(SpriteBundle {
        texture: backgorund_image,
        ..Default::default()
    });
    
}


