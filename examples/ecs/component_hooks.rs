//! This example illustrates the different ways you can employ component lifecycle hooks.
//! 这个示例展示了可以使用组件生命周期钩子的不同方式。
//!
//! Whenever possible, prefer using Bevy's change detection or Events for reacting to component changes.
//! 只要有可能，优先使用 Bevy 的变更检测或事件来响应组件的变化。
//! Events generally offer better performance and more flexible integration into Bevy's systems.
//! 事件通常能提供更好的性能，并且能更灵活地集成到 Bevy 的系统中。
//! Hooks are useful to enforce correctness but have limitations (only one hook per component,
//! 钩子对于确保正确性很有用，但也有局限性（每个组件只能有一个钩子，
//! less ergonomic than events).
//! 不如事件使用起来方便）。
//!
//! Here are some cases where components hooks might be necessary:
//! 以下是一些可能需要使用组件钩子的情况：
//!
//! - Maintaining indexes: If you need to keep custom data structures (like a spatial index) in
//! - 维护索引：如果你需要保持自定义数据结构（如空间索引）
//!     sync with the addition/removal of components.
//! 与组件的添加/移除同步。
//!
//! - Enforcing structural rules: When you have systems that depend on specific relationships
//! - 强制执行结构规则：当你有依赖于组件之间特定关系的系统时
//!     between components (like hierarchies or parent-child links) and need to maintain correctness.
//! （如层级结构或父子链接），并且需要确保正确性。

use bevy::{
    ecs::component::{ComponentHooks, HookContext, Mutable, StorageType},
    prelude::*,
};
use std::collections::HashMap;

#[derive(Debug)]
/// Hooks can also be registered during component initialization by
/// 也可以在组件初始化期间通过以下方式注册钩子：
/// using [`Component`] derive macro:
/// 使用 [`Component`] 派生宏：
/// ```no_run
/// #[derive(Component)]
/// #[component(on_add = ..., on_insert = ..., on_replace = ..., on_remove = ...)]
/// ```
struct MyComponent(KeyCode);

impl Component for MyComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;

    /// Hooks can also be registered during component initialization by
    /// 也可以在组件初始化期间通过以下方式注册钩子：
    /// implementing `register_component_hooks`
    /// 实现 `register_component_hooks` 方法
    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        // Register hooks...
        // 注册钩子...
    }
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct MyComponentIndex(HashMap<KeyCode, Entity>);

#[derive(Event)]
struct MyEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, trigger_hooks)
        .init_resource::<MyComponentIndex>()
        .add_event::<MyEvent>()
        .run();
}

fn setup(world: &mut World) {
    // In order to register component hooks the component must:
    // 为了注册组件钩子，组件必须满足以下条件：
    // - not be currently in use by any entities in the world
    // - 目前世界中没有任何实体正在使用该组件
    // - not already have a hook of that kind registered
    // - 尚未注册过该类型的钩子
    // This is to prevent overriding hooks defined in plugins and other crates as well as keeping things fast
    // 这是为了防止覆盖插件和其他 crate 中定义的钩子，同时保持性能高效
    world
        .register_component_hooks::<MyComponent>()
        // There are 4 component lifecycle hooks: `on_add`, `on_insert`, `on_replace` and `on_remove`
        // 有 4 种组件生命周期钩子：`on_add`、`on_insert`、`on_replace` 和 `on_remove`
        // A hook has 2 arguments:
        // 一个钩子有两个参数：
        // - a `DeferredWorld`, this allows access to resource and component data as well as `Commands`
        // - 一个 `DeferredWorld`，它允许访问资源和组件数据以及 `Commands`
        // - a `HookContext`, this provides access to the following contextual information:
        // - 一个 `HookContext`，它提供以下上下文信息：
        //   - the entity that triggered the hook
        //   - 触发钩子的实体
        //   - the component id of the triggering component, this is mostly used for dynamic components
        //   - 触发钩子的组件的组件 ID，这主要用于动态组件
        //   - the location of the code that caused the hook to trigger
        //   - 导致钩子触发的代码位置
        //
        // `on_add` will trigger when a component is inserted onto an entity without it
        // `on_add` 会在将组件插入到没有该组件的实体上时触发
        .on_add(
            |mut world,
             HookContext {
                 entity,
                 component_id,
                 caller,
             }| {
                // You can access component data from within the hook
                // 你可以在钩子内部访问组件数据
                let value = world.get::<MyComponent>(entity).unwrap().0;
                println!(
                    "{component_id:?} added to {entity} with value {value:?}{}",
                    caller
                        .map(|location| format!("due to {location}"))
                        .unwrap_or_default()
                );
                // Or access resources
                // 或者访问资源
                world
                    .resource_mut::<MyComponentIndex>()
                    .insert(value, entity);
                // Or send events
                // 或者发送事件
                world.send_event(MyEvent);
            },
        )
        // `on_insert` will trigger when a component is inserted onto an entity,
        // `on_insert` 会在将组件插入到实体上时触发，
        // regardless of whether or not it already had it and after `on_add` if it ran
        // 无论该实体之前是否已有该组件，如果 `on_add` 触发了，它会在 `on_add` 之后触发
        .on_insert(|world, _| {
            println!("Current Index: {:?}", world.resource::<MyComponentIndex>());
        })
        // `on_replace` will trigger when a component is inserted onto an entity that already had it,
        // `on_replace` 会在将组件插入到已经拥有该组件的实体上时触发，
        // and runs before the value is replaced.
        // 并且会在值被替换之前运行。
        // Also triggers when a component is removed from an entity, and runs before `on_remove`
        // 当从实体中移除组件时也会触发，并且会在 `on_remove` 之前运行
        .on_replace(|mut world, context| {
            let value = world.get::<MyComponent>(context.entity).unwrap().0;
            world.resource_mut::<MyComponentIndex>().remove(&value);
        })
        // `on_remove` will trigger when a component is removed from an entity,
        // `on_remove` 会在从实体中移除组件时触发，
        // since it runs before the component is removed you can still access the component data
        // 因为它会在组件被移除之前运行，所以你仍然可以访问组件数据
        .on_remove(
            |mut world,
             HookContext {
                 entity,
                 component_id,
                 caller,
             }| {
                let value = world.get::<MyComponent>(entity).unwrap().0;
                println!(
                    "{component_id:?} removed from {entity} with value {value:?}{}",
                    caller
                        .map(|location| format!("due to {location}"))
                        .unwrap_or_default()
                );
                // You can also issue commands through `.commands()`
                // 你也可以通过 `.commands()` 发出命令
                world.commands().entity(entity).despawn();
            },
        );
}

fn trigger_hooks(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    index: Res<MyComponentIndex>,
) {
    for (key, entity) in index.iter() {
        if !keys.pressed(*key) {
            commands.entity(*entity).remove::<MyComponent>();
        }
    }
    for key in keys.get_just_pressed() {
        commands.spawn(MyComponent(*key));
    }
}