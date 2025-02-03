//! This example demonstrates how to use run conditions to control when systems run.
//! 此示例展示了如何使用运行条件来控制系统的运行时机。

use bevy::prelude::*;

fn main() {
    println!();
    println!("For the first 2 seconds you will not be able to increment the counter");
    println!("在最初的 2 秒内，你无法增加计数器的值");
    println!("Once that time has passed you can press space, enter, left mouse, right mouse or touch the screen to increment the counter");
    println!("2 秒过后，你可以按下空格键、回车键、鼠标左键、鼠标右键或触摸屏幕来增加计数器的值");
    println!();

    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<InputCounter>()
        .add_systems(
            Update,
            (
                increment_input_counter
                    // The common_conditions module has a few useful run conditions
                    // common_conditions 模块提供了一些有用的运行条件
                    // for checking resources and states. These are included in the prelude.
                    // 用于检查资源和状态。这些条件包含在 prelude 中。
                    .run_if(resource_exists::<InputCounter>)
                    // `.or()` is a run condition combinator that only evaluates the second condition
                    // `.or()` 是一个运行条件组合器，只有当第一个条件返回 `false` 时，才会计算第二个条件
                    // if the first condition returns `false`. This behavior is known as "short-circuiting",
                    // 这种行为被称为“短路求值”，
                    // and is how the `||` operator works in Rust (as well as most C-family languages).
                    // 这与 Rust（以及大多数 C 系语言）中的 `||` 运算符的工作方式相同。
                    // In this case, the `has_user_input` run condition will be evaluated since the `Unused` resource has not been initialized.
                    // 在这种情况下，由于 `Unused` 资源尚未初始化，`has_user_input` 运行条件将被计算。
                    .run_if(resource_exists::<Unused>.or(
                        // This is a custom run condition, defined using a system that returns
                        // 这是一个自定义的运行条件，通过一个返回
                        // a `bool` and which has read-only `SystemParam`s.
                        // `bool` 类型且具有只读 `SystemParam` 的系统来定义。
                        // Only a single run condition must return `true` in order for the system to run.
                        // 只要有一个运行条件返回 `true`，系统就会运行。
                        has_user_input,
                    )),
                print_input_counter
                    // `.and()` is a run condition combinator that only evaluates the second condition
                    // `.and()` 是一个运行条件组合器，只有当第一个条件返回 `true` 时，才会计算第二个条件
                    // if the first condition returns `true`, analogous to the `&&` operator.
                    // 类似于 `&&` 运算符。
                    // In this case, the short-circuiting behavior prevents the second run condition from
                    // 在这种情况下，短路求值行为可以防止在 `InputCounter` 资源未初始化时，第二个运行条件引发恐慌。
                    // panicking if the `InputCounter` resource has not been initialized.
                    .run_if(resource_exists::<InputCounter>.and(
                        // This is a custom run condition in the form of a closure.
                        // 这是一个以闭包形式存在的自定义运行条件。
                        // This is useful for small, simple run conditions you don't need to reuse.
                        // 对于那些不需要复用的小而简单的运行条件，这种方式很有用。
                        // All the normal rules still apply: all parameters must be read only except for local parameters.
                        // 所有常规规则仍然适用：除了局部参数外，所有参数都必须是只读的。
                        |counter: Res<InputCounter>| counter.is_changed() && !counter.is_added(),
                    )),
                print_time_message
                    // This function returns a custom run condition, much like the common conditions module.
                    // 这个函数返回一个自定义的运行条件，类似于 common_conditions 模块中的条件。
                    // It will only return true once 2 seconds have passed.
                    // 只有在 2 秒过去后，它才会返回 true。
                    .run_if(time_passed(2.0))
                    // You can use the `not` condition from the common_conditions module
                    // 你可以使用 common_conditions 模块中的 `not` 条件
                    // to inverse a run condition. In this case it will return true if
                    // 来反转一个运行条件。在这种情况下，如果
                    // less than 2.5 seconds have elapsed since the app started.
                    // 自应用程序启动以来经过的时间少于 2.5 秒，它将返回 true。
                    .run_if(not(time_passed(2.5))),
            ),
        )
        .run();
}

#[derive(Resource, Default)]
struct InputCounter(usize);

#[derive(Resource)]
struct Unused;

/// Return true if any of the defined inputs were just pressed.
/// 如果任何定义的输入刚刚被按下，则返回 true。
///
/// This is a custom run condition, it can take any normal system parameters as long as
/// 这是一个自定义的运行条件，它可以接受任何常规的系统参数，只要
/// they are read only (except for local parameters which can be mutable).
/// 这些参数是只读的（局部参数可以是可变的）。
/// It returns a bool which determines if the system should run.
/// 它返回一个 bool 值，用于确定系统是否应该运行。
fn has_user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
        || keyboard_input.just_pressed(KeyCode::Enter)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || mouse_button_input.just_pressed(MouseButton::Right)
        || touch_input.any_just_pressed()
}

/// This is a function that returns a closure which can be used as a run condition.
/// 这是一个返回闭包的函数，该闭包可以用作运行条件。
///
/// This is useful because you can reuse the same run condition but with different variables.
/// 这很有用，因为你可以复用相同的运行条件，但使用不同的变量。
/// This is how the common conditions module works.
/// common_conditions 模块就是这样工作的。
fn time_passed(t: f32) -> impl FnMut(Local<f32>, Res<Time>) -> bool {
    move |mut timer: Local<f32>, time: Res<Time>| {
        // Tick the timer
        // 更新计时器
        *timer += time.delta_secs();
        // Return true if the timer has passed the time
        // 如果计时器超过了指定时间，则返回 true
        *timer >= t
    }
}

/// SYSTEM: Increment the input counter
/// 系统：增加输入计数器的值
/// Notice how we can take just the `ResMut` and not have to wrap
/// 注意，我们可以直接获取 `ResMut`，而不必将其包装在
/// it in an option in case it hasn't been initialized, this is because
/// 一个 Option 中以防它未被初始化，这是因为
/// it has a run condition that checks if the `InputCounter` resource exists
/// 它有一个运行条件来检查 `InputCounter` 资源是否存在
fn increment_input_counter(mut counter: ResMut<InputCounter>) {
    counter.0 += 1;
}

/// SYSTEM: Print the input counter
/// 系统：打印输入计数器的值
fn print_input_counter(counter: Res<InputCounter>) {
    println!("Input counter: {}", counter.0);
}

/// SYSTEM: Adds the input counter resource
/// 系统：打印时间消息
fn print_time_message() {
    println!("It has been more than 2 seconds since the program started and less than 2.5 seconds");
    println!("程序启动已超过 2 秒且不到 2.5 秒");
}