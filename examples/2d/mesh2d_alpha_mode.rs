//! This example is used to test how transforms interact with alpha modes for [`Mesh2d`] entities with a [`MeshMaterial2d`].
//! 此示例用于测试带有 [`MeshMaterial2d`] 的 [`Mesh2d`] 实体的变换如何与 alpha 模式交互。
//! This makes sure the depth buffer is correctly being used for opaque and transparent 2d meshes
//! 这确保深度缓冲区正确用于不透明和透明的 2D 网格。

use bevy::{
    color::palettes::css::{BLUE, GREEN, WHITE},
    prelude::*,
    sprite::AlphaMode2d,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let texture_handle = asset_server.load("branding/icon.png");
    let mesh_handle = meshes.add(Rectangle::from_size(Vec2::splat(256.0)));

    // opaque
    // 不透明
    // Each sprite should be square with the transparent parts being completely black
    // 每个精灵应该是正方形的，透明部分完全为黑色
    // The blue sprite should be on top with the white and green one behind it
    // 蓝色精灵应该在顶部，白色和绿色精灵在其后面
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: WHITE.into(),
            alpha_mode: AlphaMode2d::Opaque,
            texture: Some(texture_handle.clone()),
        })),
        Transform::from_xyz(-400.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: BLUE.into(),
            alpha_mode: AlphaMode2d::Opaque,
            texture: Some(texture_handle.clone()),
        })),
        Transform::from_xyz(-300.0, 0.0, 1.0),
    ));
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: GREEN.into(),
            alpha_mode: AlphaMode2d::Opaque,
            texture: Some(texture_handle.clone()),
        })),
        Transform::from_xyz(-200.0, 0.0, -1.0),
    ));

    // Test the interaction between opaque/mask and transparent meshes
    // 测试不透明/遮罩和透明网格之间的交互
    // The white sprite should be:
    // 白色精灵应该是：
    // - only the icon is opaque but background is transparent
    // - 只有图标是不透明的，但背景是透明的
    // - on top of the green sprite
    // - 在绿色精灵的顶部
    // - behind the blue sprite
    // - 在蓝色精灵的后面
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: WHITE.into(),
            alpha_mode: AlphaMode2d::Mask(0.5),
            texture: Some(texture_handle.clone()),
        })),
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: BLUE.with_alpha(0.7).into(),
            alpha_mode: AlphaMode2d::Blend,
            texture: Some(texture_handle.clone()),
        })),
        Transform::from_xyz(300.0, 0.0, 1.0),
    ));
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: GREEN.with_alpha(0.7).into(),
            alpha_mode: AlphaMode2d::Blend,
            texture: Some(texture_handle),
        })),
        Transform::from_xyz(400.0, 0.0, -1.0),
    ));
}