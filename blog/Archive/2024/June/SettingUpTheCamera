# Setting Up The Camera

First thing that we want to do is set up a camera for our little game.

Open your `main.rs` file and put in this code.

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)

        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
```

- We start off by importing the `bevy::prelude` which includes commonly used types, traits, and functions from the Bevy game engine.

- Then we create a new bevy application using `App::new()` and add the default plugins. After that, we add the setup system so it can run the startup stage of the application.

- We then spawn the camera in the `setup()` function.

After putting in the code and running the program, you should see an empty window appear. That is the camera view. 