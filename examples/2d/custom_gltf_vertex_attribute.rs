//! Renders a glTF mesh in 2D with a custom vertex attribute.
//! 使用自定义顶点属性在 2D 中渲染 glTF 网格。

use bevy::{
    gltf::GltfPlugin,
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef},
        render_resource::*,
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
};

/// This example uses a shader source file from the assets subdirectory
/// 此示例使用 assets 子目录中的着色器源文件
const SHADER_ASSET_PATH: &str = "shaders/custom_gltf_2d.wgsl";

/// This vertex attribute supplies barycentric coordinates for each triangle.
/// 此顶点属性为每个三角形提供重心坐标。
///
/// Each component of the vector corresponds to one corner of a triangle. It's
/// equal to 1.0 in that corner and 0.0 in the other two. Hence, its value in
/// the fragment shader indicates proximity to a corner or the opposite edge.
/// 向量的每个分量对应于三角形的一个角。在该角为 1.0，在其他两个角为 0.0。
/// 因此，其在片段着色器中的值表示接近某个角或对边。
const ATTRIBUTE_BARYCENTRIC: MeshVertexAttribute =
    MeshVertexAttribute::new("Barycentric", 2137464976, VertexFormat::Float32x3);

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
            ..default()
        })
        .add_plugins((
            DefaultPlugins.set(
                GltfPlugin::default()
                    // Map a custom glTF attribute name to a `MeshVertexAttribute`.
                    // 将自定义的 glTF 属性名称映射到 `MeshVertexAttribute`。
                    // The glTF file used here has an attribute name with *two*
                    // underscores: __BARYCENTRIC
                    // 此处使用的 glTF 文件具有带有 *两个* 下划线的属性名称：__BARYCENTRIC
                    // One is stripped to do the comparison here.
                    // 在此处进行比较时，一个下划线被去除。
                    .add_custom_vertex_attribute("_BARYCENTRIC", ATTRIBUTE_BARYCENTRIC),
            ),
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // Add a mesh loaded from a glTF file. This mesh has data for `ATTRIBUTE_BARYCENTRIC`.
    // 添加从 glTF 文件加载的网格。此网格具有 `ATTRIBUTE_BARYCENTRIC` 的数据。
    let mesh = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("models/barycentric/barycentric.gltf"),
    );
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(CustomMaterial {})),
        Transform::from_scale(150.0 * Vec3::ONE),
    ));

    commands.spawn(Camera2d);
}

/// This custom material uses barycentric coordinates from
/// `ATTRIBUTE_BARYCENTRIC` to shade a white border around each triangle. The
/// thickness of the border is animated using the global time shader uniform.
/// 此自定义材质使用来自 `ATTRIBUTE_BARYCENTRIC` 的重心坐标为每个三角形绘制白色边框。
/// 边框的厚度使用全局时间着色器 uniform 进行动画处理。
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(1),
            ATTRIBUTE_BARYCENTRIC.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}