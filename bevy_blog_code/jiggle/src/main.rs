use bevy::{
    prelude::*,
    color::palettes::css::*, 
    math::UVec2,
    render::camera::Viewport, 
    window::{PrimaryWindow, Window},
    input::mouse::{MouseButtonInput, MouseButton},
    ecs::query::WorldQuery,
};

#[derive(Component)]
pub struct Grid {
    enabled: bool,
    size: i32,
    cell_size: f32,
}

#[derive(Component)]
struct FullScreen {
    enabled: bool,
}

#[derive(Component)]
struct FlyCamera {
    yaw: f32,   // rotation around Y axis in radians
    pitch: f32, // rotation around X axis in radians
}

#[derive(Component)]
struct SphereTag;

#[derive(Component, Default)]
struct JiggleAnimation {
    active: bool,
    timer: f32,
}

#[derive(Component)]
struct MainCamera;

const JIGGLE_DURATION: f32 = 1.5; // seconds

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, grid)
        .add_systems(Update, fly_camera)
        .add_systems(Update, jiggle_sphere)
        .add_systems(Update, jiggle_on_click) // <-- Add this
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // main camera
    commands.spawn((
        Name::new("MainCamera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCamera { yaw: 0.0, pitch: 0.0 },
        MainCamera,
    ));

    // light source
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // grid entity
    commands.spawn(Grid {
        enabled: false,
        size: 10,
        cell_size: 1.0
    });

    // sphere
    commands.spawn((
        Name::new("JiggleSphere"),
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_u8(124, 144, 255),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        SphereTag,
        JiggleAnimation::default(),
    ));
}

// Draw grid and axes, toggle with Space
fn grid(
    mut gizmos: Gizmos,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut grid: Single<&mut Grid>,
) {
        // toggle grid visibility
    if keyboard_input.just_pressed(KeyCode::Space) {
        grid.enabled = !grid.enabled;
    }

    if grid.enabled {
        // grid lines
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
        // axes
        gizmos.line(Vec3::new(-100.0, 0.01, 0.0), Vec3::new(100.0, 0.0, 0.0), RED);
        gizmos.line(Vec3::new(0.0, -100.0, 0.0), Vec3::new(0.0, 100.0, 0.0), GREEN);
        gizmos.line(Vec3::new(0.0, 0.01, -100.0), Vec3::new(0.0, 0.0, 100.0), BLUE);
        }
}

// WASD + QE movement and arrow keys for camera rotation
fn fly_camera(
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 5.0;
    let rot_speed = 1.5; // radians/sec

    for (mut transform, mut camera) in &mut query {
        // spin on Y axis
        if keys.pressed(KeyCode::ArrowLeft) {
            camera.yaw += rot_speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::ArrowRight) {
            camera.yaw -= rot_speed * time.delta_secs();
        }
        // pitch up/down
        if keys.pressed(KeyCode::ArrowUp) {
            camera.pitch += rot_speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::ArrowDown) {
            camera.pitch -= rot_speed * time.delta_secs();
        }
        camera.pitch = camera.pitch.clamp(-1.54, 1.54); // clamp pitch to avoid flipping

        // apply yaw and pitch rotation to the camera
        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, camera.yaw) *
            Quat::from_axis_angle(Vec3::X, camera.pitch);

        // movement (WASD for horizontal, QE for vertical)
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

// Procedural jiggle animation for the sphere when A is pressed
fn jiggle_sphere(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut JiggleAnimation), With<SphereTag>>,
) {
    let jiggle_amplitude = 1.0; // Start amplitude (big jiggle)
    let jiggle_speed = 16.0;     // Fast jiggle

    for (mut transform, mut jiggle) in &mut query {
        // Start jiggle on B press (not while held)
        if keys.just_pressed(KeyCode::KeyB) {
            jiggle.active = true;
            jiggle.timer = 0.0;
        }

        if jiggle.active {
            jiggle.timer += time.delta_secs();
            // Decay amplitude over time (quadratic decay)
            let decay = ((JIGGLE_DURATION - jiggle.timer) / JIGGLE_DURATION).max(0.0);
            let amplitude = jiggle_amplitude * decay * decay;
            let offset = (jiggle.timer * jiggle_speed).sin() * amplitude;
            transform.translation.y = offset;

            // Stop jiggle after duration
            if jiggle.timer >= JIGGLE_DURATION {
                jiggle.active = false;
                transform.translation.y = 0.0;
            }
        } else {
            transform.translation.y = 0.0;
        }
    }
}

fn jiggle_on_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&GlobalTransform, &mut JiggleAnimation), With<SphereTag>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.get_single() else { return; };
        if let Some(cursor_pos) = window.cursor_position() {
            let Ok((camera, camera_transform)) = cameras.get_single() else { return; };
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                let ray_direction = ray.direction.as_vec3();
                for (sphere_transform, mut jiggle) in &mut query {
                    let center = sphere_transform.translation();
                    let radius = 0.5;
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
}