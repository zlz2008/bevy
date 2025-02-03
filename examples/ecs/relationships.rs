//! Entities generally don't exist in isolation. Instead, they are related to other entities in various ways.
//! 实体通常不会孤立存在，相反，它们会以各种方式与其他实体相关联。
//! While Bevy comes with a built-in [`ChildOf`]/[`Children`] relationship
//! 虽然 Bevy 自带了内置的 [`ChildOf`]/[`Children`] 关系
//! (which enables transform and visibility propagation),
//! （这种关系支持变换和可见性的传播）
//! you can define your own relationships using components.
//! 但你也可以使用组件来定义自己的关系。
//!
//! We can define a custom relationship by creating two components:
//! 我们可以通过创建两个组件来定义自定义关系：
//! one to store the relationship itself, and another to keep track of the reverse relationship.
//! 一个用于存储关系本身，另一个用于跟踪反向关系。
//! Bevy's [`ChildOf`] component implements the [`Relationship`] trait, serving as the source of truth,
//! Bevy 的 [`ChildOf`] 组件实现了 [`Relationship`] 特征，作为事实的来源，
//! while the [`Children`] component implements the [`RelationshipTarget`] trait and is used to accelerate traversals down the hierarchy.
//! 而 [`Children`] 组件实现了 [`RelationshipTarget`] 特征，用于加速在层级结构中的遍历。
//!
//! In this example we're creating a [`Targeting`]/[`TargetedBy`] relationship,
//! 在这个示例中，我们将创建一个 [`Targeting`]/[`TargetedBy`] 关系，
//! demonstrating how you might model units which target a single unit in combat.
//! 展示如何对战斗中一个单位瞄准另一个单位的情况进行建模。

use bevy::ecs::entity::hash_set::EntityHashSet;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;

/// The entity that this entity is targeting.
/// 此实体正在瞄准的实体。
///
/// This is the source of truth for the relationship,
/// 这是该关系的事实来源，
/// and can be modified directly to change the target.
/// 可以直接修改它来改变目标。
#[derive(Component, Debug)]
#[relationship(relationship_target = TargetedBy)]
struct Targeting(Entity);

/// All entities that are targeting this entity.
/// 所有正在瞄准此实体的实体。
///
/// This component is updated reactively using the component hooks introduced by deriving
/// 这个组件会通过派生的组件钩子进行被动更新，
/// the [`Relationship`] trait. We should not modify this component directly,
/// 我们不应该直接修改这个组件，
/// but can safely read its field. In a larger project, we could enforce this through the use of
/// 但可以安全地读取其字段。在更大的项目中，我们可以通过使用
/// private fields and public getters.
/// 私有字段和公共访问器来强制实施这一点。
#[derive(Component, Debug)]
#[relationship_target(relationship = Targeting)]
struct TargetedBy(Vec<Entity>);

fn main() {
    // Operating on a raw `World` and running systems one at a time
    // 直接操作原始的 `World` 并逐个运行系统
    // is great for writing tests and teaching abstract concepts!
    // 对于编写测试和讲解抽象概念非常有用！
    let mut world = World::new();

    // We're going to spawn a few entities and relate them to each other in a complex way.
    // 我们将生成几个实体，并以复杂的方式将它们相互关联起来。
    // To start, Bob will target Alice, Charlie will target Bob,
    // 首先，Bob 将瞄准 Alice，Charlie 将瞄准 Bob，
    // and Alice will target Charlie. This creates a loop in the relationship graph.
    // 而 Alice 将瞄准 Charlie。这会在关系图中形成一个循环。
    //
    // Then, we'll spawn Devon, who will target Charlie,
    // 然后，我们将生成 Devon，他将瞄准 Charlie，
    // creating a more complex graph with a branching structure.
    // 从而创建一个具有分支结构的更复杂的图。
    fn spawning_entities_with_relationships(mut commands: Commands) {
        // Calling .id() after spawning an entity will return the `Entity` identifier of the spawned entity,
        // 在生成实体后调用 .id() 会返回所生成实体的 `Entity` 标识符，
        // even though the entity itself is not yet instantiated in the world.
        // 即使该实体本身尚未在世界中实例化。
        // This works because Commands will reserve the entity ID before actually spawning the entity,
        // 这是因为 Commands 会在实际生成实体之前预留实体 ID，
        // through the use of atomic counters.
        // 通过使用原子计数器来实现。
        let alice = commands.spawn(Name::new("Alice")).id();
        // Relations are just components, so we can add them into the bundle that we're spawning.
        // 关系只是组件，所以我们可以将它们添加到正在生成的实体包中。
        let bob = commands.spawn((Name::new("Bob"), Targeting(alice))).id();

        // The `with_related` helper method on `EntityCommands` can be used to add relations in a more ergonomic way.
        // `EntityCommands` 上的 `with_related` 辅助方法可以更方便地添加关系。
        let charlie = commands
           .spawn((Name::new("Charlie"), Targeting(bob)))
            // The `with_related` method will automatically add the `Targeting` component to any entities spawned within the closure,
            // `with_related` 方法会自动为闭包内生成的任何实体添加 `Targeting` 组件，
            // targeting the entity that we're calling `with_related` on.
            // 这些实体将瞄准我们调用 `with_related` 的那个实体。
           .with_related::<Targeting>(|related_spawner_commands| {
                // We could spawn multiple entities here, and they would all target `charlie`.
                // 我们可以在这里生成多个实体，它们都将瞄准 `charlie`。
                related_spawner_commands.spawn(Name::new("Devon"));
            })
           .id();

        // Simply inserting the `Targeting` component will automatically create and update the `TargetedBy` component on the target entity.
        // 简单地插入 `Targeting` 组件会自动在目标实体上创建并更新 `TargetedBy` 组件。
        // We can do this at any point; not just when the entity is spawned.
        // 我们可以在任何时候进行此操作，而不仅仅是在实体生成时。
        commands.entity(alice).insert(Targeting(charlie));
    }

    world
       .run_system_once(spawning_entities_with_relationships)
       .unwrap();

    fn debug_relationships(
        // Not all of our entities are targeted by something, so we use `Option` in our query to handle this case.
        // 并非所有实体都被其他实体瞄准，所以我们在查询中使用 `Option` 来处理这种情况。
        relations_query: Query<(&Name, &Targeting, Option<&TargetedBy>)>,
        name_query: Query<&Name>,
    ) {
        let mut relationships = String::new();

        for (name, targeting, maybe_targeted_by) in relations_query.iter() {
            let targeting_name = name_query.get(targeting.0).unwrap();
            let targeted_by_string = if let Some(targeted_by) = maybe_targeted_by {
                let mut vec_of_names = Vec::<&Name>::new();

                for entity in &targeted_by.0 {
                    let name = name_query.get(*entity).unwrap();
                    vec_of_names.push(name);
                }

                // Convert this to a nice string for printing.
                // 将其转换为适合打印的字符串。
                let vec_of_str: Vec<&str> = vec_of_names.iter().map(|name| name.as_str()).collect();
                vec_of_str.join(", ")
            } else {
                "nobody".to_string()
            };

            relationships.push_str(&format!(
                "{name} is targeting {targeting_name}, and is targeted by {targeted_by_string}\n",
            ));
        }

        println!("{}", relationships);
    }

    world.run_system_once(debug_relationships).unwrap();

    // Demonstrates how to correctly mutate relationships.
    // 展示如何正确地修改关系。
    // Relationship components are immutable! We can't query for the `Targeting` component mutably and modify it directly,
    // 关系组件是不可变的！我们不能可变地查询 `Targeting` 组件并直接修改它，
    // but we can insert a new `Targeting` component to replace the old one.
    // 但我们可以插入一个新的 `Targeting` 组件来替换旧的。
    // This allows the hooks on the `Targeting` component to update the `TargetedBy` component correctly.
    // 这样可以让 `Targeting` 组件上的钩子正确地更新 `TargetedBy` 组件。
    // The `TargetedBy` component will be updated automatically!
    // `TargetedBy` 组件会自动更新！
    fn mutate_relationships(name_query: Query<(Entity, &Name)>, mut commands: Commands) {
        // Let's find Devon by doing a linear scan of the entity names.
        // 我们通过线性扫描实体名称来找到 Devon。
        let devon = name_query
           .iter()
           .find(|(_entity, name)| name.as_str() == "Devon")
           .unwrap()
           .0;

        let alice = name_query
           .iter()
           .find(|(_entity, name)| name.as_str() == "Alice")
           .unwrap()
           .0;

        println!("Making Devon target Alice.\n");
        commands.entity(devon).insert(Targeting(alice));
    }

    world.run_system_once(mutate_relationships).unwrap();
    world.run_system_once(debug_relationships).unwrap();

    // Systems can return errors,
    // 系统可以返回错误，
    // which can be used to signal that something went wrong during the system's execution.
    // 这些错误可用于表明系统执行过程中出现了问题。
    #[derive(Debug)]
    #[expect(
        dead_code,
        reason = "Rust considers types that are only used by their debug trait as dead code."
    )]
    struct TargetingCycle {
        initial_entity: Entity,
        visited: EntityHashSet,
    }

    /// Bevy's relationships come with all sorts of useful methods for traversal.
    /// Bevy 的关系提供了各种有用的遍历方法。
    /// Here, we're going to look for cycles using a depth-first search.
    /// 在这里，我们将使用深度优先搜索来查找循环。
    fn check_for_cycles(
        // We want to check every entity for cycles
        // 我们要检查每个实体是否存在循环
        query_to_check: Query<Entity, With<Targeting>>,
        // Fetch the names for easier debugging.
        // 获取名称以便于调试。
        name_query: Query<&Name>,
        // The targeting_query allows us to traverse the relationship graph.
        // targeting_query 允许我们遍历关系图。
        targeting_query: Query<&Targeting>,
    ) -> Result<(), TargetingCycle> {
        for initial_entity in query_to_check.iter() {
            let mut visited = EntityHashSet::new();
            let mut targeting_name = name_query.get(initial_entity).unwrap().clone();
            println!("Checking for cycles starting at {targeting_name}",);

            // There's all sorts of methods like this; check the `Query` docs for more!
            // 有很多类似这样的方法；更多信息请查看 `Query` 的文档！
            // This would also be easy to do by just manually checking the `Targeting` component,
            // 也可以通过手动检查 `Targeting` 组件来实现，
            // and calling `query.get(targeted_entity)` on the entity that it targets in a loop.
            // 并在循环中对它所瞄准的实体调用 `query.get(targeted_entity)`。
            for targeting in targeting_query.iter_ancestors(initial_entity) {
                let target_name = name_query.get(targeting).unwrap();
                println!("{targeting_name} is targeting {target_name}",);
                targeting_name = target_name.clone();

                if !visited.insert(targeting) {
                    return Err(TargetingCycle {
                        initial_entity,
                        visited,
                    });
                }
            }
        }

        // If we've checked all the entities and haven't found a cycle, we're good!
        // 如果我们检查了所有实体都没有发现循环，那就没问题了！
        Ok(())
    }

    // Calling `world.run_system_once` on systems which return Results gives us two layers of errors:
    // 对返回 Result 的系统调用 `world.run_system_once` 会产生两层错误：
    // the first checks if running the system failed, and the second checks if the system itself returned an error.
    // 第一层检查运行系统是否失败，第二层检查系统本身是否返回了错误。
    // We're unwrapping the first, but checking the output of the system itself.
    // 我们解开第一层错误，但检查系统本身的输出。
    let cycle_result = world.run_system_once(check_for_cycles).unwrap();
    println!("{cycle_result:?} \n");
    // We deliberately introduced a cycle during spawning!
    // 我们在生成实体时故意引入了一个循环！
    assert!(cycle_result.is_err());

    // Now, let's demonstrate removing relationships and break the cycle.
    // 现在，让我们展示如何移除关系并打破循环。
    fn untarget(mut commands: Commands, name_query: Query<(Entity, &Name)>) {
        // Let's find Charlie by doing a linear scan of the entity names.
        // 我们通过线性扫描实体名称来找到 Charlie。
        let charlie = name_query
           .iter()
           .find(|(_entity, name)| name.as_str() == "Charlie")
           .unwrap()
           .0;

        // We can remove the `Targeting` component to remove the relationship
        // 我们可以移除 `Targeting` 组件来移除关系
        // and break the cycle we saw earlier.
        // 并打破我们之前看到的循环。
        println!("Removing Charlie's targeting relationship.\n");
        commands.entity(charlie).remove::<Targeting>();
    }

    world.run_system_once(untarget).unwrap();
    world.run_system_once(debug_relationships).unwrap();
    // Cycle free!
    // 没有循环了！
    let cycle_result = world.run_system_once(check_for_cycles).unwrap();
    println!("{cycle_result:?} \n");
    assert!(cycle_result.is_ok());
}