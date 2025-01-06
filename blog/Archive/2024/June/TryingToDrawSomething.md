# Drawing Something
Now that we've set up our camera, it's time to put something on the canvas.

```rust
use bevy::prelude::*;
use bevy::color::palettes::basic::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_circle)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_circle(mut gizmos: Gizmos,) {
    gizmos.circle_2d(Vec2::new(0.0, 0.0), 100.0, RED);
}
```

- We add the `draw_circle()` function to be run during the update stage. And draw a simple circle using `gizmos`. Our circle can then potentially become the player.
