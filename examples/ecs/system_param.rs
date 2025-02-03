//! This example creates a custom [`SystemParam`] struct that counts the number of players.
//! 此示例创建了一个自定义的 [`SystemParam`] 结构体，用于统计玩家的数量。

use bevy::{ecs::system::SystemParam, prelude::*};

fn main() {
    App::new()
        .insert_resource(PlayerCount(0))
        .add_systems(Startup, spawn)
        .add_systems(Update, count_players)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerCount(usize);

/// The [`SystemParam`] struct can contain any types that can also be included in a
/// [`SystemParam`] 结构体可以包含任何能够在系统函数签名中出现的类型。
/// system function signature.
///
/// In this example, it includes a query and a mutable resource.
/// 在这个示例中，它包含一个查询和一个可变资源。
#[derive(SystemParam)]
struct PlayerCounter<'w, 's> {
    players: Query<'w, 's, &'static Player>,
    count: ResMut<'w, PlayerCount>,
}

impl<'w, 's> PlayerCounter<'w, 's> {
    fn count(&mut self) {
        self.count.0 = self.players.iter().len();
    }
}

/// Spawn some players to count
/// 生成一些玩家以供统计
fn spawn(mut commands: Commands) {
    commands.spawn(Player);
    commands.spawn(Player);
    commands.spawn(Player);
}

/// The [`SystemParam`] can be used directly in a system argument.
/// [`SystemParam`] 可以直接在系统参数中使用。
fn count_players(mut counter: PlayerCounter) {
    counter.count();

    println!("{} players in the game", counter.count.0);
}