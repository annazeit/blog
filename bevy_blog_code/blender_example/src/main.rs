use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};
use bevy::input::mouse::MouseButton;

const GREY: Color = Color::srgb(0.5, 0.5, 0.5);
const RED: Color = Color::srgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);

const JIGGLE_DURATION: f32 = 1.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, fly_camera)
        .add_systems(Update, grid)
        //.add_systems(Update, spin_donut)
        //.add_systems(Update, donut_jiggle)
        .add_systems(Update, donut_flip)
        .add_systems(Update, update_donut_coords_text)
        .run();
}

#[derive(Component)]
struct FlyCamera {
    yaw: f32,
    pitch: f32,
}

#[derive(Component)]
pub struct Grid {
    enabled: bool,
    size: i32,
    cell_size: f32,
}

#[derive(Component)]
struct DonutRoot;

#[derive(Component, Default)]
struct JiggleAnimation {
    active: bool,
    timer: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        FlyCamera { yaw: 0.0, pitch: 0.0 },
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Donut GLB scene
    commands.spawn((
        SceneBundle {
            scene: bevy::prelude::SceneRoot(asset_server.load("Donut.glb#Scene0")),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        DonutRoot,
        JiggleAnimation::default(),
    ));

    // Grid entity
    commands.spawn(Grid {
        enabled: false,
        size: 10,
        cell_size: 1.0,
    });
}

fn grid(
    mut gizmos: Gizmos,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut grid: Single<&mut Grid>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        grid.enabled = !grid.enabled;
    }

    if grid.enabled {
        for i in -grid.size..=grid.size {
            let pos = i as f32 * grid.cell_size;
            gizmos.line(
                Vec3::new(pos, 0.0, -grid.size as f32),
                Vec3::new(pos, 0.0, grid.size as f32),
                GREY,
            );
            gizmos.line(
                Vec3::new(-grid.size as f32, 0.0, pos),
                Vec3::new(grid.size as f32, 0.0, pos),
                GREY,
            );
        }
        gizmos.line(Vec3::new(-100.0, 0.01, 0.0), Vec3::new(100.0, 0.0, 0.0), RED);
        gizmos.line(Vec3::new(0.0, -100.0, 0.0), Vec3::new(0.0, 100.0, 0.0), GREEN);
        gizmos.line(Vec3::new(0.0, 0.01, -100.0), Vec3::new(0.0, 0.0, 100.0), BLUE);
    }
}

fn fly_camera(
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 5.0;
    let rot_speed = 1.5;

    for (mut transform, mut camera) in &mut query {
        if keys.pressed(KeyCode::ArrowLeft) {
            camera.yaw += rot_speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::ArrowRight) {
            camera.yaw -= rot_speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::ArrowUp) {
            camera.pitch += rot_speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::ArrowDown) {
            camera.pitch -= rot_speed * time.delta_secs();
        }
        camera.pitch = camera.pitch.clamp(-1.54, 1.54);

        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, camera.yaw) *
            Quat::from_axis_angle(Vec3::X, camera.pitch);

        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction += *transform.forward() * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyS) {
            direction -= *transform.forward() * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyA) {
            direction -= *transform.right() * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += *transform.right() * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyQ) {
            direction += Vec3::Y * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyE) {
            direction -= Vec3::Y * time.delta_secs();
        }
        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * speed * time.delta_secs();
        }
    }
}

// Donut spins and hovers in place
fn spin_donut(
    mut donut: Single<&mut Transform, With<DonutRoot>>,
    time: Res<Time>,
) {
    let hover_speed = 3.0;
    let hover_height = 0.3;
    let base_height = 1.0;
    donut.translation.y = base_height + hover_height * (hover_speed * time.elapsed_secs()).sin();
    donut.rotate_y(1.0 * time.delta_secs());
}

fn donut_jiggle(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&GlobalTransform, &mut Transform, &mut JiggleAnimation), With<DonutRoot>>,
) {
    // click to trigger jiggle
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.get_single() else { return; };
        if let Some(cursor_pos) = window.cursor_position() {
            let Ok((camera, camera_transform)) = cameras.get_single() else { return; };
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                let ray_direction = ray.direction.as_vec3();
                for (donut_transform, _, mut jiggle) in &mut query {
                    let center = donut_transform.translation();
                    let radius = 1.0;
                    let origin_to_center = center - ray.origin;
                    let tca = origin_to_center.dot(ray_direction);
                    let d2 = origin_to_center.length_squared() - tca * tca;
                    if d2 <= radius * radius {
                        jiggle.active = true;
                        jiggle.timer = 0.0;
                    }
                }
            }
        }
    }

    // animate jiggle
    let jiggle_amplitude = 1.0;
    let jiggle_speed = 16.0;
    for (_, mut transform, mut jiggle) in &mut query {
        if jiggle.active {
            jiggle.timer += time.delta_secs();
            let decay = ((JIGGLE_DURATION - jiggle.timer) / JIGGLE_DURATION).max(0.0);
            let amplitude = jiggle_amplitude * decay * decay;
            let offset = (jiggle.timer * jiggle_speed).sin() * amplitude;
            transform.translation.y = 1.0 + offset;
            if jiggle.timer >= JIGGLE_DURATION {
                jiggle.active = false;
                transform.translation.y = 1.0;
            }
        }
    }
}

fn update_donut_coords_text(
    donut_query: Query<&Transform, With<DonutRoot>>,
    grid_query: Query<&Grid>,
) {
    let show = grid_query.get_single().map_or(false, |g| g.enabled);
    if show {
        if let Ok(donut_transform) = donut_query.get_single() {
            let pos = donut_transform.translation;
            println!("Donut: ({:.2}, {:.2}, {:.2})", pos.x, pos.y, pos.z);
        }
    }
}
fn donut_flip(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&GlobalTransform, &mut Transform, &mut JiggleAnimation), With<DonutRoot>>,
) {
    // On click, check if donut was clicked and trigger flip
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.get_single() else { return; };
        if let Some(cursor_pos) = window.cursor_position() {
            let Ok((camera, camera_transform)) = cameras.get_single() else { return; };
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                let ray_direction = ray.direction.as_vec3();
                for (donut_transform, _, mut anim) in &mut query {
                    let center = donut_transform.translation();
                    let radius = 1.0;
                    let origin_to_center = center - ray.origin;
                    let tca = origin_to_center.dot(ray_direction);
                    let d2 = origin_to_center.length_squared() - tca * tca;
                    if d2 <= radius * radius && !anim.active {
                        anim.active = true;
                        anim.timer = 0.0;
                    }
                }
            }
        }
    }

    // Animate flip if active
    let flip_duration = 1.2;
    let jump_height = 3.0;
    let hover_time = 0.25;

    for (_, mut transform, mut anim) in &mut query {
        if anim.active {
            anim.timer += time.delta_secs();

            let t = anim.timer;
            let up_time = (flip_duration - hover_time) / 2.0;
            let down_time = up_time;
            let total_time = up_time + hover_time + down_time;

            // Calculate vertical position (parabolic jump with hover at top)
            let y = if t < up_time {
                let progress = t / up_time;
                1.0 + jump_height * (progress * std::f32::consts::PI / 2.0).sin()
            } else if t < up_time + hover_time {
                1.0 + jump_height
            } else if t < total_time {
                let progress = (t - up_time - hover_time) / down_time;
                1.0 + jump_height * (1.0 - (progress * std::f32::consts::PI / 2.0).sin())
            } else {
                anim.active = false;
                anim.timer = 0.0;
                1.0
            };

            // Calculate rotation (360 flip over the duration)
            let rotation = if t < total_time {
                Quat::from_rotation_x(std::f32::consts::TAU * (t / total_time))
            } else {
                Quat::IDENTITY
            };

            transform.translation.y = y;
            transform.rotation = rotation;
        }
    }
}