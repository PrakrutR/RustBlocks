use bevy::prelude::*;
use std::time::Duration;

// Star components
#[derive(Component)]
pub struct Star {
    pub twinkle_speed: f32,
    pub twinkle_strength: f32,
    pub parallax_factor: f32,
}

// City components
#[derive(Component)]
pub struct CityElement {
    pub bob_speed: f32,
    pub bob_amount: f32,
    pub layer: f32, // For parallax effect
}

#[derive(Component)]
pub struct CityLight {
    pub blink_interval: Timer,
    pub color_on: Color,
    pub color_off: Color,
    pub is_on: bool,
}

// Background effects
#[derive(Component)]
pub struct ParallaxBackground {
    pub speed: f32,
    pub layer: f32,
}

#[derive(Component)]
pub struct ScanLines;

#[derive(Component)]
pub struct CrtDistortion {
    pub timer: Timer,
    pub active: bool,
    pub strength: f32,
}

// Grid effects
#[derive(Component)]
pub struct GridLines {
    pub fade_timer: Timer,
    pub fading_in: bool,
    pub alpha: f32,
}

// Particle trail effect for tetrominos
#[derive(Component)]
pub struct TrailParticle {
    pub lifetime: Timer,
    pub initial_alpha: f32,
    pub color: Color,
    pub size: Vec2,
}

#[derive(Component)]
pub struct Pulsing {
    pub speed: f32,
    pub min_scale: f32,
    pub max_scale: f32,
}

// Color oscillation effect
#[derive(Component)]
pub struct ColorOscillation {
    pub base_color: Color,
    pub target_color: Color,
    pub speed: f32,
    pub strength: f32,
}

// Global timer resources for synced effects
#[derive(Resource)]
pub struct EffectTimers {
    pub crt_distortion: Timer,
    pub grid_effect: Timer,
}

impl Default for EffectTimers {
    fn default() -> Self {
        Self {
            crt_distortion: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
            grid_effect: Timer::new(Duration::from_secs(25), TimerMode::Repeating),
        }
    }
}