//! Demonstrate stepping through systems in order of execution.
//! 演示按执行顺序逐步执行系统。
//!
//! To run this example, you must enable the `bevy_debug_stepping` feature.
//! 要运行此示例，你必须启用 `bevy_debug_stepping` 功能。

use bevy::{ecs::schedule::Stepping, log::LogPlugin, prelude::*};

fn main() {
    let mut app = App::new();

    app
        // to display log messages from Stepping resource
        // 用于显示来自 Stepping 资源的日志消息
        .add_plugins(LogPlugin::default())
        .add_systems(
            Update,
            (
                update_system_one,
                // establish a dependency here to simplify descriptions below
                // 在此处建立依赖关系以简化下面的描述
                update_system_two.after(update_system_one),
                update_system_three.after(update_system_two),
                update_system_four,
            ),
        )
        .add_systems(PreUpdate, pre_update_system);

    // For the simplicity of this example, we directly modify the `Stepping`
    // 为了简化此示例，我们直接修改 `Stepping`
    // resource here and run the systems with `App::update()`.  Each call to
    // 资源并使用 `App::update()` 运行系统。每次调用
    // `App::update()` is the equivalent of a single frame render when using
    // `App::update()` 相当于使用 `App::run()` 时的一帧渲染。
    // `App::run()`.
    //
    // In a real-world situation, the `Stepping` resource would be modified by
    // 在实际情况中，`Stepping` 资源将由一个系统根据用户输入进行修改。
    // a system based on input from the user.  A full demonstration of this can
    // 完整的演示可以在 breakout 示例中找到。
    // be found in the breakout example.
    println!(
        r#"
    Actions: call app.update()
     Result: All systems run normally"#
    );
    // 操作：调用 app.update()
    // 结果：所有系统正常运行
    app.update();

    println!(
        r#"
    Actions: Add the Stepping resource then call app.update()
     Result: All systems run normally.  Stepping has no effect unless explicitly
             configured for a Schedule, and Stepping has been enabled."#
    );
    // 操作：添加 Stepping 资源，然后调用 app.update()
    // 结果：所有系统正常运行。除非为某个调度表显式配置了 Stepping 并启用它，否则 Stepping 不会产生任何影响。
    app.insert_resource(Stepping::new());
    app.update();

    println!(
        r#"
    Actions: Add the Update Schedule to Stepping; enable Stepping; call
             app.update()
     Result: Only the systems in PreUpdate run.  When Stepping is enabled,
             systems in the configured schedules will not run unless:
             * Stepping::step_frame() is called
             * Stepping::continue_frame() is called
             * System has been configured to always run"#
    );
    // 操作：将 Update 调度表添加到 Stepping；启用 Stepping；调用 app.update()
    // 结果：只有 PreUpdate 中的系统会运行。当 Stepping 启用时，配置的调度表中的系统不会运行，除非：
    // * 调用了 Stepping::step_frame()
    // * 调用了 Stepping::continue_frame()
    // * 系统已配置为始终运行
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.add_schedule(Update).enable();
    app.update();

    println!(
        r#"
    Actions: call Stepping.step_frame(); call app.update()
     Result: The PreUpdate systems run, and one Update system will run.  In
             Stepping, step means run the next system across all the schedules 
             that have been added to the Stepping resource."#
    );
    // 操作：调用 Stepping.step_frame()；调用 app.update()
    // 结果：PreUpdate 系统运行，并且一个 Update 系统将运行。在 Stepping 中，step 意味着在添加到 Stepping 资源的所有调度表中运行下一个系统。
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.step_frame();
    app.update();

    println!(
        r#"
    Actions: call app.update()
     Result: Only the PreUpdate systems run.  The previous call to
             Stepping::step_frame() only applies for the next call to
             app.update()/the next frame rendered.
    "#
    );
    // 操作：调用 app.update()
    // 结果：只有 PreUpdate 系统运行。之前对 Stepping::step_frame() 的调用仅适用于下一次调用 app.update()/下一帧渲染。
    app.update();

    println!(
        r#"
    Actions: call Stepping::continue_frame(); call app.update()
     Result: PreUpdate system will run, and all remaining Update systems will
             run.  Stepping::continue_frame() tells stepping to run all systems
             starting after the last run system until it hits the end of the
             frame, or it encounters a system with a breakpoint set.  In this
             case, we previously performed a step, running one system in Update.
             This continue will cause all remaining systems in Update to run."#
    );
    // 操作：调用 Stepping::continue_frame()；调用 app.update()
    // 结果：PreUpdate 系统将运行，并且所有剩余的 Update 系统将运行。Stepping::continue_frame() 告诉 Stepping 从最后一个运行的系统之后开始运行所有系统，直到到达帧的末尾，或者遇到设置了断点的系统。在这种情况下，我们之前执行了一步，运行了一个 Update 系统。这次继续将导致 Update 中所有剩余的系统运行。
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.continue_frame();
    app.update();

    println!(
        r#"
    Actions: call Stepping::step_frame() & app.update() four times in a row
     Result: PreUpdate system runs every time we call app.update(), along with
             one system from the Update schedule each time.  This shows what
             execution would look like to step through an entire frame of 
             systems."#
    );
    // 操作：连续四次调用 Stepping::step_frame() 和 app.update()
    // 结果：每次调用 app.update() 时，PreUpdate 系统都会运行，并且每次都会运行一个 Update 调度表中的系统。这展示了逐步执行一帧系统的执行情况。
    for _ in 0..4 {
        let mut stepping = app.world_mut().resource_mut::<Stepping>();
        stepping.step_frame();
        app.update();
    }

    println!(
        r#"
    Actions: Stepping::always_run(Update, update_system_two); step through all
             systems
     Result: PreUpdate system and update_system_two() will run every time we
             call app.update().  We'll also only need to step three times to
             execute all systems in the frame.  Stepping::always_run() allows
             us to granularly allow systems to run when stepping is enabled."#
    );
    // 操作：调用 Stepping::always_run(Update, update_system_two)；逐步执行所有系统
    // 结果：每次调用 app.update() 时，PreUpdate 系统和 update_system_two() 都会运行。我们也只需要执行三步就能执行帧中的所有系统。Stepping::always_run() 允许我们在启用 Stepping 时精细地允许系统运行。
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.always_run(Update, update_system_two);
    for _ in 0..3 {
        let mut stepping = app.world_mut().resource_mut::<Stepping>();
        stepping.step_frame();
        app.update();
    }

    println!(
        r#"
    Actions: Stepping::never_run(Update, update_system_two); continue through
             all systems
     Result: All systems except update_system_two() will execute.
             Stepping::never_run() allows us to disable systems while Stepping
             is enabled."#
    );
    // 操作：调用 Stepping::never_run(Update, update_system_two)；继续执行所有系统
    // 结果：除了 update_system_two() 之外的所有系统都将执行。Stepping::never_run() 允许我们在启用 Stepping 时禁用系统。
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.never_run(Update, update_system_two);
    stepping.continue_frame();
    app.update();

    println!(
        r#"
    Actions: Stepping::set_breakpoint(Update, update_system_two); continue,
             step, continue
     Result: During the first continue, pre_update_system() and
             update_system_one() will run.  update_system_four() may also run
             as it has no dependency on update_system_two() or
             update_system_three().  Nether update_system_two() nor
             update_system_three() will run in the first app.update() call as
             they form a chained dependency on update_system_one() and run
             in order of one, two, three.  Stepping stops system execution in
             the Update schedule when it encounters the breakpoint for
             update_system_three().
             During the step we run update_system_two() along with the
             pre_update_system().
             During the final continue pre_update_system() and
             update_system_three() run."#
    );
    // 操作：调用 Stepping::set_breakpoint(Update, update_system_two)；继续，单步执行，继续
    // 结果：在第一次继续时，pre_update_system() 和 update_system_one() 将运行。update_system_four() 也可能运行，因为它不依赖于 update_system_two() 或 update_system_three()。在第一次调用 app.update() 时，update_system_two() 和 update_system_three() 都不会运行，因为它们对 update_system_one() 形成了链式依赖，并且按一、二、三的顺序运行。当 Stepping 在 Update 调度表中遇到 update_system_three() 的断点时，会停止系统执行。
    // 在单步执行时，我们会运行 update_system_two() 以及 pre_update_system()。
    // 在最后一次继续时，pre_update_system() 和 update_system_three() 会运行。
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.set_breakpoint(Update, update_system_two);
    stepping.continue_frame();
    app.update();
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.step_frame();
    app.update();
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.continue_frame();
    app.update();

    println!(
        r#"
    Actions: Stepping::clear_breakpoint(Update, update_system_two); continue
             through all systems
     Result: All systems will run"#
    );
    // 操作：调用 Stepping::clear_breakpoint(Update, update_system_two)；继续执行所有系统
    // 结果：所有系统都将运行
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.clear_breakpoint(Update, update_system_two);
    stepping.continue_frame();
    app.update();

    println!(
        r#"
    Actions: Stepping::disable(); app.update()
     Result: All systems will run.  With Stepping disabled, there's no need to
             call Stepping::step_frame() or Stepping::continue_frame() to run
             systems in the Update schedule."#
    );
    // 操作：调用 Stepping::disable()；调用 app.update()
    // 结果：所有系统都将运行。禁用 Stepping 后，无需调用 Stepping::step_frame() 或 Stepping::continue_frame() 来运行 Update 调度表中的系统。
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.disable();
    app.update();
}

fn pre_update_system() {
    println!("▶ pre_update_system");
}
fn update_system_one() {
    println!("▶ update_system_one");
}
fn update_system_two() {
    println!("▶ update_system_two");
}
fn update_system_three() {
    println!("▶ update_system_three");
}
fn update_system_four() {
    println!("▶ update_system_four");
}