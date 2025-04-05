use bevy::prelude::*;

// Main application states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Settings,
}

// In-game states (active when AppState is Playing)
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Spawning,  // Generating a new tetromino
    Falling,   // Tetromino is falling
    Locking,   // Tetromino is in the process of locking to the board
    Clearing,  // Clearing completed lines
}

// Bundle states together for initialization in the plugin
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<AppState>()
            .add_state::<GameState>();
    }
}