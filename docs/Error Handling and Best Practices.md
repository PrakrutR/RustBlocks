# Rust Error Handling & Best Practices 

## Error Handling Patterns

### Result and Option Types

```rust
// Prefer Result for operations that can fail
fn load_game_assets() -> Result<GameAssets, AssetError> {
    // Implementation
}

// Use Option for values that might not exist
fn get_active_tetromino() -> Option<Tetromino> {
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

### Error Propagation

```rust
// Use the ? operator for clean error propagation
fn initialize_game() -> Result<Game, GameError> {
    let assets = load_game_assets()?;
    let board = create_board()?;
    let renderer = initialize_renderer()?;
    
    Ok(Game { assets, board, renderer })
}
```

### Error Recovery Patterns

```rust
// Use fallbacks for non-critical errors
fn load_high_scores() -> Vec<Score> {
    match read_scores_from_storage() {
        Ok(scores) => scores,
        Err(e) => {
            log::warn!("Failed to load scores: {}", e);
            Vec::new() // Fallback to empty scores
        }
    }
}
```

## Memory Management Approaches

### Game State Organization

```rust
// Organize game state to leverage ownership
struct Game {
    board: Board,
    current_piece: Option<Tetromino>,
    next_pieces: Vec<Tetromino>,
    score: Score,
}

// Use references for read-only access
fn render_game(game: &Game) {
    // Implementation
}

// Use mutable references for state changes
fn update_game(game: &mut Game) {
    // Implementation
}
```

### Resource Management

```rust
// Use RAII for resource management
struct AudioManager {
    sound_effects: HashMap<SoundEffect, AudioHandle>,
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        // Clean up audio resources
    }
}
```

### Avoiding Allocations During Gameplay

```rust
// Pre-allocate collections
let mut particle_effects = Vec::with_capacity(MAX_PARTICLES);

// Reuse vectors instead of creating new ones
fn clear_completed_lines(board: &mut Board) {
    board.line_clear_buffer.clear(); // Reuse existing vector
    // Find completed lines and add to buffer
}
```

## Performance Optimization Techniques

### Static Dispatch

```rust
// Prefer static dispatch for performance-critical code
trait Renderable {
    fn render(&self, renderer: &Renderer);
}

// Use impl Trait for static dispatch
fn render_game_elements(elements: &[impl Renderable], renderer: &Renderer) {
    for element in elements {
        element.render(renderer);
    }
}
```

### Avoiding Unnecessary Clones

```rust
// Use references instead of cloning
fn preview_piece_position(board: &Board, piece: &Tetromino) -> bool {
    // Implementation using references
}

// When needed, consider Cow for efficient handling
use std::borrow::Cow;

fn get_piece_display_name(piece: &Tetromino) -> Cow<'static, str> {
    match piece.shape {
        TetrominoShape::I => Cow::Borrowed("I-Piece"),
        TetrominoShape::Custom(ref name) => Cow::Owned(format!("Custom: {}", name)),
        // Other cases...
    }
}
```

### Fixed-Size Arrays and Const Generics

```rust
// Use const generics for board dimensions
struct Board<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[CellState; WIDTH]; HEIGHT],
}

// Standard Tetris board
type StandardBoard = Board<10, 20>;
```

## Mobile-Specific Considerations

### Efficient Touch Input Handling

```rust
// Process input events efficiently
fn handle_touch_input(touch_event: TouchEvent) -> GameAction {
    match touch_event.phase {
        TouchPhase::Started => handle_touch_start(touch_event),
        TouchPhase::Moved => {
            // Only process meaningful movements (optimization)
            if touch_event.distance_moved() > MOVEMENT_THRESHOLD {
                handle_touch_move(touch_event)
            } else {
                GameAction::None
            }
        }
        TouchPhase::Ended => handle_touch_end(touch_event),
    }
}
```

### Battery-Conscious Design

```rust
// Implement frame skipping for battery efficiency
fn update_game_state(game: &mut Game, delta_time: f32) {
    game.accumulated_time += delta_time;
    
    // Only update at fixed intervals
    while game.accumulated_time >= FIXED_TIME_STEP {
        perform_game_update(game);
        game.accumulated_time -= FIXED_TIME_STEP;
    }
}
```

### Minimizing System Calls

```rust
// Batch rendering operations
fn render_board(board: &Board, renderer: &mut Renderer) {
    // Begin batch to minimize GPU calls
    renderer.begin_batch();
    
    // Add all cells to the batch
    for y in 0..board.height {
        for x in 0..board.width {
            if let Some(cell_type) = board.get_cell(x, y) {
                renderer.add_to_batch(cell_type, x, y);
            }
        }
    }
    
    // Single draw call
    renderer.end_batch_and_render();
}
```

## Testing Strategies

### Unit Testing Game Logic

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tetromino_rotation() {
        let mut piece = Tetromino::new(TetrominoShape::L);
        let original_cells = piece.get_cells();
        
        piece.rotate_clockwise();
        
        // After 4 rotations, should be back to original position
        for _ in 0..3 {
            piece.rotate_clockwise();
        }
        
        assert_eq!(original_cells, piece.get_cells());
    }
    
    #[test]
    fn test_line_clearing() {
        let mut board = Board::new(10, 20);
        
        // Fill a line
        for x in 0..10 {
            board.set_cell(x, 19, CellType::Filled);
        }
        
        let lines_cleared = board.clear_completed_lines();
        
        assert_eq!(lines_cleared, 1);
        // Check the board state after clearing
        for x in 0..10 {
            assert_eq!(board.get_cell(x, 19), CellType::Empty);
        }
    }
}
```

### Property-Based Testing

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_board_bounds_check(x in 0..15u32, y in 0..25u32) {
            let board = Board::new(10, 20);
            
            // Board should correctly report if coordinates are in bounds
            let expected = x < 10 && y < 20;
            assert_eq!(board.is_in_bounds(x, y), expected);
        }
    }
}
```

### Integration Testing

```rust
#[test]
fn test_game_initialization() {
    let game = Game::new().expect("Failed to initialize game");
    
    assert!(game.board.is_empty());
    assert!(game.current_piece.is_some());
    assert_eq!(game.score.value, 0);
    // More assertions...
}
```
## Threading Strategy

### Multi-Threading Optimization

```rust
// Configure thread pool based on available cores
fn initialize_thread_pool() -> ThreadPool {
    let num_cores = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(2);
    
    // Use at least one thread, but scale based on device capabilities
    let thread_count = std::cmp::max(1, num_cores - 1);
    
    ThreadPool::new(thread_count)
}
```

### Work Distribution Strategy

```rust
// Distribute appropriate workloads across threads
struct GameEngine {
    thread_pool: ThreadPool,
    // Other engine components...
}

impl GameEngine {
    fn update(&mut self, delta_time: f32) {
        // Main game logic remains on primary thread
        self.update_game_state(delta_time);
        
        // Offload appropriate parallel tasks
        self.thread_pool.execute(|| {
            // Particle effects, AI calculations, 
            // physics simulations, etc.
        });
    }
}
```

### Thread Synchronization

```rust
// Carefully manage shared state between threads
use std::sync::{Arc, Mutex};

struct ParticleSystem {
    particles: Arc<Mutex<Vec<Particle>>>,
}

impl ParticleSystem {
    fn update_particles(&self) {
        self.thread_pool.execute({
            let particles = Arc::clone(&self.particles);
            move || {
                let mut particles = particles.lock().unwrap();
                // Update particles in parallel
            }
        });
    }
}
```

## Common Pitfalls and Solutions

1. **Borrow Checker Challenges**: Use clear ownership boundaries between game components.

2. **Excessive Cloning**: Implement game logic to work with references when possible.

3. **Premature Optimization**: Build for correctness first, then optimize hot paths.

4. **Error Recovery**: Design the game to recover gracefully from non-critical errors.

