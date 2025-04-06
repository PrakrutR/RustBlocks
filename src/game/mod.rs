use bevy::{prelude::*, app::AppExit, time::common_conditions::on_timer};
use std::time::Duration;

// Only importing what we need
use crate::components::visual_effects::*;
use crate::systems::visual_effects::*;

// === Components for this file ===

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct Pulsing {
    speed: f32,
    min_scale: f32,
    max_scale: f32,
}

// === Plugin ===

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Background color
            .insert_resource(ClearColor(Color::hex("111133").unwrap()))
            
            // Startup: timers & background entities
            .add_systems(Startup, init_effect_timers)
            .add_systems(Startup, (
                spawn_background,
                spawn_stars,
                spawn_grid,
                spawn_city,
                spawn_scan_lines,
                setup_menu,
            ))
            
            // Update loop: visual effects
            .add_systems(Update, (
                animate_stars,
                animate_city,
                animate_city_lights,
                animate_color_oscillation,
                animate_grid_lines,
                animate_scan_lines,
                spawn_falling_tetrominos_with_trails.run_if(on_timer(Duration::from_secs_f32(0.8))),
                animate_falling_tetrominos_with_trails,
                animate_particle_trails,
                animate_pulsing_elements,
                handle_exit,
            ));
    }
}

// === Setup main menu UI ===

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let vt323_font: Handle<Font> = asset_server.load("fonts/VT323-Regular.ttf");
    let share_tech_mono_font: Handle<Font> = asset_server.load("fonts/ShareTechMono-Regular.ttf");

    let main_menu = commands.spawn((
        SpatialBundle::default(),
        MainMenuUI,
    )).id();

    commands.entity(main_menu).with_children(|parent| {
        // Title
        parent.spawn((
            Text2dBundle {
                text: Text::from_section(
                    "RUSTBLOCKS",
                    TextStyle {
                        font: vt323_font.clone(),
                        font_size: 120.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
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
                    "A RETRO-MODERN TETRIS EXPERIENCE",
                    TextStyle {
                        font: share_tech_mono_font.clone(),
                        font_size: 24.0,
                        color: Color::hex("00FFAA").unwrap(),
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(0.0, 120.0, 10.0),
                ..default()
            },
        );

        // "Press Enter" prompt
        parent.spawn((
            Text2dBundle {
                text: Text::from_section(
                    "PRESS ENTER TO START",
                    TextStyle {
                        font: vt323_font.clone(),
                        font_size: 36.0,
                        color: Color::hex("FF00AA").unwrap(),
                    },
                )
                .with_alignment(TextAlignment::Center),
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
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(380.0, -280.0, 10.0),
                ..default()
            },
        );
    });
}

// === Input handler ===

fn handle_exit(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
