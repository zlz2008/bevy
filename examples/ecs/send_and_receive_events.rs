//! From time to time, you may find that you want to both send and receive an event of the same type in a single system.
//! 时不时地，你可能会发现自己想在单个系统中同时发送和接收同一类型的事件。
//!
//! Of course, this results in an error: the borrows of [`EventWriter`] and [`EventReader`] overlap,
//! 当然，这会导致一个错误：[`EventWriter`] 和 [`EventReader`] 的借用发生了重叠，
//! if and only if the [`Event`] type is the same.
//! 当且仅当 [`Event`] 类型相同时会出现此问题。
//! One system parameter borrows the [`Events`] resource mutably, and another system parameter borrows the [`Events`] resource immutably.
//! 一个系统参数可变地借用了 [`Events`] 资源，而另一个系统参数不可变地借用了 [`Events`] 资源。
//! If Bevy allowed this, this would violate Rust's rules against aliased mutability.
//! 如果 Bevy 允许这样做，就会违反 Rust 关于避免别名可变引用的规则。
//! In other words, this would be Undefined Behavior (UB)!
//! 换句话说，这将是未定义行为（UB）！
//!
//! There are two ways to solve this problem:
//! 有两种方法可以解决这个问题：
//!
//! 1. Use [`ParamSet`] to check out the [`EventWriter`] and [`EventReader`] one at a time.
//! 1. 使用 [`ParamSet`] 一次只使用一个 [`EventWriter`] 或 [`EventReader`]。
//! 2. Use a [`Local`] [`EventCursor`] instead of an [`EventReader`], and use [`ResMut`] to access [`Events`].
//! 2. 使用 [`Local`] [`EventCursor`] 代替 [`EventReader`]，并使用 [`ResMut`] 来访问 [`Events`]。
//!
//! In the first case, you're being careful to only check out only one of the [`EventWriter`] or [`EventReader`] at a time.
//! 在第一种情况下，你要小心地一次只使用一个 [`EventWriter`] 或 [`EventReader`]。
//! By "temporally" separating them, you avoid the overlap.
//! 通过“时间上”分离它们，你可以避免借用重叠。
//!
//! In the second case, you only ever have one access to the underlying  [`Events`] resource at a time.
//! 在第二种情况下，你一次只能对底层的 [`Events`] 资源进行一次访问。
//! But in exchange, you have to manually keep track of which events you've already read.
//! 但作为交换，你必须手动记录哪些事件你已经读过了。
//!
//! Let's look at an example of each.
//! 让我们来看一下每种方法的示例。

use bevy::{diagnostic::FrameCount, ecs::event::EventCursor, prelude::*};

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_event::<DebugEvent>()
        .add_event::<A>()
        .add_event::<B>()
        .add_systems(Update, read_and_write_different_event_types)
        .add_systems(
            Update,
            (
                send_events,
                debug_events,
                send_and_receive_param_set,
                debug_events,
                send_and_receive_manual_event_reader,
                debug_events,
            )
                .chain(),
        );
    // We're just going to run a few frames, so we can see and understand the output.
    // 我们只运行几帧，这样我们就可以看到并理解输出结果。
    app.update();
    // By running for longer than one frame, we can see that we're caching our cursor in the event queue properly.
    // 通过运行超过一帧，我们可以看到我们在事件队列中正确地缓存了游标。
    app.update();
}

#[derive(Event)]
struct A;

#[derive(Event)]
struct B;

// This works fine, because the types are different,
// 这可以正常工作，因为类型不同，
// so the borrows of the `EventWriter` and `EventReader` don't overlap.
// 所以 `EventWriter` 和 `EventReader` 的借用不会重叠。
// Note that these borrowing rules are checked at system initialization time,
// 注意，这些借用规则是在系统初始化时检查的，
// not at compile time, as Bevy uses internal unsafe code to split the `World` into disjoint pieces.
// 而不是在编译时，因为 Bevy 使用内部的不安全代码将 `World` 分割成不相交的部分。
fn read_and_write_different_event_types(mut a: EventWriter<A>, mut b: EventReader<B>) {
    for _ in b.read() {}
    a.send(A);
}

/// A dummy event type.
/// 一个虚拟的事件类型。
#[derive(Debug, Clone, Event)]
struct DebugEvent {
    resend_from_param_set: bool,
    resend_from_local_event_reader: bool,
    times_sent: u8,
}

/// A system that sends all combinations of events.
/// 一个发送所有事件组合的系统。
fn send_events(mut events: EventWriter<DebugEvent>, frame_count: Res<FrameCount>) {
    println!("Sending events for frame {}", frame_count.0);

    events.send(DebugEvent {
        resend_from_param_set: false,
        resend_from_local_event_reader: false,
        times_sent: 1,
    });
    events.send(DebugEvent {
        resend_from_param_set: true,
        resend_from_local_event_reader: false,
        times_sent: 1,
    });
    events.send(DebugEvent {
        resend_from_param_set: false,
        resend_from_local_event_reader: true,
        times_sent: 1,
    });
    events.send(DebugEvent {
        resend_from_param_set: true,
        resend_from_local_event_reader: true,
        times_sent: 1,
    });
}

/// A system that prints all events sent since the last time this system ran.
/// 一个打印自上次该系统运行以来发送的所有事件的系统。
///
/// Note that some events will be printed twice, because they were sent twice.
/// 注意，有些事件会被打印两次，因为它们被发送了两次。
fn debug_events(mut events: EventReader<DebugEvent>) {
    for event in events.read() {
        println!("{event:?}");
    }
}

/// A system that both sends and receives events using [`ParamSet`].
/// 一个使用 [`ParamSet`] 同时发送和接收事件的系统。
fn send_and_receive_param_set(
    mut param_set: ParamSet<(EventReader<DebugEvent>, EventWriter<DebugEvent>)>,
    frame_count: Res<FrameCount>,
) {
    println!(
        "Sending and receiving events for frame {} with a `ParamSet`",
        frame_count.0
    );

    // We must collect the events to resend, because we can't access the writer while we're iterating over the reader.
    // 我们必须收集要重新发送的事件，因为在遍历读取器时我们不能访问写入器。
    let mut events_to_resend = Vec::new();

    // This is p0, as the first parameter in the `ParamSet` is the reader.
    // 这是 p0，因为 `ParamSet` 中的第一个参数是读取器。
    for event in param_set.p0().read() {
        if event.resend_from_param_set {
            events_to_resend.push(event.clone());
        }
    }

    // This is p1, as the second parameter in the `ParamSet` is the writer.
    // 这是 p1，因为 `ParamSet` 中的第二个参数是写入器。
    for mut event in events_to_resend {
        event.times_sent += 1;
        param_set.p1().send(event);
    }
}

/// A system that both sends and receives events using a [`Local`] [`EventCursor`].
/// 一个使用 [`Local`] [`EventCursor`] 同时发送和接收事件的系统。
fn send_and_receive_manual_event_reader(
    // The `Local` `SystemParam` stores state inside the system itself, rather than in the world.
    // `Local` 系统参数将状态存储在系统本身内部，而不是存储在世界中。
    // `EventCursor<T>` is the internal state of `EventReader<T>`, which tracks which events have been seen.
    // `EventCursor<T>` 是 `EventReader<T>` 的内部状态，用于跟踪哪些事件已经被处理过。
    mut local_event_reader: Local<EventCursor<DebugEvent>>,
    // We can access the `Events` resource mutably, allowing us to both read and write its contents.
    // 我们可以可变地访问 `Events` 资源，这样我们就可以读写其内容。
    mut events: ResMut<Events<DebugEvent>>,
    frame_count: Res<FrameCount>,
) {
    println!(
        "Sending and receiving events for frame {} with a `Local<EventCursor>",
        frame_count.0
    );

    // We must collect the events to resend, because we can't mutate events while we're iterating over the events.
    // 我们必须收集要重新发送的事件，因为在遍历事件时我们不能修改事件。
    let mut events_to_resend = Vec::new();

    for event in local_event_reader.read(&events) {
        if event.resend_from_local_event_reader {
            // For simplicity, we're cloning the event.
            // 为了简单起见，我们克隆事件。
            // In this case, since we have mutable access to the `Events` resource,
            // 在这种情况下，由于我们可以可变地访问 `Events` 资源，
            // we could also just mutate the event in-place,
            // 我们也可以直接就地修改事件，
            // or drain the event queue into our `events_to_resend` vector.
            // 或者将事件队列中的元素转移到我们的 `events_to_resend` 向量中。
            events_to_resend.push(event.clone());
        }
    }

    for mut event in events_to_resend {
        event.times_sent += 1;
        events.send(event);
    }
}