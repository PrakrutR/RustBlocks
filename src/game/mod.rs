use bevy::prelude::*;
use bevy::app::AppExit;

// Define our tetromino colors for visual identity
pub const COLORS: [Color; 7] = [
    Color::rgb(0.0, 0.8, 0.8),  // I - Cyan
    Color::rgb(0.8, 0.8, 0.0),  // O - Yellow
    Color::rgb(0.8, 0.0, 0.8),  // T - Purple
    Color::rgb(0.0, 0.8, 0.0),  // S - Green
    Color::rgb(0.8, 0.0, 0.0),  // Z - Red
    Color::rgb(0.0, 0.0, 0.8),  // J - Blue
    Color::rgb(0.8, 0.4, 0.0),  // L - Orange
];

// Simple component for rotation
#[derive(Component)]
struct Rotating {
    speed: f32,
}

// Main game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (animate_shapes, handle_exit));
    }
}

// Setup the initial scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add 2D camera
    commands.spawn(Camera2dBundle::default());
    
    // Add title text
    commands.spawn(
        TextBundle::from_section(
            "RustBlocks",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 72.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(300.0),
            top: Val::Px(100.0),
            ..default()
        }),
    );
    
    // Add subtitle
    commands.spawn(
        TextBundle::from_section(
            "A Tetris game using Bevy Engine",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 28.0,
                color: Color::rgb(0.8, 0.8, 0.8),
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(250.0),
            top: Val::Px(180.0),
            ..default()
        }),
    );
    
    // Create tetromino blocks in a circular pattern
    for (i, color) in COLORS.iter().enumerate() {
        let angle = (i as f32 / COLORS.len() as f32) * std::f32::consts::TAU;
        let radius = 150.0;
        let x_pos = 400.0 + radius * angle.cos();
        let y_pos = 350.0 + radius * angle.sin();
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: *color,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                ..default()
            },
            Rotating {
                speed: 0.5 + (i as f32 * 0.1),
            },
        ));
    }
    
    // Add instructions text
    commands.spawn(
        TextBundle::from_section(
            "Press ESC to exit",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                color: Color::rgba(1.0, 1.0, 1.0, 0.7),
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(320.0),
            top: Val::Px(500.0),
            ..default()
        }),
    );
}

// Animate the tetromino blocks
fn animate_shapes(time: Res<Time>, mut query: Query<(&mut Transform, &Rotating)>) {
    for (mut transform, rotating) in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds() * rotating.speed);
    }
}

// Handle exit when ESC is pressed
fn handle_exit(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}