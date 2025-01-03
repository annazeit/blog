# Getting Started with Rust and Bevy

## Step 1: Install Rust

1. **Download and run the Rust installer**:
   - Go to the [official Rust website](https://www.rust-lang.org/).
   - Click on the "Get Started" button.
   - Follow the instructions to download and run the installer for your operating system.

2. **Verify the installation**:
   - Open a terminal or command prompt.
   - Run the following command to check the Rust version:
     ```sh
     rustc --version
     ```
   - You should see the installed version of Rust.

## Step 2: Install Bevy

1. **Create a new Rust project**:
   - Open a terminal or command prompt.
   - Run the following command to create a new Rust project:
     ```sh
     cargo new my_bevy_game
     ```
   - Navigate to the project directory:
     ```sh
     cd my_bevy_game
     ```

2. **Add Bevy as a dependency**:
   - Open the `Cargo.toml` file in your project directory.
   - Add the following line under `[dependencies]`:
     ```toml
     [dependencies]
     bevy = "0.11"  # Check the latest version on the Bevy website
     ```

3. **Write a simple Bevy application**:
   - Open the `src/main.rs` file in your project directory.
   - Replace the content with this simple code:
     ```rust
     use bevy::prelude::*;

     fn main() {
         App::build()
             .add_plugins(DefaultPlugins)
             .add_startup_system(hello_world.system())
             .run();
     }

     fn hello_world() {
         println!("Hello, Bevy!");
     }
     ```

4. **Run your Bevy application**:
   - In the terminal or command prompt, run the following command:
     ```sh
     cargo run
     ```
   - You should see "Hello, Bevy!" printed in the terminal.

Congratulations! You have now installed Rust and Bevy and created a simple Bevy application.