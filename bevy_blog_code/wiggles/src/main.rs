use bevy::prelude::*;
use std::f32;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Gizmos, Mut, Query, Res, Time};
use bevy::window::{WindowPlugin, Window};


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
        .add_systems(Startup, setup)
        .add_plugins(SpritePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, sprite_start);
        app.add_systems(Update, sprite_movement);
    }
}

#[derive(Component)]
pub struct Sprite {
    position: Vec2,
    // direction: Direction, // Remove if unused
    circle_angle: f32,
}

fn sprite_start(mut commands: Commands) {
    for _ in 0..1 {
        commands.spawn(Sprite {
            position: Vec2::new(0.0, 0.0),
            // direction: Direction::Up, // Remove if unused
            circle_angle: 0.0,
        });
    }
}

fn sprite_update(
    sprite: &mut Mut<Sprite>,
    time: &Res<Time>
){
    // Parameters for the circular path
    let circle_radius = 150.0;
    let circle_speed = 0.5; // radians per second

    // Update the angle
    sprite.circle_angle += circle_speed * time.delta_secs();

    // Keep angle in [0, 2PI]
    if sprite.circle_angle > std::f32::consts::TAU {
        sprite.circle_angle -= std::f32::consts::TAU;
    }

    // Calculate new base position on the circle
    sprite.position = Vec2::new(
        circle_radius * sprite.circle_angle.cos(),
        circle_radius * sprite.circle_angle.sin(),
    );
}

fn sprite_animate(
    sprite: &mut Mut<Sprite>,
    time: &Res<Time>,
    gizmos: &mut Gizmos
) {
    let steps = 100;
    for i in 0..steps {

        // Wiggle offset
        let wiggle_speed = 60.0;
        let radian_in_sec = 2.0 * f32::consts::PI / 60.0;
        let time_angle = time.elapsed_secs() * radian_in_sec * wiggle_speed;
        let step_angle = 2.0 * f32::consts::PI * (i as f32 / steps as f32);
        let seconds_cycle = f32::sin(time_angle + step_angle);
        let wave_amplitude = 20.0;
        let x_wiggle = wave_amplitude * seconds_cycle;

        // The worm's body follows the circle, but each segment is offset along the tangent
        let angle = sprite.circle_angle + (i as f32 / steps as f32) * 0.5; // spread segments along the circle
        let base = Vec2::new(
            150.0 * angle.cos(),
            150.0 * angle.sin(),
        );
        // Tangent vector (perpendicular to radius)
        let tangent = Vec2::new(-angle.sin(), angle.cos());

        let position = base + tangent * x_wiggle;

        let radius_factor = f32::sin(2.0 * f32::consts::PI * (i as f32 / steps as f32));
        let radius = (radius_factor) * 30.0 + 10.0;

        let color = Color::hsl(360. * i as f32 / steps as f32, 0.95, 0.7);
        gizmos.circle_2d(position, radius, color);
    }
}

// The sprite is animated by changing its translation depending on the time that has passed since the last frame.
pub fn sprite_movement(
    time: Res<Time>,
    mut sprite_query: Query<&mut Sprite>,
    mut gizmos: Gizmos,
) {
    for mut sprite in &mut sprite_query {
        sprite_update(&mut sprite, &time);
        sprite_animate(&mut sprite, &time, &mut gizmos);
    }
}
