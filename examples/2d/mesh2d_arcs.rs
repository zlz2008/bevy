//! Demonstrates UV mappings of the [`CircularSector`] and [`CircularSegment`] primitives.
//! 展示 [`CircularSector`] 和 [`CircularSegment`] 原语的 UV 映射。
//!
//! Also draws the bounding boxes and circles of the primitives.
//! 同时绘制原语的边界框和圆。

use std::f32::consts::FRAC_PI_2;

use bevy::{
    color::palettes::css::{BLUE, GRAY, RED},
    math::{
        bounding::{Bounded2d, BoundingVolume},
        Isometry2d,
    },
    prelude::*,
    render::mesh::{CircularMeshUvMode, CircularSectorMeshBuilder, CircularSegmentMeshBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                draw_bounds::<CircularSector>,
                draw_bounds::<CircularSegment>,
            ),
        )
        .run();
}

#[derive(Component, Debug)]
struct DrawBounds<Shape: Bounded2d + Send + Sync + 'static>(Shape);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(asset_server.load("branding/icon.png"));

    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(GRAY.into()),
            ..default()
        },
    ));

    const NUM_SLICES: i32 = 8;
    const SPACING_X: f32 = 100.0;
    const OFFSET_X: f32 = SPACING_X * (NUM_SLICES - 1) as f32 / 2.0;

    // This draws NUM_SLICES copies of the Bevy logo as circular sectors and segments,
    // 这将绘制 NUM_SLICES 个 Bevy 徽标作为圆形扇形和线段，
    // with successively larger angles up to a complete circle.
    // 角度逐渐增大，直到形成一个完整的圆。
    for i in 0..NUM_SLICES {
        let fraction = (i + 1) as f32 / NUM_SLICES as f32;

        let sector = CircularSector::from_turns(40.0, fraction);
        // We want to rotate the circular sector so that the sectors appear clockwise from north.
        // 我们希望旋转圆形扇形，使扇形从北方向顺时针出现。
        // We must rotate it both in the Transform and in the mesh's UV mappings.
        // 我们必须在 Transform 和网格的 UV 映射中都进行旋转。
        let sector_angle = -sector.half_angle();
        let sector_mesh =
            CircularSectorMeshBuilder::new(sector).uv_mode(CircularMeshUvMode::Mask {
                angle: sector_angle,
            });
        commands.spawn((
            Mesh2d(meshes.add(sector_mesh)),
            MeshMaterial2d(material.clone()),
            Transform {
                translation: Vec3::new(SPACING_X * i as f32 - OFFSET_X, 50.0, 0.0),
                rotation: Quat::from_rotation_z(sector_angle),
                ..default()
            },
            DrawBounds(sector),
        ));

        let segment = CircularSegment::from_turns(40.0, fraction);
        // For the circular segment, we will draw Bevy charging forward, which requires rotating the
        // 对于圆形线段，我们将绘制 Bevy 向前冲刺，这需要旋转
        // shape and texture by 90 degrees.
        // 形状和纹理旋转 90 度。
        //
        // Note that this may be unintuitive; it may feel like we should rotate the texture by the
        // 请注意，这可能不直观；可能感觉我们应该将纹理旋转
        // opposite angle to preserve the orientation of Bevy. But the angle is not the angle of the
        // 相反的角度以保持 Bevy 的方向。但角度不是纹理本身的角度，
        // texture itself, rather it is the angle at which the vertices are mapped onto the texture.
        // 而是顶点映射到纹理的角度。
        // so it is the negative of what you might otherwise expect.
        // 因此它是你可能会预期的相反角度。
        let segment_angle = -FRAC_PI_2;
        let segment_mesh =
            CircularSegmentMeshBuilder::new(segment).uv_mode(CircularMeshUvMode::Mask {
                angle: -segment_angle,
            });
        commands.spawn((
            Mesh2d(meshes.add(segment_mesh)),
            MeshMaterial2d(material.clone()),
            Transform {
                translation: Vec3::new(SPACING_X * i as f32 - OFFSET_X, -50.0, 0.0),
                rotation: Quat::from_rotation_z(segment_angle),
                ..default()
            },
            DrawBounds(segment),
        ));
    }
}

fn draw_bounds<Shape: Bounded2d + Send + Sync + 'static>(
    q: Query<(&DrawBounds<Shape>, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (shape, transform) in &q {
        let (_, rotation, translation) = transform.to_scale_rotation_translation();
        let translation = translation.truncate();
        let rotation = rotation.to_euler(EulerRot::XYZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));

        let aabb = shape.0.aabb_2d(isometry);
        gizmos.rect_2d(aabb.center(), aabb.half_size() * 2.0, RED);

        let bounding_circle = shape.0.bounding_circle(isometry);
        gizmos.circle_2d(bounding_circle.center, bounding_circle.radius(), BLUE);
    }
}