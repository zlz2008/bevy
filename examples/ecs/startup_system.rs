//! Demonstrates a startup system (one that runs once when the app starts up).
//! 演示一个启动系统（应用程序启动时运行一次的系统）。

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, startup_system)
        .add_systems(Update, normal_system)
        .run();
}

/// Startup systems are run exactly once when the app starts up.
/// 启动系统在应用程序启动时恰好运行一次。
/// They run right before "normal" systems run.
/// 它们在“常规”系统运行之前立即运行。
fn startup_system() {
    println!("startup system ran first");
}

fn normal_system() {
    println!("normal system ran second");
}