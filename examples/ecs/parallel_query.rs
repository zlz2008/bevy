//! Illustrates parallel queries with `ParallelIterator`.
//! 演示如何使用 `ParallelIterator` 进行并行查询。

use bevy::{ecs::batching::BatchingStrategy, prelude::*};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Component, Deref)]
struct Velocity(Vec2);

fn spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let texture = asset_server.load("branding/icon.png");

    // We're seeding the PRNG here to make this example deterministic for testing purposes.
    // 为了测试目的，我们在这里为伪随机数生成器（PRNG）设置种子，以使这个示例具有确定性。
    // This isn't strictly required in practical use unless you need your app to be deterministic.
    // 在实际使用中，除非你需要应用程序具有确定性，否则这不是严格必需的。
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
    for z in 0..128 {
        commands.spawn((
            Sprite::from_image(texture.clone()),
            Transform::from_scale(Vec3::splat(0.1))
               .with_translation(Vec2::splat(0.0).extend(z as f32)),
            Velocity(20.0 * Vec2::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5)),
        ));
    }
}

// Move sprites according to their velocity
// 根据精灵的速度移动精灵
fn move_system(mut sprites: Query<(&mut Transform, &Velocity)>) {
    // Compute the new location of each sprite in parallel on the
    // 在计算任务池上并行计算每个精灵的新位置
    // ComputeTaskPool
    //
    // This example is only for demonstrative purposes. Using a
    // 这个示例仅用于演示目的。对于像对仅 128 个元素进行加法这样的低成本操作，
    // ParallelIterator for an inexpensive operation like addition on only 128
    // 使用 `ParallelIterator` 通常不会比使用普通的 `Iterator` 更快。
    // elements will not typically be faster than just using a normal Iterator.
    // 有关何时使用或不使用 `ParallelIterator` 而非普通 `Iterator` 的更多信息，请参阅 `ParallelIterator` 的文档。
    // See the ParallelIterator documentation for more information on when
    // to use or not use ParallelIterator over a normal Iterator.
    sprites
       .par_iter_mut()
       .for_each(|(mut transform, velocity)| {
            transform.translation += velocity.extend(0.0);
        });
}

// Bounce sprites outside the window
// 让超出窗口范围的精灵反弹
fn bounce_system(window: Query<&Window>, mut sprites: Query<(&Transform, &mut Velocity)>) {
    let Ok(window) = window.get_single() else {
        return;
    };
    let width = window.width();
    let height = window.height();
    let left = width / -2.0;
    let right = width / 2.0;
    let bottom = height / -2.0;
    let top = height / 2.0;
    // The default batch size can also be overridden.
    // 也可以覆盖默认的批处理大小。
    // In this case a batch size of 32 is chosen to limit the overhead of
    // 在这种情况下，选择 32 的批处理大小是为了限制 `ParallelIterator` 的开销，
    // ParallelIterator, since negating a vector is very inexpensive.
    // 因为对向量取反的操作成本非常低。
    sprites
       .par_iter_mut()
       .batching_strategy(BatchingStrategy::fixed(32))
       .for_each(|(transform, mut v)| {
            if !(left < transform.translation.x
                && transform.translation.x < right
                && bottom < transform.translation.y
                && transform.translation.y < top)
            {
                // For simplicity, just reverse the velocity; don't use realistic bounces
                // 为了简单起见，只是反转速度，不使用真实的反弹效果
                v.0 = -v.0;
            }
        });
}

fn main() {
    App::new()
       .add_plugins(DefaultPlugins)
       .add_systems(Startup, spawn_system)
       .add_systems(Update, (move_system, bounce_system))
       .run();
}