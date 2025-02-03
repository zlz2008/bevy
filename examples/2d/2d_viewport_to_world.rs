//! This example demonstrates how to use the `Camera::viewport_to_world_2d` method.
//! 此示例演示了如何使用 `Camera::viewport_to_world_2d` 方法。

use bevy::{color::palettes::basic::WHITE, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    // 根据光标的位置计算世界坐标。
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., WHITE);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Create a minimal UI explaining how to interact with the example
    // 创建一个简单的 UI，解释如何与示例交互。
    commands.spawn((
        Text::new("Move the mouse to see the circle follow your cursor."),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}