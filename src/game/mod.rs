use bevy::{prelude::*, app::AppExit, time::common_conditions::on_timer, core::FrameCount};
use std::time::Duration;
use rand::{thread_rng, Rng};
use once_cell::sync::Lazy;

use crate::components::tetromino::{TetrominoType, FallingTetromino, spawn_tetromino};

// Background and UI colors using Lazy static
pub static BG_COLOR: Lazy<Color> = Lazy::new(|| Color::hex("111133").unwrap());
pub static GRID_COLOR: Lazy<Color> = Lazy::new(|| Color::hex("333366").unwrap());
pub static ACCENT_COLOR: Lazy<Color> = Lazy::new(|| Color::hex("00FFAA").unwrap());
pub static ACCENT_COLOR_2: Lazy<Color> = Lazy::new(|| Color::hex("FF00AA").unwrap());

#[derive(Component)]
struct Rotating {
    speed: f32,
}

#[derive(Component)]
struct Pulsing {
    speed: f32,
    min_scale: f32,
    max_scale: f32,
}

#[derive(Component)]
struct ScanLines;

#[derive(Component)]
struct GridLines;

#[derive(Component)]
struct MainMenuUI;

#[derive(Resource)]
struct GameAssets {
    vt323_font: Handle<Font>,
    share_tech_mono_font: Handle<Font>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(*BG_COLOR))
            .add_systems(Startup, setup)
            .add_systems(Update, (
                animate_falling_tetrominos,
                animate_rotating_elements,
                animate_pulsing_elements,
                animate_scan_lines,
                spawn_falling_tetrominos.run_if(on_timer(Duration::from_secs_f32(0.8))),
                update_ui_positions,
                handle_exit,
            ));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let vt323_font: Handle<Font> = asset_server.load("fonts/VT323-Regular.ttf");
    let share_tech_mono_font: Handle<Font> = asset_server.load("fonts/ShareTechMono-Regular.ttf");

    commands.insert_resource(GameAssets {
        vt323_font: vt323_font.clone(),
        share_tech_mono_font: share_tech_mono_font.clone(),
    });

    commands.spawn(Camera2dBundle::default());

    // Grid lines
    let grid_size = 40.0;
    let num_lines = 30;

    for i in -num_lines..=num_lines {
        // Vertical lines
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: *GRID_COLOR,
                    custom_size: Some(Vec2::new(1.0, 2000.0)),
                    ..default()
                },
                transform: Transform::from_xyz(i as f32 * grid_size, 0.0, 1.0),
                ..default()
            },
            GridLines,
        ));

        // Horizontal lines
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: *GRID_COLOR,
                    custom_size: Some(Vec2::new(2000.0, 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, i as f32 * grid_size, 1.0),
                ..default()
            },
            GridLines,
        ));
    }

    // Scan lines overlay
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.03),
                custom_size: Some(Vec2::new(2000.0, 2000.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        ScanLines,
    ));

    // Main menu UI container
    let main_menu = commands.spawn((
        SpatialBundle::default(),
        MainMenuUI,
    )).id();

    // Title text
    commands.entity(main_menu).with_children(|parent| {
        parent.spawn((
            Text2dBundle {
                text: Text::from_section(
                    "RustBlocks",
                    TextStyle {
                        font: vt323_font.clone(),
                        font_size: 120.0,
                        color: Color::WHITE,
                    },
                ).with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(0.0, 180.0, 10.0),
                ..default()
            },
            Pulsing {
                speed: 0.8,
                min_scale: 1.0,
                max_scale: 1.05,
            },
        ));

        // Subtitle
        parent.spawn(
            Text2dBundle {
                text: Text::from_section(
                    "Retro-Modern Tetris in Rust",
                    TextStyle {
                        font: share_tech_mono_font.clone(),
                        font_size: 24.0,
                        color: *ACCENT_COLOR,
                    },
                ).with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(0.0, 120.0, 10.0),
                ..default()
            },
        );

        // "Press Enter" text
        parent.spawn((
            Text2dBundle {
                text: Text::from_section(
                    "PRESS ENTER TO START",
                    TextStyle {
                        font: vt323_font.clone(),
                        font_size: 36.0,
                        color: *ACCENT_COLOR_2,
                    },
                ).with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(0.0, -160.0, 10.0),
                ..default()
            },
            Pulsing {
                speed: 2.0,
                min_scale: 0.95,
                max_scale: 1.05,
            },
        ));

        // Version info
        parent.spawn(
            Text2dBundle {
                text: Text::from_section(
                    "V0.1.0",
                    TextStyle {
                        font: share_tech_mono_font.clone(),
                        font_size: 16.0,
                        color: Color::rgba(1.0, 1.0, 1.0, 0.4),
                    },
                ).with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(380.0, -280.0, 10.0),
                ..default()
            },
        );
    });
}

fn spawn_falling_tetrominos(
    mut commands: Commands,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let mut rng = thread_rng();

    let tetromino_type = TetrominoType::random();
    let width_range = window.width() * 0.7;
    let x_pos = rng.gen_range(-width_range..width_range);
    let y_pos = window.height() / 2.0 + 100.0;

    let rotation_speed = rng.gen_range(0.2..1.0);
    let fall_speed = rng.gen_range(40.0..100.0);

    spawn_tetromino(
        &mut commands,
        tetromino_type,
        Vec3::new(x_pos, y_pos, 5.0),
        20.0,
        Some(FallingTetromino {
            speed: fall_speed,
            rotation_speed,
        }),
    );
}

fn update_ui_positions(
    windows: Query<&Window>,
    mut ui_query: Query<&mut Transform, With<MainMenuUI>>,
) {
    if let Ok(mut ui_transform) = ui_query.get_single_mut() {
        let _window = windows.single(); // fixed unused variable
        ui_transform.translation.x = 0.0;
        ui_transform.translation.y = 0.0;
    }
}

fn animate_falling_tetrominos(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &FallingTetromino)>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let bottom = -window.height() / 2.0 - 100.0;

    for (entity, mut transform, tetromino) in query.iter_mut() {
        transform.translation.y -= tetromino.speed * time.delta_seconds();
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds() * tetromino.rotation_speed);

        if transform.translation.y < bottom {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn animate_rotating_elements(time: Res<Time>, mut query: Query<(&mut Transform, &Rotating)>) {
    for (mut transform, rotating) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds() * rotating.speed);
    }
}

fn animate_pulsing_elements(time: Res<Time>, mut query: Query<(&mut Transform, &Pulsing)>) {
    for (mut transform, pulsing) in query.iter_mut() {
        let scale_factor = ((time.elapsed_seconds() * pulsing.speed).sin() * 0.5 + 0.5)
            * (pulsing.max_scale - pulsing.min_scale)
            + pulsing.min_scale;

        transform.scale = Vec3::splat(scale_factor);
    }
}

fn animate_scan_lines(
    time: Res<Time>,
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Sprite, With<ScanLines>>,
) {
    for mut sprite in query.iter_mut() {
        if frame_count.0 % 2 == 0 {
            let flicker = (time.elapsed_seconds() * 2.5).sin() * 0.01 + 0.03;
            sprite.color.set_a(flicker);
        }
    }
}

fn handle_exit(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
