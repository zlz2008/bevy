//! This example shows how you can know when a [`Component`] has been removed, so you can react to it.
//! 此示例展示了如何知晓一个 [`Component`] 何时被移除，从而对其进行响应。
//!
//! When a [`Component`] is removed from an [`Entity`], all [`Observer`] with an [`OnRemove`] trigger for
//! 当一个 [`Component`] 从一个 [`Entity`] 上被移除时，所有针对该
//! that [`Component`] will be notified. These observers will be called immediately after the
//! [`Component`] 且带有 [`OnRemove`] 触发器的 [`Observer`] 都会收到通知。这些观察者会在
//! [`Component`] is removed. For more info on observers, see the
//! [`Component`] 被移除后立即被调用。关于观察者的更多信息，请参阅
//! [observers example](https://github.com/bevyengine/bevy/blob/main/examples/ecs/observers.rs).
//! [观察者示例](https://github.com/bevyengine/bevy/blob/main/examples/ecs/observers.rs)。
//!
//! Advanced users may also consider using a lifecycle hook
//! 高级用户也可以考虑使用生命周期钩子
//! instead of an observer, as it incurs less overhead for a case like this.
//! 而非观察者，因为在这种情况下，生命周期钩子的开销更小。
//! See the [component hooks example](https://github.com/bevyengine/bevy/blob/main/examples/ecs/component_hooks.rs).
//! 请参阅 [组件钩子示例](https://github.com/bevyengine/bevy/blob/main/examples/ecs/component_hooks.rs)。
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // This system will remove a component after two seconds.
        // 这个系统会在两秒后移除一个组件。
        .add_systems(Update, remove_component)
        // This observer will react to the removal of the component.
        // 这个观察者会对组件的移除做出响应。
        .add_observer(react_on_removal)
        .run();
}

/// This `struct` is just used for convenience in this example. This is the [`Component`] we'll be
/// 这个 `struct` 仅为方便本示例使用。这就是我们将赋予 `Entity` 的 [`Component`]，
/// giving to the `Entity` so we have a [`Component`] to remove in `remove_component()`.
/// 这样在 `remove_component()` 函数中就有一个 [`Component`] 可以移除。
#[derive(Component)]
struct MyComponent;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("branding/icon.png")),
        // Add the `Component`.
        // 添加 `Component`。
        MyComponent,
    ));
}

fn remove_component(
    time: Res<Time>,
    mut commands: Commands,
    query: Query<Entity, With<MyComponent>>,
) {
    // After two seconds have passed the `Component` is removed.
    // 两秒过后，移除 `Component`。
    if time.elapsed_secs() > 2.0 {
        if let Some(entity) = query.iter().next() {
            commands.entity(entity).remove::<MyComponent>();
        }
    }
}

fn react_on_removal(trigger: Trigger<OnRemove, MyComponent>, mut query: Query<&mut Sprite>) {
    // The `OnRemove` trigger was automatically called on the `Entity` that had its `MyComponent` removed.
    // `OnRemove` 触发器会自动在移除了 `MyComponent` 的 `Entity` 上被调用。
    let entity = trigger.target();
    if let Ok(mut sprite) = query.get_mut(entity) {
        sprite.color = Color::srgb(0.5, 1., 1.);
    }
}