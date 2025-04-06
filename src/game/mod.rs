use bevy::{prelude::*, app::AppExit, time::common_conditions::on_timer, core::FrameCount};
use std::time::Duration;
use rand::{thread_rng, Rng};
use once_cell::sync::Lazy;

use crate::components::tetromino::{TetrominoType, FallingTetromino, spawn_tetromino, COLORS};

// Use statics instead of consts for non-const function calls
pub static BG_COLOR: Lazy<Color> = Lazy::new(|| Color::hex("111133").unwrap());
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

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.03),
                custom_size: Some(Vec2::new(800.0, 600.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        ScanLines,
    ));

    commands.spawn((
        TextBundle::from_section(
            "RUSTBLOCKS",
            TextStyle {
                font: vt323_font.clone(),
                font_size: 120.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(250.0),
            top: Val::Px(120.0),
            ..default()
        }),
        Pulsing {
            speed: 0.8,
            min_scale: 1.0,
            max_scale: 1.05,
        },
    ));

    commands.spawn(
        TextBundle::from_section(
            "A RETRO-MODERN TETRIS EXPERIENCE",
            TextStyle {
                font: share_tech_mono_font.clone(),
                font_size: 24.0,
                color: *ACCENT_COLOR,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(220.0),
            top: Val::Px(220.0),
            ..default()
        }),
    );

    for (i, color) in COLORS.iter().enumerate() {
        let angle = (i as f32 / COLORS.len() as f32) * std::f32::consts::TAU;
        let radius = 180.0;
        let x_pos = radius * angle.cos();
        let y_pos = 350.0 + (radius * angle.sin() * 0.5);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: *color,
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, y_pos, 10.0),
                ..default()
            },
            Rotating {
                speed: 0.3 + (i as f32 * 0.08),
            },
        ));
    }

    commands.spawn((
        TextBundle::from_section(
            "PRESS ENTER TO START",
            TextStyle {
                font: vt323_font.clone(),
                font_size: 36.0,
                color: *ACCENT_COLOR_2,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(270.0),
            top: Val::Px(500.0),
            ..default()
        }),
        Pulsing {
            speed: 2.0,
            min_scale: 0.95,
            max_scale: 1.05,
        },
    ));

    commands.spawn(
        TextBundle::from_section(
            "V0.1.0",
            TextStyle {
                font: share_tech_mono_font.clone(),
                font_size: 16.0,
                color: Color::rgba(1.0, 1.0, 1.0, 0.4),
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            bottom: Val::Px(20.0),
            ..default()
        }),
    );
}

fn spawn_falling_tetrominos(
    mut commands: Commands,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let mut rng = thread_rng();

    let tetromino_type = TetrominoType::random();
    let x_pos = rng.gen_range(-window.width() / 2.0..window.width() / 2.0);
    let y_pos = window.height() / 2.0 + 50.0;

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
