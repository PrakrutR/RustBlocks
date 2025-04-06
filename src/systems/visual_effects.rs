use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::common_conditions::on_timer};
use rand::{thread_rng, Rng};
use std::time::Duration;

use crate::components::visual_effects::*;
use crate::components::tetromino::{TetrominoType, COLORS};
use crate::components::tetromino::FallingTetromino;

// System to spawn stars for the background
pub fn spawn_stars(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single();
    let mut rng = thread_rng();
    
    // Spawn around 100 stars
    for _ in 0..100 {
        let x = rng.gen_range(-window.width() / 1.5..window.width() / 1.5);
        let y = rng.gen_range(-window.height() / 1.5..window.height() / 1.5);
        let z = -10.0; // Behind everything else
        
        let size = rng.gen_range(1.0..3.0);
        let brightness = rng.gen_range(0.5..1.0);
        let twinkle_speed = rng.gen_range(0.5..2.0);
        let twinkle_strength = rng.gen_range(0.2..0.5);
        let parallax_factor = rng.gen_range(0.01..0.05);
        
        // Create either a square or circle star randomly
        if rng.gen_bool(0.7) {
            // Square star (pixel art style)
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 1.0, 1.0, brightness),
                        custom_size: Some(Vec2::new(size, size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, z),
                    ..default()
                },
                Star {
                    twinkle_speed,
                    twinkle_strength,
                    parallax_factor,
                },
            ));
        } else {
            // Circle star
            let star_mesh = meshes.add(shape::Circle::new(size / 2.0).into());
            let star_material = materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, brightness)));
            
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: star_mesh.into(),
                    material: star_material,
                    transform: Transform::from_xyz(x, y, z),
                    ..default()
                },
                Star {
                    twinkle_speed,
                    twinkle_strength,
                    parallax_factor,
                },
            ));
        }
    }
}

// System to spawn the city skyline
pub fn spawn_city(
    mut commands: Commands,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let mut rng = thread_rng();
    
    // Base y position for the city (near bottom of screen)
    let base_y = -window.height() / 2.0 + 80.0;
    
    // Building colors
    let building_colors = [
        Color::hex("6930C3").unwrap(), // Purple
        Color::hex("5E60CE").unwrap(), // Blue-Purple
        Color::hex("5390D9").unwrap(), // Blue
        Color::hex("4EA8DE").unwrap(), // Light Blue
        Color::hex("48BFE3").unwrap(), // Cyan
    ];
    
    // Window/light colors
    let light_colors = [
        Color::hex("FF9F1C").unwrap(), // Orange-yellow
        Color::hex("FFBF69").unwrap(), // Light orange
        Color::hex("FCCD50").unwrap(), // Yellow
    ];
    
    // Spawn 15-20 buildings across the screen
    let city_width = window.width() * 1.5;
    let num_buildings = rng.gen_range(15..21);
    let building_spacing = city_width / (num_buildings as f32);
    
    for i in 0..num_buildings {
        let x_pos = -city_width / 2.0 + (i as f32 * building_spacing) + rng.gen_range(-20.0..20.0);
        let building_height = rng.gen_range(30.0..120.0);
        let building_width = rng.gen_range(20.0..50.0);
        
        // Determine if this building has a tetromino shape influence
        let is_tetromino_inspired = rng.gen_bool(0.25);
        
        if is_tetromino_inspired {
            // Choose a random tetromino shape and color
            let tetro_type = TetrominoType::random();
            let color = COLORS[tetro_type.index()];
            
            // Create a tetromino-inspired building
            spawn_tetromino_building(&mut commands, x_pos, base_y, tetro_type, color);
        } else {
            // Create a standard building
            let color = building_colors[rng.gen_range(0..building_colors.len())];
            let layer = rng.gen_range(0.0..0.3);
            
            // Main building body
            let building_entity = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(building_width, building_height)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x_pos, base_y + building_height / 2.0, -5.0 + layer),
                    ..default()
                },
                CityElement {
                    bob_speed: rng.gen_range(0.2..0.7),
                    bob_amount: rng.gen_range(0.5..2.0),
                    layer,
                },
            )).id();
            
            // Add windows/lights to the building
            let num_floors = (building_height / 10.0).floor() as i32;
            let num_windows_per_floor = (building_width / 8.0).floor() as i32;
            
            commands.entity(building_entity).with_children(|parent| {
                for floor in 0..num_floors {
                    for window in 0..num_windows_per_floor {
                        // Only add windows with 70% probability
                        if rng.gen_bool(0.7) {
                            let window_x = -building_width / 2.0 + 5.0 + (window as f32 * 8.0);
                            let window_y = -building_height / 2.0 + 5.0 + (floor as f32 * 10.0);
                            
                            // Choose if the light is initially on or off
                            let is_on = rng.gen_bool(0.6);
                            let light_color = light_colors[rng.gen_range(0..light_colors.len())];
                            
                            parent.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: if is_on { light_color } else { Color::rgba(0.1, 0.1, 0.2, 0.5) },
                                        custom_size: Some(Vec2::new(4.0, 4.0)),
                                        ..default()
                                    },
                                    transform: Transform::from_xyz(window_x, window_y, 0.1),
                                    ..default()
                                },
                                CityLight {
                                    blink_interval: Timer::new(
                                        Duration::from_secs_f32(rng.gen_range(0.5..8.0)),
                                        TimerMode::Repeating,
                                    ),
                                    color_on: light_color,
                                    color_off: Color::rgba(0.1, 0.1, 0.2, 0.5),
                                    is_on,
                                },
                            ));
                        }
                    }
                }
            });
        }
    }
}

// Helper function to spawn tetromino-shaped buildings
fn spawn_tetromino_building(
    commands: &mut Commands,
    x_pos: f32,
    base_y: f32,
    tetro_type: TetrominoType,
    color: Color,
) {
    let mut rng = thread_rng();
    let layer = rng.gen_range(0.0..0.3);
    let block_size = 12.0;
    let shape = tetro_type.shape();
    
    // Create parent entity for the tetromino building
    let building_entity = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(x_pos, base_y, -5.0 + layer),
            ..default()
        },
        CityElement {
            bob_speed: rng.gen_range(0.1..0.4),
            bob_amount: rng.gen_range(0.5..1.5),
            layer,
        },
    )).id();
    
    // Add blocks to form the tetromino shape
    commands.entity(building_entity).with_children(|parent| {
        for y in 0..4 {
            for x in 0..4 {
                if shape[y][x] {
                    let block_x = (x as f32 - 1.5) * block_size;
                    let block_y = (y as f32 - 1.5) * block_size + 30.0; // Lift it up a bit
                    
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::new(block_size - 1.0, block_size - 1.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(block_x, block_y, 0.0),
                        ..default()
                    });
                    
                    // Randomly add a light to some blocks
                    if rng.gen_bool(0.3) {
                        let light_color = Color::rgba(
                            color.r() + 0.2,
                            color.g() + 0.2,
                            color.b() + 0.2,
                            0.9,
                        );
                        
                        parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: light_color,
                                    custom_size: Some(Vec2::new(2.0, 2.0)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(block_x, block_y, 0.1),
                                ..default()
                            },
                            CityLight {
                                blink_interval: Timer::new(
                                    Duration::from_secs_f32(rng.gen_range(0.5..5.0)),
                                    TimerMode::Repeating,
                                ),
                                color_on: light_color,
                                color_off: Color::rgba(color.r() * 0.5, color.g() * 0.5, color.b() * 0.5, 0.5),
                                is_on: true,
                            },
                        ));
                    }
                }
            }
        }
    });
}

// Spawn the background gradient
pub fn spawn_background(
    mut commands: Commands,
    windows: Query<&Window>,
) {
    let window = windows.single();
    
    // Create a large background quad 
    let bg_size = Vec2::new(window.width() * 1.5, window.height() * 1.5);
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::hex("111133").unwrap(),
                custom_size: Some(bg_size),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -20.0),
            ..default()
        },
        ColorOscillation {
            base_color: Color::hex("111133").unwrap(),
            target_color: Color::hex("221144").unwrap(),
            speed: 0.05,
            strength: 0.3,
        },
    ));
}

// Spawn grid lines
pub fn spawn_grid(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();
    let grid_size = 20.0; // Size of one tetromino block
    
    // Calculate number of lines needed to cover the screen
    let num_lines_x = (window.width() / grid_size).ceil() as i32 + 4;
    let num_lines_y = (window.height() / grid_size).ceil() as i32 + 4;
    
    let grid_entity = commands.spawn((
        SpatialBundle::default(),
        GridLines {
            fade_timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once),
            fading_in: true,
            alpha: 0.0,
        },
    )).id();
    
    commands.entity(grid_entity).with_children(|parent| {
        // Create horizontal lines
        for i in -num_lines_y..=num_lines_y {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.2, 0.2, 0.4, 0.0), // Start fully transparent
                    custom_size: Some(Vec2::new(window.width() * 2.0, 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, i as f32 * grid_size, 2.0),
                ..default()
            });
        }
        
        // Create vertical lines
        for i in -num_lines_x..=num_lines_x {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.2, 0.2, 0.4, 0.0), // Start fully transparent
                    custom_size: Some(Vec2::new(1.0, window.height() * 2.0)),
                    ..default()
                },
                transform: Transform::from_xyz(i as f32 * grid_size, 0.0, 2.0),
                ..default()
            });
        }
    });
}

// Spawn scan lines effect
pub fn spawn_scan_lines(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();
    
    // Create scan lines overlay
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.03),
                custom_size: Some(Vec2::new(window.width() * 2.0, window.height() * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 90.0),
            ..default()
        },
        ScanLines,
    ));
    
    // Create CRT distortion effect
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.05, 0.2, 0.0), // Start invisible
                custom_size: Some(Vec2::new(window.width() * 2.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 91.0),
            ..default()
        },
        CrtDistortion {
            timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
            active: false,
            strength: 0.0,
        },
    ));
}

// System to update star animations
pub fn animate_stars(
    time: Res<Time>,
    mut stars: Query<(&mut Sprite, &Star, &mut Transform)>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    
    for (mut sprite, star, mut transform) in stars.iter_mut() {
        // Twinkle effect
        let alpha = 0.5 + (time.elapsed_seconds() * star.twinkle_speed).sin() * star.twinkle_strength;
        sprite.color.set_a(alpha);
        
        // Parallax effect (subtle movement)
        let parallax_offset = (time.elapsed_seconds() * 0.1).sin() * star.parallax_factor * window.width() * 0.1;
        transform.translation.x += parallax_offset * time.delta_seconds();
        
        // Keep stars on screen with wraparound
        let half_width = window.width() / 1.5;
        if transform.translation.x > half_width {
            transform.translation.x = -half_width;
        } else if transform.translation.x < -half_width {
            transform.translation.x = half_width;
        }
    }
}

// System to animate city elements
pub fn animate_city(
    time: Res<Time>,
    mut city_elements: Query<(&mut Transform, &CityElement)>,
) {
    for (mut transform, element) in city_elements.iter_mut() {
        // Gentle bobbing motion
        let bob_offset = (time.elapsed_seconds() * element.bob_speed).sin() * element.bob_amount;
        transform.translation.y += bob_offset * time.delta_seconds();
    }
}

// System to animate city lights
pub fn animate_city_lights(
    time: Res<Time>,
    mut lights: Query<(&mut Sprite, &mut CityLight)>,
) {
    for (mut sprite, mut light) in lights.iter_mut() {
        // Update the blink timer
        if light.blink_interval.tick(time.delta()).just_finished() {
            // Toggle light state
            light.is_on = !light.is_on;
            
            // Update sprite color based on state
            sprite.color = if light.is_on { light.color_on } else { light.color_off };
        }
    }
}

// System to animate color oscillation
pub fn animate_color_oscillation(
    time: Res<Time>,
    mut color_elements: Query<(&mut Sprite, &ColorOscillation)>,
) {
    for (mut sprite, oscillation) in color_elements.iter_mut() {
        let t = (time.elapsed_seconds() * oscillation.speed).sin() * 0.5 + 0.5;
        let t = t * oscillation.strength;
        
        // Lerp between colors
        sprite.color = Color::rgba(
            lerp(oscillation.base_color.r(), oscillation.target_color.r(), t),
            lerp(oscillation.base_color.g(), oscillation.target_color.g(), t),
            lerp(oscillation.base_color.b(), oscillation.target_color.b(), t),
            1.0,
        );
    }
}

// System to animate grid lines fading
pub fn animate_grid_lines(
    time: Res<Time>,
    mut effect_timers: ResMut<EffectTimers>,
    mut grid_query: Query<(&mut GridLines, &Children)>,
    mut sprite_query: Query<&mut Sprite>,
) {
    // Update the grid effect timer
    if effect_timers.grid_effect.tick(time.delta()).just_finished() {
        // Toggle the grid visibility when timer finishes
        for (mut grid_lines, children) in grid_query.iter_mut() {
            grid_lines.fading_in = true;
            grid_lines.fade_timer.reset();
        }
    }
    
    // Update grid fade animation
    for (mut grid_lines, children) in grid_query.iter_mut() {
        if grid_lines.fade_timer.tick(time.delta()).finished() {
            // If fade timer is finished, start the opposite animation
            grid_lines.fading_in = !grid_lines.fading_in;
            grid_lines.fade_timer.reset();
        }
        
        // Calculate current alpha based on fade direction
        let progress = grid_lines.fade_timer.percent();
        grid_lines.alpha = if grid_lines.fading_in {
            progress * 0.3 // Max alpha of 0.3
        } else {
            (1.0 - progress) * 0.3
        };
        
        // Update all child sprites with the new alpha
        for &child in children.iter() {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                sprite.color.set_a(grid_lines.alpha);
            }
        }
    }
}

// System to animate scan lines and CRT distortion
pub fn animate_scan_lines(
    time: Res<Time>,
    mut effect_timers: ResMut<EffectTimers>,
    mut scan_lines: Query<&mut Sprite, (With<ScanLines>, Without<CrtDistortion>)>,
    mut crt_distortion: Query<(&mut Sprite, &mut Transform, &mut CrtDistortion), Without<ScanLines>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    
    // Animate scan lines - subtle flicker
    for mut sprite in scan_lines.iter_mut() {
        let flicker = (time.elapsed_seconds() * 2.5).sin() * 0.01 + 0.03;
        sprite.color.set_a(flicker);
    }
    
    // Update CRT distortion timer
    if effect_timers.crt_distortion.tick(time.delta()).just_finished() {
        // Activate CRT distortion effect
        for (_, _, mut distortion) in crt_distortion.iter_mut() {
            distortion.active = true;
            distortion.timer.reset();
        }
    }
    
    // Animate CRT distortion
    for (mut sprite, mut transform, mut distortion) in crt_distortion.iter_mut() {
        if distortion.active {
            // Update distortion timer
            if distortion.timer.tick(time.delta()).just_finished() {
                distortion.active = false;
                sprite.color.set_a(0.0);
                continue;
            }
            
            // Calculate current strength
            let progress = distortion.timer.percent();
            let effect_duration = 0.5; // Effect active for 0.5 seconds
            
            if progress < effect_duration {
                // During active phase
                let normalized_progress = progress / effect_duration;
                distortion.strength = if normalized_progress < 0.5 {
                    normalized_progress * 2.0
                } else {
                    (1.0 - normalized_progress) * 2.0
                };
                
                // Set color and position
                sprite.color.set_a(distortion.strength * 0.4);
                
                // Move the distortion line
                let position_y = ((time.elapsed_seconds() * 20.0) % window.height()) - window.height() / 2.0;
                transform.translation.y = position_y;
            } else {
                // Fade out phase
                distortion.strength = 0.0;
                sprite.color.set_a(0.0);
            }
        }
    }
}

// System to spawn falling tetrominos with particle trails
pub fn spawn_falling_tetrominos_with_trails(
    mut commands: Commands,
    windows: Query<&Window>,
    time: Res<Time>,
) {
    let window = windows.single();
    let mut rng = thread_rng();
    
    // Only spawn new tetrominos at a regular interval
    if time.elapsed_seconds() % 0.8 < time.delta_seconds() {
        // Randomly pick tetromino type
        let tetromino_type = TetrominoType::random();
        let color = tetromino_type.color();
        
        // Random position at top of window with wider range
        let width_range = window.width() * 0.7;
        let x_pos = rng.gen_range(-width_range..width_range);
        let y_pos = window.height() / 2.0 + 100.0; // Start above the visible area
        
        // Fall speed varies slightly
        let fall_speed = rng.gen_range(30.0..70.0);
        
        // Create the tetromino parent entity
        let tetromino_entity = commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(x_pos, y_pos, 5.0),
                ..default()
            },
            FallingTetromino {
                speed: fall_speed,
                rotation_speed: 0.0, // No rotation as requested
            },
        )).id();
        
        // Add blocks to form the tetromino shape
        let block_size = 20.0;
        let shape = tetromino_type.shape();
        
        commands.entity(tetromino_entity).with_children(|parent| {
            for y in 0..4 {
                for x in 0..4 {
                    if shape[y][x] {
                        let block_x = (x as f32 - 1.5) * block_size;
                        let block_y = (y as f32 - 1.5) * block_size;
                        
                        parent.spawn(SpriteBundle {
                            sprite: Sprite {
                                color,
                                custom_size: Some(Vec2::new(block_size - 1.0, block_size - 1.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(block_x, -block_y, 0.0),
                            ..default()
                        });
                    }
                }
            }
        });
    }
}

// System to animate falling tetrominos and create particle trails
pub fn animate_falling_tetrominos_with_trails(
    mut commands: Commands,
    time: Res<Time>,
    mut tetrominos: Query<(Entity, &mut Transform, &FallingTetromino)>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let bottom = -window.height() / 2.0 - 100.0; // Bottom of screen + buffer
    let mut rng = thread_rng();
    
    for (entity, mut transform, tetromino) in tetrominos.iter_mut() {
        // Apply falling movement
        let prev_y = transform.translation.y;
        transform.translation.y -= tetromino.speed * time.delta_seconds();
        
        // Create particle trail (50% chance per frame to avoid too many particles)
        if rng.gen_bool(0.5) {
            // Get the tetromino color from one of its children
            let trail_color = Color::rgba(1.0, 1.0, 1.0, 0.3); // Default fallback
            
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: trail_color,
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        transform.translation.x + rng.gen_range(-10.0..10.0),
                        transform.translation.y + rng.gen_range(0.0..20.0),
                        4.0, // Slightly behind the tetromino
                    ),
                    ..default()
                },
                TrailParticle {
                    lifetime: Timer::new(Duration::from_secs_f32(0.8), TimerMode::Once),
                    initial_alpha: 0.3,
                    color: trail_color,
                    size: Vec2::new(8.0, 8.0),
                },
            ));
        }
        
        // Remove when off screen
        if transform.translation.y < bottom {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// System to animate particle trails
pub fn animate_particle_trails(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Sprite, &mut TrailParticle, &mut Transform)>,
) {
    for (entity, mut sprite, mut particle, mut transform) in particles.iter_mut() {
        // Update timer
        particle.lifetime.tick(time.delta());
        
        if particle.lifetime.finished() {
            // Remove expired particles
            commands.entity(entity).despawn();
        } else {
            // Fade out based on remaining lifetime
            let remaining_percent = 1.0 - particle.lifetime.percent();
            sprite.color.set_a(particle.initial_alpha * remaining_percent);
            
            // Shrink slightly
            let current_size = lerp(particle.size.x, 0.0, particle.lifetime.percent());
            sprite.custom_size = Some(Vec2::new(current_size, current_size));
            
            // Float upward slightly
            transform.translation.y += 2.0 * time.delta_seconds();
        }
    }
}

// System to update the pulsing effect
pub fn animate_pulsing_elements(time: Res<Time>, mut query: Query<(&mut Transform, &Pulsing)>) {
    for (mut transform, pulsing) in query.iter_mut() {
        let scale_factor = ((time.elapsed_seconds() * pulsing.speed).sin() * 0.5 + 0.5) 
            * (pulsing.max_scale - pulsing.min_scale) 
            + pulsing.min_scale;
        
        transform.scale = Vec3::splat(scale_factor);
    }
}

// Helper function for linear interpolation
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

// System to initialize effect timers
pub fn init_effect_timers(mut commands: Commands) {
    commands.insert_resource(EffectTimers::default());
}