//! Example of how to draw to a texture from the CPU.
//! 展示如何从 CPU 绘制到纹理的示例。
//!
//! You can set the values of individual pixels to whatever you want.
//! 你可以将每个像素的值设置为你想要的任何值。
//! Bevy provides user-friendly APIs that work with [`Color`](bevy::color::Color)
//! values and automatically perform any necessary conversions and encoding
//! into the texture's native pixel format.
//! Bevy 提供了与 [`Color`](bevy::color::Color) 值一起使用的用户友好 API，并自动执行任何必要的转换和编码到纹理的本机像素格式。

use bevy::color::{color_difference::EuclideanDistance, palettes::css};
use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // In this example, we will use a fixed timestep to draw a pattern on the screen
        // one pixel at a time, so the pattern will gradually emerge over time, and
        // the speed at which it appears is not tied to the framerate.
        // 在此示例中，我们将使用固定的时间步长在屏幕上逐个像素绘制图案，因此图案会随着时间的推移逐渐出现，
        // 并且其出现速度与帧率无关。
        // Let's make the fixed update very fast, so it doesn't take too long. :)
        // 让我们使固定更新非常快，这样不会花费太长时间。:)
        .insert_resource(Time::<Fixed>::from_hz(1024.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, draw)
        .run();
}

/// Store the image handle that we will draw to, here.
/// 存储我们将要绘制的图像句柄。
#[derive(Resource)]
struct MyProcGenImage(Handle<Image>);

#[derive(Resource)]
struct SeededRng(ChaCha8Rng);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    // Create an image that we are going to draw into
    // 创建一个我们将要绘制的图像
    let mut image = Image::new_fill(
        // 2D image of size 256x256
        // 大小为 256x256 的 2D 图像
        Extent3d {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // Initialize it with a beige color
        // 使用米色初始化它
        &(css::BEIGE.to_u8_array()),
        // Use the same encoding as the color we set
        // 使用与我们设置的颜色相同的编码
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    // To make it extra fancy, we can set the Alpha of each pixel,
    // so that it fades out in a circular fashion.
    // 为了使它更加花哨，我们可以设置每个像素的 Alpha 值，使其以圆形方式淡出。
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let center = Vec2::new(IMAGE_WIDTH as f32 / 2.0, IMAGE_HEIGHT as f32 / 2.0);
            let max_radius = IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32 / 2.0;
            let r = Vec2::new(x as f32, y as f32).distance(center);
            let a = 1.0 - (r / max_radius).clamp(0.0, 1.0);

            // Here we will set the A value by accessing the raw data bytes.
            // (it is the 4th byte of each pixel, as per our `TextureFormat`)
            // 在这里，我们将通过访问原始数据字节来设置 A 值。
            // （根据我们的 `TextureFormat`，它是每个像素的第 4 个字节）

            // Find our pixel by its coordinates
            // 通过坐标找到我们的像素
            let pixel_bytes = image.pixel_bytes_mut(UVec3::new(x, y, 0)).unwrap();
            // Convert our f32 to u8
            // 将我们的 f32 转换为 u8
            pixel_bytes[3] = (a * u8::MAX as f32) as u8;
        }
    }

    // Add it to Bevy's assets, so it can be used for rendering
    // this will give us a handle we can use
    // (to display it in a sprite, or as part of UI, etc.)
    // 将其添加到 Bevy 的资源中，以便可以用于渲染
    // 这将为我们提供一个可以使用的句柄
    // （可以在精灵中显示它，或作为 UI 的一部分等）
    let handle = images.add(image);

    // Create a sprite entity using our image
    // 使用我们的图像创建一个精灵实体
    commands.spawn(Sprite::from_image(handle.clone()));
    commands.insert_resource(MyProcGenImage(handle));

    // We're seeding the PRNG here to make this example deterministic for testing purposes.
    // This isn't strictly required in practical use unless you need your app to be deterministic.
    // 我们在这里为 PRNG 设置种子，以使此示例在测试目的下具有确定性。
    // 在实际使用中，除非你需要应用程序具有确定性，否则这不是严格要求的。
    let seeded_rng = ChaCha8Rng::seed_from_u64(19878367467712);
    commands.insert_resource(SeededRng(seeded_rng));
}

/// Every fixed update tick, draw one more pixel to make a spiral pattern
/// 每次固定更新时，绘制一个像素以形成螺旋图案
fn draw(
    my_handle: Res<MyProcGenImage>,
    mut images: ResMut<Assets<Image>>,
    // Used to keep track of where we are
    // 用于跟踪我们当前的位置
    mut i: Local<u32>,
    mut draw_color: Local<Color>,
    mut seeded_rng: ResMut<SeededRng>,
) {
    if *i == 0 {
        // Generate a random color on first run.
        // 在第一次运行时生成随机颜色。
        *draw_color = Color::linear_rgb(seeded_rng.0.gen(), seeded_rng.0.gen(), seeded_rng.0.gen());
    }

    // Get the image from Bevy's asset storage.
    // 从 Bevy 的资源存储中获取图像。
    let image = images.get_mut(&my_handle.0).expect("Image not found");

    // Compute the position of the pixel to draw.
    // 计算要绘制的像素的位置。

    let center = Vec2::new(IMAGE_WIDTH as f32 / 2.0, IMAGE_HEIGHT as f32 / 2.0);
    let max_radius = IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32 / 2.0;
    let rot_speed = 0.0123;
    let period = 0.12345;

    let r = ops::sin(*i as f32 * period) * max_radius;
    let xy = Vec2::from_angle(*i as f32 * rot_speed) * r + center;
    let (x, y) = (xy.x as u32, xy.y as u32);

    // Get the old color of that pixel.
    // 获取该像素的旧颜色。
    let old_color = image.get_color_at(x, y).unwrap();

    // If the old color is our current color, change our drawing color.
    // 如果旧颜色是我们当前的颜色，则更改我们的绘制颜色。
    let tolerance = 1.0 / 255.0;
    if old_color.distance(&draw_color) <= tolerance {
        *draw_color = Color::linear_rgb(seeded_rng.0.gen(), seeded_rng.0.gen(), seeded_rng.0.gen());
    }

    // Set the new color, but keep old alpha value from image.
    // 设置新颜色，但保留图像中的旧 Alpha 值。
    image
        .set_color_at(x, y, draw_color.with_alpha(old_color.alpha()))
        .unwrap();

    *i += 1;
}