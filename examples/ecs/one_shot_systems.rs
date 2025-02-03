//! Demonstrates the use of "one-shot systems", which run once when triggered.
//! 演示了“一次性系统”的使用，这类系统在被触发时仅运行一次。
//!
//! These can be useful to help structure your logic in a push-based fashion,
//! 它们有助于以基于推送的方式组织你的逻辑，
//! reducing the overhead of running extremely rarely run systems
//! 减少极少运行的系统的运行开销，
//! and improving schedule flexibility.
//! 并提高调度的灵活性。
//!
//! See the [`World::run_system`](World::run_system) or
//! 有关更多详细信息，请参阅 [`World::run_system`](World::run_system) 或
//! [`World::run_system_once`](World#method.run_system_once_with)
//! [`World::run_system_once`](World#method.run_system_once_with) 文档。
//! docs for more details.

use bevy::{
    ecs::system::{RunSystemOnce, SystemId},
    prelude::*,
};

fn main() {
    App::new()
       .add_plugins(DefaultPlugins)
       .add_systems(
            Startup,
            (
                setup_ui,
                setup_with_commands,
                setup_with_world.after(setup_ui), // since we run `system_b` once in world it needs to run after `setup_ui`
                // 由于我们在世界中运行了一次 `system_b`，它需要在 `setup_ui` 之后运行
            ),
        )
       .add_systems(Update, (trigger_system, evaluate_callbacks).chain())
       .run();
}

#[derive(Component)]
struct Callback(SystemId);

#[derive(Component)]
struct Triggered;

#[derive(Component)]
struct A;
#[derive(Component)]
struct B;

fn setup_with_commands(mut commands: Commands) {
    let system_id = commands.register_system(system_a);
    commands.spawn((Callback(system_id), A));
}

fn setup_with_world(world: &mut World) {
    // We can run it once manually
    // 我们可以手动运行一次
    world.run_system_once(system_b).unwrap();
    // Or with a Callback
    // 或者使用回调
    let system_id = world.register_system(system_b);
    world.spawn((Callback(system_id), B));
}

/// Tag entities that have callbacks we want to run with the `Triggered` component.
/// 用 `Triggered` 组件标记那些具有我们想要运行的回调的实体。
fn trigger_system(
    mut commands: Commands,
    query_a: Single<Entity, With<A>>,
    query_b: Single<Entity, With<B>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyA) {
        let entity = *query_a;
        commands.entity(entity).insert(Triggered);
    }
    if input.just_pressed(KeyCode::KeyB) {
        let entity = *query_b;
        commands.entity(entity).insert(Triggered);
    }
}

/// Runs the systems associated with each `Callback` component if the entity also has a `Triggered` component.
/// 如果实体同时具有 `Triggered` 组件，则运行与每个 `Callback` 组件关联的系统。
///
/// This could be done in an exclusive system rather than using `Commands` if preferred.
/// 如果愿意，也可以在排他系统中完成此操作，而不是使用 `Commands`。
fn evaluate_callbacks(query: Query<(Entity, &Callback), With<Triggered>>, mut commands: Commands) {
    for (entity, callback) in query.iter() {
        commands.run_system(callback.0);
        commands.entity(entity).remove::<Triggered>();
    }
}

fn system_a(entity_a: Single<Entity, With<Text>>, mut writer: TextUiWriter) {
    *writer.text(*entity_a, 3) = String::from("A");
    info!("A: One shot system registered with Commands was triggered");
    info!("A：使用 Commands 注册的一次性系统被触发");
}

fn system_b(entity_b: Single<Entity, With<Text>>, mut writer: TextUiWriter) {
    *writer.text(*entity_b, 3) = String::from("B");
    info!("B: One shot system registered with World was triggered");
    info!("B：使用 World 注册的一次性系统被触发");
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
       .spawn((
            Text::default(),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
               ..default()
            },
        ))
       .with_children(|p| {
            p.spawn(TextSpan::new("Press A or B to trigger a one-shot system\n"));
            p.spawn(TextSpan::new("Last Triggered: "));
            p.spawn((
                TextSpan::new("-"),
                TextColor(bevy::color::palettes::css::ORANGE.into()),
            ));
        });
}