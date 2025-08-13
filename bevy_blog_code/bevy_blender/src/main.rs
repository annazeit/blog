use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};
use bevy::input::mouse::MouseButton;

#[derive(Component)]
struct DonutTag;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, check_donut_click)
        .run();
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        SceneBundle {
            scene: bevy::prelude::SceneRoot(asset_server.load("Donut.glb#Scene0")),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        DonutTag,
    ));
}

fn check_donut_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.get_single() else { return; };
        let Some(cursor_pos) = window.cursor_position() else { return; };
        let Ok((camera, camera_transform)) = cameras.get_single() else { return; };
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else { return; };
        let donut_center = Vec3::ZERO; // assuming donut at origin
        let radius = 1.0; // adjust as needed
        let origin_to_center = donut_center - ray.origin;
        let tca = origin_to_center.dot(ray.direction.as_vec3());
        let d2 = origin_to_center.length_squared() - tca * tca;
        if d2 <= radius * radius {
            println!("Donut clicked!");
        }
    }
}