//! Shows how anonymous functions / closures can be used as systems.
//! 展示了如何将匿名函数/闭包用作系统。

use bevy::{log::LogPlugin, prelude::*};

fn main() {
    // create a simple closure.
    // 创建一个简单的闭包。
    let simple_closure = || {
        // this is a closure that does nothing.
        // 这是一个什么都不做的闭包。
        info!("Hello from a simple closure!");
    };

    // create a closure, with an 'input' value.
    // 创建一个带有“输入”值的闭包。
    let complex_closure = |mut value: String| {
        move || {
            info!("Hello from a complex closure! {}", value);

            // we can modify the value inside the closure. this will be saved between calls.
            // 我们可以在闭包内部修改这个值。该值在多次调用之间会被保存。
            value = format!("{value} - updated");

            // you could also use an outside variable like presented in the inlined closures
            // 你也可以像内联闭包中那样使用外部变量
            // info!("outside_variable! {}", outside_variable);
        }
    };

    let outside_variable = "bar".to_string();

    App::new()
        .add_plugins(LogPlugin::default())
        // we can use a closure as a system
        // 我们可以将闭包用作一个系统
        .add_systems(Update, simple_closure)
        // or we can use a more complex closure, and pass an argument to initialize a Local variable.
        // 或者我们可以使用一个更复杂的闭包，并传递一个参数来初始化一个局部变量。
        .add_systems(Update, complex_closure("foo".into()))
        // we can also inline a closure
        // 我们也可以内联一个闭包
        .add_systems(Update, || {
            info!("Hello from an inlined closure!");
        })
        // or use variables outside a closure
        // 或者使用闭包外部的变量
        .add_systems(Update, move || {
            info!(
                "Hello from an inlined closure that captured the 'outside_variable'! {}",
                outside_variable
            );
            // you can use outside_variable, or any other variables inside this closure.
            // 你可以在这个闭包内部使用 outside_variable 或任何其他变量。
            // their states will be saved.
            // 它们的状态会被保存。
        })
        .run();
}