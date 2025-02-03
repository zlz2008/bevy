//! This example illustrates how to react to component and resource changes.
//! 本示例演示了如何对组件和资源的变化做出反应。

use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                change_component,
                change_component_2,
                change_resource,
                change_detection,
            ),
        )
        .run();
}

#[derive(Component, PartialEq, Debug)]
struct MyComponent(f32);

#[derive(Resource, PartialEq, Debug)]
struct MyResource(f32);

fn setup(mut commands: Commands) {
    // Note the first change detection log correctly points to this line because the component is
    // added. Although commands are deferred, they are able to track the original calling location.
    // 注意第一个变化检测日志正确地指向这一行，因为组件被添加了。虽然命令是延迟执行的，但它们能够跟踪原始的调用位置。
    commands.spawn(MyComponent(0.0));
    commands.insert_resource(MyResource(0.0));
}

fn change_component(time: Res<Time>, mut query: Query<(Entity, &mut MyComponent)>) {
    for (entity, mut component) in &mut query {
        if rand::thread_rng().gen_bool(0.1) {
            let new_component = MyComponent(time.elapsed_secs().round());
            info!("New value: {new_component:?} {entity}");
            // Change detection occurs on mutable dereference, and does not consider whether or not
            // a value is actually equal. To avoid triggering change detection when nothing has
            // actually changed, you can use the `set_if_neq` method on any component or resource
            // that implements PartialEq.
            // 变化检测发生在可变解引用时，并且不考虑值是否实际相等。为了避免在没有任何实际变化时触发变化检测，
            // 你可以在任何实现了PartialEq的组件或资源上使用`set_if_neq`方法。
            component.set_if_neq(new_component);
        }
    }
}

/// This is a duplicate of the `change_component` system, added to show that change tracking can
/// help you find *where* your component is being changed, when there are multiple possible
/// locations.
/// 这是`change_component`系统的副本，添加它是为了展示当有多个可能的位置时，
/// 变化跟踪可以帮助你找到组件是在*哪里*被修改的。
fn change_component_2(time: Res<Time>, mut query: Query<(Entity, &mut MyComponent)>) {
    for (entity, mut component) in &mut query {
        if rand::thread_rng().gen_bool(0.1) {
            let new_component = MyComponent(time.elapsed_secs().round());
            info!("New value: {new_component:?} {entity}");
            component.set_if_neq(new_component);
        }
    }
}

/// Change detection concepts for components apply similarly to resources.
/// 组件的变更检测概念同样适用于资源。
fn change_resource(time: Res<Time>, mut my_resource: ResMut<MyResource>) {
    if rand::thread_rng().gen_bool(0.1) {
        let new_resource = MyResource(time.elapsed_secs().round());
        info!("New value: {new_resource:?}");
        my_resource.set_if_neq(new_resource);
    }
}

/// Query filters like [`Changed<T>`] and [`Added<T>`] ensure only entities matching these filters
/// will be returned by the query.
///
/// Using the [`Ref<T>`] system param allows you to access change detection information, but does
/// not filter the query.
/// 像[`Changed<T>`]和[`Added<T>`]这样的查询过滤器确保只有匹配这些过滤器的实体
/// 会被查询返回。
///
/// 使用[`Ref<T>`]系统参数可以让你访问变更检测信息，但不会过滤查询。
fn change_detection(
    changed_components: Query<Ref<MyComponent>, Changed<MyComponent>>,
    my_resource: Res<MyResource>,
) {
    for component in &changed_components {
        // By default, you can only tell that a component was changed.
        //
        // This is useful, but what if you have multiple systems modifying the same component, how
        // will you know which system is causing the component to change?
        // 默认情况下，你只能知道组件被改变了。
        //
        // 这很有用，但如果你有多个系统修改同一个组件，
        // 你怎么知道是哪个系统导致了组件的改变？
        warn!(
            "Change detected!\n\t-> value: {:?}\n\t-> added: {}\n\t-> changed: {}\n\t-> changed by: {}",
            component,
            component.is_added(),
            component.is_changed(),
            // If you enable the `track_location` feature, you can unlock the `changed_by()`
            // method. It returns the file and line number that the component or resource was
            // changed in. It's not recommended for released games, but great for debugging!
            // 如果你启用了`track_location`特性，你可以解锁`changed_by()`方法。
            // 它返回组件或资源被修改的文件和行号。不建议在发布的游戏中使用，但对调试很有用！
            component.changed_by()
        );
    }

    if my_resource.is_changed() {
        warn!(
            "Change detected!\n\t-> value: {:?}\n\t-> added: {}\n\t-> changed: {}\n\t-> changed by: {}",
            my_resource,
            my_resource.is_added(),
            my_resource.is_changed(),
            my_resource.changed_by() // Like components, requires `track_location` feature.
        );
    }
}
