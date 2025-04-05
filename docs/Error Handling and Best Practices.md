# Rust Error Handling & Bevy Best Practices

## Error Handling Patterns

### Result and Option Types

```rust
// Prefer Result for operations that can fail
fn load_game_assets(asset_server: &Res<AssetServer>) -> Result<GameAssets, AssetError> {
    // Implementation
}

// Use Option for values that might not exist
fn get_active_tetromino(query: &Query<(Entity, &ActiveTetromino)>) -> Option<Entity> {
    // Implementation
}
```

### Error Type Strategy

```rust
// Define a game-specific error enum
#[derive(Debug)]
pub enum GameError {
    AssetLoadFailure(String),
    InvalidBoardState,
    RenderingFailure(String),
    AudioFailure(String),
}

// Implement standard error traits
impl std::error::Error for GameError {}
impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Implementation
    }
}
```

### Error Propagation with Bevy

```rust
// Using Result with Bevy systems
fn asset_loading_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut game_assets: ResMut<GameAssets>,
) -> Result<(), GameError> {
    let texture = asset_server.load("textures/block.png");
    
    // Check if asset is loaded
    if asset_server.get_load_state(texture.clone_weak()) != LoadState::Loaded {
        return Err(GameError::AssetLoadFailure("Block texture failed to load".into()));
    }
    
    game_assets.block_texture = texture;
    Ok(())
}

// Using the error handling system
fn setup_plugin(app: &mut App) {
    app.add_systems(Startup, 
        asset_loading_system.pipe(handle_asset_loading_error)
    );
}

// Error handler system
fn handle_asset_loading_error(In(result): In<Result<(), GameError>>) {
    if let Err(error) = result {
        error!("Asset loading error: {}", error);
        // Handle the error appropriately
    }
}
```

### Error Recovery Patterns

```rust
// Use fallbacks for non-critical errors
fn load_high_scores(mut commands: Commands) {
    match read_scores_from_storage() {
        Ok(scores) => {
            commands.insert_resource(HighScores(scores));
        },
        Err(e) => {
            error!("Failed to load scores: {}", e);
            // Fallback to empty scores
            commands.insert_resource(HighScores(Vec::new()));
        }
    }
}
```

## Bevy ECS Best Practices

### Component Design

```rust
// Keep components small and focused
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct TetrominoType(TetroShape);

#[derive(Component)]
struct Rotation(RotationState);

// Use marker components for categorization
#[derive(Component)]
struct ActiveTetromino;

#[derive(Component)]
struct GhostTetromino;

// Bundle components for common entity types
#[derive(Bundle)]
struct TetrominoBundle {
    position: Position,
    tetromino_type: TetrominoType,
    rotation: Rotation,
    active: ActiveTetromino,
    // Render components
    sprite: SpriteBundle,
}
```

### Resource Management

```rust
// Game state stored in resources
#[derive(Resource)]
struct Board {
    cells: [[CellState; 10]; 20],
}

#[derive(Resource)]
struct Score {
    value: u32,
    level: u32,
    lines_cleared: u32,
}

// Access resources in systems
fn update_score_system(
    mut score: ResMut<Score>,
    board_events: EventReader<LinesClearedEvent>,
) {
    for event in board_events.iter() {
        score.value += calculate_score(event.lines_count, score.level);
        score.lines_cleared += event.lines_count as u32;
        // Update level based on lines cleared
        score.level = (score.lines_cleared / 10) + 1;
    }
}
```

### System Design

```rust
// Keep systems focused on a single task
fn tetromino_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Position, With<ActiveTetromino>>,
    board: Res<Board>,
    time: Res<Time>,
) {
    // Implementation
}

// Use query filters effectively
fn render_ghost_tetromino(
    mut ghost_query: Query<(&mut Transform, &Position), With<GhostTetromino>>,
    active_query: Query<(&Position, &TetrominoType, &Rotation), With<ActiveTetromino>>,
    board: Res<Board>,
) {
    // Implementation
}

// Use system sets for organization and ordering
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameplaySystems {
    InputProcessing,
    TetrominoMovement,
    CollisionDetection,
    LineClearing,
}

fn setup_game_systems(app: &mut App) {
    app
        .configure_sets(
            Update,
            (
                GameplaySystems::InputProcessing,
                GameplaySystems::TetrominoMovement,
                GameplaySystems::CollisionDetection,
                GameplaySystems::LineClearing,
            ).chain()
        )
        .add_systems(Update, process_input.in_set(GameplaySystems::InputProcessing))
        .add_systems(Update, move_tetromino.in_set(GameplaySystems::TetrominoMovement))
        // Additional systems
}
```

### State Management

```rust
// Define game states
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

// Add state-specific systems
fn setup_states(app: &mut App) {
    app
        .add_state::<GameState>()
        .add_systems(OnEnter(GameState::Playing), spawn_initial_tetromino)
        .add_systems(OnExit(GameState::Playing), cleanup_game_entities)
        .add_systems(
            Update,
            (
                tetromino_movement_system,
                collision_detection_system,
            ).run_if(in_state(GameState::Playing))
        )
}

// State transition system
fn check_game_over(
    board: Res<Board>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if is_game_over(&board) {
        next_state.set(GameState::GameOver);
    }
}
```

### Event Handling

```rust
// Define custom events
#[derive(Event)]
struct LinesClearedEvent {
    lines: Vec<usize>,
    lines_count: usize,
}

#[derive(Event)]
struct TetrominoPlacedEvent {
    position: Position,
    tetromino_type: TetrominoType,
}

// Register events
fn register_events(app: &mut App) {
    app
        .add_event::<LinesClearedEvent>()
        .add_event::<TetrominoPlacedEvent>();
}

// Send events
fn check_lines_system(
    board: Res<Board>,
    mut line_events: EventWriter<LinesClearedEvent>,
) {
    let lines = find_completed_lines(&board);
    if !lines.is_empty() {
        line_events.send(LinesClearedEvent {
            lines: lines.clone(),
            lines_count: lines.len(),
        });
    }
}

// Receive events
fn handle_lines_cleared(
    mut events: EventReader<LinesClearedEvent>,
    mut board: ResMut<Board>,
) {
    for event in events.iter() {
        clear_lines(&mut board, &event.lines);
    }
}
```

## Performance Optimization in Bevy

### Entity Management

```rust
// Efficiently create entities with bundles
fn spawn_tetromino(
    commands: &mut Commands,
    tetromino_type: TetrominoType,
    position: Position,
    assets: &GameAssets,
) {
    commands.spawn(TetrominoBundle {
        position,
        tetromino_type,
        rotation: Rotation(RotationState::R0),
        active: ActiveTetromino,
        sprite: SpriteBundle {
            texture: assets.block_texture.clone(),
            transform: Transform::from_xyz(
                position.x as f32 * BLOCK_SIZE,
                position.y as f32 * BLOCK_SIZE,
                1.0
            ),
            ..Default::default()
        },
    });
}

// Efficiently clean up entities
fn cleanup_game_entities(
    mut commands: Commands,
    tetromino_query: Query<Entity, Or<(With<ActiveTetromino>, With<GhostTetromino>)>>,
) {
    for entity in tetromino_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

### System Optimization

```rust
// Use change detection to avoid unnecessary work
fn update_transform_system(
    mut query: Query<(&Position, &mut Transform), Changed<Position>>,
) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation.x = position.x as f32 * BLOCK_SIZE;
        transform.translation.y = position.y as f32 * BLOCK_SIZE;
    }
}

// Use RemovedComponents for cleanup
fn handle_removed_tetrominos(
    mut commands: Commands,
    mut removed: RemovedComponents<ActiveTetromino>,
    ghost_query: Query<Entity, With<GhostTetromino>>,
) {
    if !removed.is_empty() {
        // When active tetromino is removed, also remove ghost
        for ghost_entity in ghost_query.iter() {
            commands.entity(ghost_entity).despawn();
        }
    }
}
```

### Asset Management

```rust
// Efficiently load and manage assets
#[derive(Resource)]
struct GameAssets {
    block_textures: [Handle<Image>; 7],
    font: Handle<Font>,
    place_sound: Handle<AudioSource>,
    clear_sound: Handle<AudioSource>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        block_textures: [
            asset_server.load("textures/block_i.png"),
            asset_server.load("textures/block_o.png"),
            asset_server.load("textures/block_t.png"),
            asset_server.load("textures/block_s.png"),
            asset_server.load("textures/block_z.png"),
            asset_server.load("textures/block_j.png"),
            asset_server.load("textures/block_l.png"),
        ],
        font: asset_server.load("fonts/game_font.ttf"),
        place_sound: asset_server.load("audio/place.ogg"),
        clear_sound: asset_server.load("audio/clear.ogg"),
    });
}
```

## Testing in Bevy

### Unit Testing Components and Resources

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tetromino_rotation() {
        let rotation = Rotation(RotationState::R0);
        let rotated = rotation.rotate_cw();
        
        assert_eq!(rotated, Rotation(RotationState::R90));
        
        // Test full rotation
        let mut rotation = Rotation(RotationState::R0);
        for _ in 0..4 {
            rotation = rotation.rotate_cw();
        }
        assert_eq!(rotation, Rotation(RotationState::R0));
    }
}
```

### Integration Testing with Bevy

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use bevy::prelude::*;
    use bevy::app::AppExit;
    
    // Test that game initializes properly
    #[test]
    fn test_game_initialization() {
        // Create a test app
        let mut app = App::new();
        
        // Add only the minimal plugins needed for testing
        app.add_plugins(MinimalPlugins)
           .add_plugin(GamePlugin);
        
        // Initialize and run for a single update
        app.update();
        
        // Verify initial state
        assert_eq!(app.world.resource::<State<GameState>>().get(), &GameState::MainMenu);
        
        // Verify resources were added
        assert!(app.world.contains_resource::<Board>());
        assert!(app.world.contains_resource::<Score>());
    }
    
    // Test state transitions
    #[test]
    fn test_state_transition() {
        let mut app = App::new();
        
        app.add_plugins(MinimalPlugins)
           .add_plugin(GamePlugin);
        
        // Trigger state change
        app.world.resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        
        // Run update to process state change
        app.update();
        
        // Verify state changed
        assert_eq!(app.world.resource::<State<GameState>>().get(), &GameState::Playing);
        
        // Verify playing state systems ran
        let active_tetromino_query = app.world.query_filtered::<Entity, With<ActiveTetromino>>();
        assert_eq!(active_tetromino_query.iter(&app.world).count(), 1);
    }
}
```

## Common Bevy Patterns and Solutions

### Plugin Organization

```rust
// Main game plugin that organizes all functionality
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add sub-plugins
            .add_plugin(BoardPlugin)
            .add_plugin(TetrominoPlugin)
            .add_plugin(UIPlugin)
            .add_plugin(AudioPlugin)
            
            // Add states
            .add_state::<GameState>()
            
            // Add resources
            .init_resource::<Score>()
            .init_resource::<GameConfig>()
            
            // Add events
            .add_event::<LinesClearedEvent>()
            .add_event::<TetrominoPlacedEvent>()
            
            // Add systems
            .add_systems(Startup, setup_game)
            .add_systems(Update, (
                process_input,
                update_game_logic,
                render_updates,
            ).chain());
    }
}

// Sub-plugin example
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Board>()
            .add_systems(Startup, setup_board)
            .add_systems(Update, (
                check_completed_lines,
                clear_completed_lines,
            ).chain().run_if(in_state(GameState::Playing)));
    }
}
```

### Handle Window Configuration

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RustBlocks".into(),
                resolution: (800.0, 600.0).into(),
                resizable: true,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(GamePlugin)
        .run();
}

// Handle fullscreen toggling
fn toggle_fullscreen_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::Fullscreen,
                _ => WindowMode::Windowed,
            };
        }
    }
}