use bevy::{
    color::palettes::basic::*, prelude::* 
};

#[derive(Component)]
struct MyMusic;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system) // button stuff
        .add_systems(Update, volume) // audio stuff
        .run();
}


const NORMAL_BUTTON: Color = Color::srgb(0.96, 0.94, 0.90); // Milky white color
const HOVERED_BUTTON: Color = Color::srgb(0.96, 0.96, 0.86); // Beige color
const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.80, 0.65); // Darker beige color

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d); // UI camera

    commands.spawn((
        AudioPlayer::new(asset_server.load("sillymusic.ogg")),
        MyMusic
    ));

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ));
        });
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    music_controller: Query<&AudioSink, With<MyMusic>>, // for audio control
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
                pause(&music_controller); // pause the music
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn pause(
    music_controller: &Query<&AudioSink, With<MyMusic>>,
) {
    if let Ok(sink) = music_controller.get_single() {
        sink.toggle();
    }
}

fn volume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink, With<MyMusic>>,
) {
    if let Ok(sink) = music_controller.get_single() {
        if keyboard_input.just_pressed(KeyCode::Equal) {
            if sink.volume() < 4.9 {
                sink.set_volume(sink.volume() + 0.1);
            }
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            if sink.volume() > 0.0 {
                sink.set_volume(sink.volume() - 0.1);
            }
        }
    }

    // print the current volume to the terminal
    if let Ok(sink) = music_controller.get_single() {
        println!("Volume: {:.1}", sink.volume()); // print the volume rounded to 1 decimal place
    }
}