//! Illustrates how to make a single system from multiple functions running in sequence,
//! 演示了如何将多个函数按顺序组合成一个系统，
//! passing the output of the first into the input of the next.
//! 并将前一个函数的输出作为下一个函数的输入。

use bevy::prelude::*;
use std::num::ParseIntError;

use bevy::log::{debug, error, info, Level, LogPlugin};

fn main() {
    App::new()
        .insert_resource(Message("42".to_string()))
        .insert_resource(OptionalWarning(Err("Got to rusty?".to_string())))
        .add_plugins(LogPlugin {
            level: Level::TRACE,
            filter: "".to_string(),
            ..default()
        })
        .add_systems(
            Update,
            (
                parse_message_system.pipe(handler_system),
                data_pipe_system.map(|out| info!("{out}")),
                parse_message_system.map(|out| debug!("{out:?}")),
                warning_pipe_system.map(|out| {
                    if let Err(err) = out {
                        error!("{err}");
                    }
                }),
                parse_error_message_system.map(|out| {
                    if let Err(err) = out {
                        error!("{err}");
                    }
                }),
                parse_message_system.map(drop),
            ),
        )
        .run();
}

#[derive(Resource, Deref)]
struct Message(String);

#[derive(Resource, Deref)]
struct OptionalWarning(Result<(), String>);

// This system produces a Result<usize> output by trying to parse the Message resource.
// 该系统尝试解析 Message 资源，产生一个 Result<usize> 类型的输出。
fn parse_message_system(message: Res<Message>) -> Result<usize, ParseIntError> {
    message.parse::<usize>()
}

// This system produces a Result<()> output by trying to parse the Message resource.
// 该系统尝试解析 Message 资源，产生一个 Result<()> 类型的输出。
fn parse_error_message_system(message: Res<Message>) -> Result<(), ParseIntError> {
    message.parse::<usize>()?;
    Ok(())
}

// This system takes a Result<usize> input and either prints the parsed value or the error message
// 该系统接受一个 Result<usize> 类型的输入，根据结果打印解析后的值或错误信息。
// Try changing the Message resource to something that isn't an integer. You should see the error
// 尝试将 Message 资源的值改为非整数，你会看到错误信息被打印出来。
// message printed.
fn handler_system(In(result): In<Result<usize, ParseIntError>>) {
    match result {
        Ok(value) => println!("parsed message: {value}"),
        Err(err) => println!("encountered an error: {err:?}"),
    }
}

// This system produces a String output by trying to clone the String from the Message resource.
// 该系统尝试克隆 Message 资源中的字符串，产生一个 String 类型的输出。
fn data_pipe_system(message: Res<Message>) -> String {
    message.0.clone()
}

// This system produces a Result<String> output by trying to extract a String from the
// 该系统尝试从 OptionalWarning 资源中提取一个字符串，产生一个 Result<String> 类型的输出。
// OptionalWarning resource. Try changing the OptionalWarning resource to None. You should
// 尝试将 OptionalWarning 资源的值改为 None，你将看不到警告信息被打印出来。
// not see the warning message printed.
fn warning_pipe_system(message: Res<OptionalWarning>) -> Result<(), String> {
    message.0.clone()
}