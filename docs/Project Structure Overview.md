# Structure Overview

## Project Directory Structure

```
tetris-rust/
├── Cargo.toml           # Project dependencies and metadata
├── src/
│   ├── main.rs          # Application entry point
│   ├── game.rs          # Game loop and state management
│   ├── components/      # Core game components
│   │   ├── tetromino.rs # Tetromino shapes and rotation logic
│   │   ├── board.rs     # Game board and collision detection
│   │   ├── scoring.rs   # Score tracking and level progression
│   ├── rendering/       # Visual rendering components
│   │   ├── shapes.rs    # Lyon and Tiny-skia rendering utilities
│   │   ├── effects.rs   # Visual effects (particles, animations)
│   ├── ui/              # User interface components
│   │   ├── menus.rs     # Game menus (start, pause, settings)
│   │   ├── hud.rs       # In-game heads-up display
│   ├── audio/           # Sound and music components
│   │   ├── aeffects.rs   # Sound effect management
│   │   ├── music.rs     # Background music management
│   ├── input/           # Input handling
│   │   ├── touch.rs     # Touch controls for mobile
│   ├── utils/           # Utility functions
│   │   ├── config.rs    # Game configuration
│   │   ├── debug.rs     # Debugging utilities
├── assets/              # Game assets
│   ├── audio/           # Sound effects and music
│   ├── fonts/           # Text fonts
├── android/             # Android-specific files
│   ├── AndroidManifest.xml  # Android configuration
│   ├── build.gradle     # Android build configuration
├── tests/               # Test modules
```

## Core Component Structure

### Game Core (game.rs)
- **GameState**: Enum defining different game states (Menu, Playing, Paused, GameOver)
- **Game**: Main struct coordinating the game components and managing the game loop
- **update()**: Updates game state based on input and game rules
- **draw()**: Coordinates rendering of all game elements

### Tetromino System (components/tetromino.rs)
- **TetrominoType**: Enum of the seven standard Tetris pieces
- **Tetromino**: Struct containing position, rotation, and type
- **rotation_system**: Implementation of the Super Rotation System
- **spawn_tetromino()**: Creates new tetrominoes at the top of the board

### Board System (components/board.rs)
- **Board**: 2D grid representation of the game board
- **check_collision()**: Collision detection between pieces and board
- **clear_lines()**: Logic for detecting and clearing completed lines
- **lock_piece()**: Solidifies a tetromino into the board

### Rendering Pipeline (rendering/shapes.rs)
- Integration with Lyon for vector shape creation
- Integration with Tiny-skia for rendering
- **draw_tetromino()**: Renders active and ghost tetrominoes
- **draw_board()**: Renders the game board and locked pieces

### Touch Input System (input/touch.rs)
- **TouchHandler**: Manages and processes touch inputs
- **swipe_detection()**: Logic for handling swipe gestures
- **tap_detection()**: Logic for handling taps

## Initialization Flow
1. Application starts in main.rs
2. Game creates and initializes all components
3. Asset loading occurs during initialization
4. Main game loop begins, alternating between update and draw cycles

## Build and Deployment
- Development builds use cargo run
- Android builds processed through cargo-apk
- Asset bundling handled via build.rs

