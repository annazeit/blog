# Adding Player Component and Controls

## 1. Create the player component

The circle will become the player, so we need to create a `Player` struct for it with parameters like position and color. 

- The `Player` struct is a component that can be attached to entities in Bevy's Entity-Component-System.

- We also spawn a player entity in the `setup()` function just as we did with the camera.

Here is the new and improved code:

**main.rs**

```rust
use bevy::prelude::*; // Includes commonly used types, traits, and functions from the Bevy game engine.
use bevy::color::palettes::basic::*;

#[derive(Component)] // Marks the Player struct as a component.
struct Player {
    position: Vec2,
    color: Srgba,
} 
fn main() {
    App::new() // Creates a new Bevy application.
    
        .add_plugins(DefaultPlugins) 

        // Adds the setup system to the Startup stage, which runs once at the beginning.
        .add_systems(Startup, setup) 

        //Adds the draw_player system to the Update stage, which runs every frame.
        .add_systems(Update, draw_player) 

         // Runs the application.
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()); //Spawns a 2D camera entity.

    commands.spawn(Player { //Spawns a Player entity with these parameters.
        position: Vec2::new(0.0, 0.0),
        color: RED,
    });
}

fn draw_player(
    mut gizmos: Gizmos,
    mut player_query: Query<&mut Player>, 
) {
    let size_radius = 20.0;
    for player in &mut player_query {
        // Draw a circle at the player's position.
        gizmos.circle_2d(player.position, size_radius, player.color); 
    }    
}
```
- Althought there is a little more code, it still does the same thing as the previous code.

## 2. Up and Down Controls

Lets add simple controls to the player so that it is able to move up and down.

- We will use the up and down arrow keys to move the player.

- Lets adjust the `draw_player()` function and add a keyboard input.

- We add two simple `if` statements. 

Now the `draw_player()` function should look something like this:

```rust
fn draw_player(
    mut gizmos: Gizmos,
    mut player_query: Query<&mut Player>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let size_radius = 20.0;
    for mut player in &mut player_query {
        bvfnmhj;'

        // Draws a circle at the player's position.
        gizmos.circle_2d(player.position, size_radius, player.color); 

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player.position.y += 1.0; // Moves the player up.
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            player.position.y -= 1.0; // Moves the player down.
        }
    }    
}
```
- Run your code and try pressing the up and down arrow buttons. Your player should move up and down.

## 3. Adding Rotation 

Now lets make the controls a bit more advanced and let the player turn and rotate.

- We will add another parameter to the Player struct called `direction_angle`.

- Its the angle (in radians) that the player is facing. This angle is used to determine the direction in which the player moves when the forward or backward keys are pressed. 

- To do that we need to edit the struct and the setup function.

```rust
#[derive(Component)] // Marks the Player struct as a component that can be attached to entities in Bevy's Entity-Component-System.
struct Player {
    position: Vec2,
    directiom_angle: f32,
    color: Srgba,
} 
```
```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()); //Spawns a 2D camera entity.

    commands.spawn(Player { //Spawns a Player entity with these parameters.
        position: Vec2::new(0.0, 0.0),
        directiom_angle: 0.0,
        color: RED,
    });
}
```
- Next, we need to edit the `draw_player` function. The `direction_angle` of the player will change when the left or right arrow key is pressed.

- Usually, angles are measured from left to right, so we will make it so that the right arrow increases the angle and the left arrow decreases it.

- If you know anything about vectors, you should know that when two vectors sum together, it results in another vector. Because `player.position` is a vector, in order to change it, we need to add another vector to it. However, `direction_angle` is not a vector, but we can make it into one by calculating the sine and cosine of the angle and then using them as the x and y components of the `movement_vector`.

- Then the `movement_vector` will be added to the current `player.position`. Depending on the x and y components of the `movement_vector`, the `player.position` will change in the direction indicated by the arrow keys.

The updated `draw_player` function looks like this:

```rust
fn draw_player(
    mut gizmos: Gizmos,
    mut player_query: Query<&mut Player>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let size_radius = 20.0;
    for mut player in &mut player_query {
        gizmos.circle_2d(player.position, size_radius, player.color); // Draws a circle at the player's position.
    
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.directiom_angle -= 0.1; // Rotates the player to the left.
        } 
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            player.directiom_angle += 0.1; // Rotates the player to the right.
        }

        // Calculate the movement vector based on the player's direction and speed.
        let x = f32::sin(player.directiom_angle); 
        let y = f32::cos(player.directiom_angle);
        let movement_vector = Vec2::new(x, y);

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player.position += movement_vector; // Moves the player forward.
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            player.position -= movement_vector; // Moves the player backward.
        }
    }    
}
```
Now, run your code and try pressing the arrow keys. If everything is correct, your player should move forewards and backwards and turn when you press the left or right key.

## 4. Speed

- If we want the player to go a little faster (or slower), you need to add another parameter named `speed` to the `Player` struct and define it in the `setup()` function where the player entity is spawned. Then multiply the `player.speed` by the x,y vector when defining the `movement_vector`. 

- Because of the multiplication operation, `movement_vector` will be larger (or smaller), hence the player will move further when the arrow key is pressed, creating the effect of it moving faster.

Here is the finished code:

 **main.rs**
 
 ```rust
use bevy::prelude::*; // includes commonly used types, traits, and functions from the Bevy game engine.
use bevy::color::palettes::basic::*;
//use bevy::input::ButtonInput;

#[derive(Component)] // Marks the Player struct as a component that can be attached to entities in Bevy's Entity-Component-System.
struct Player {
    position: Vec2,
    directiom_angle: f32,
    speed: f32,
    color: Srgba,
} 
fn main() {     
    App::new() // Creates a new Bevy application.
    
        .add_plugins(DefaultPlugins) 

        // Adds the setup system to the Startup stage, which runs once at the beginning.
        .add_systems(Startup, setup) 

        //Adds the player_update system to the Update stage, which runs every frame.
        .add_systems(Update, draw_player) 

         // Runs the application.
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()); //Spawns a 2D camera entity.

    commands.spawn(Player { //Spawns a Player entity with these parameters.
        position: Vec2::new(0.0, 0.0),
        directiom_angle: 0.0,
        speed: 3.0,
        color: RED,
    });
}

fn draw_player(
    mut gizmos: Gizmos,
    mut player_query: Query<&mut Player>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let size_radius = 20.0;
    for mut player in &mut player_query {
        gizmos.circle_2d(player.position, size_radius, player.color); // Draws a circle at the player's position.
    
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.directiom_angle -= 0.1; // Rotates the player to the left.
        } 
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            player.directiom_angle += 0.1; // Rotates the player to the right.
        }

        // Calculate the movement vector based on the player's direction and speed.
        let x = f32::sin(player.directiom_angle);
        let y = f32::cos(player.directiom_angle);
        let movement_vector = Vec2::new(x, y) * player.speed;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player.position += movement_vector; // Moves the player forward.
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            player.position -= movement_vector; // Moves the player backward.
        }
    }    
}
 ```
Congratulations! You have made a fully conrtolable player.