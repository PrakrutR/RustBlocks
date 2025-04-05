# Structure Overview

## Project Directory Structure

```
rustblocks/
├── Cargo.toml           # Project dependencies and metadata
├── src/
│   ├── main.rs          # Application entry point and Bevy app configuration
│   ├── game/            # Game state and core loop management
│   │   ├── states.rs    # Game state definitions and transitions
│   │   ├── resources.rs # Global game resources
│   ├── components/      # ECS components
│   │   ├── tetromino.rs # Tetromino data components
│   │   ├── board.rs     # Game board components
│   │   ├── scoring.rs   # Score and level tracking
│   ├── systems/         # ECS systems
│   │   ├── input.rs     # Input handling systems
│   │   ├── gameplay.rs  # Core gameplay systems
│   │   ├── rendering.rs # Visual systems
│   │   ├── audio.rs     # Sound systems
│   ├── plugins/         # Bevy plugins for feature encapsulation
│   │   ├── game_plugin.rs  # Main game systems
│   │   ├── ui_plugin.rs    # User interface
│   │   ├── audio_plugin.rs # Sound management
│   ├── assets/          # Asset loading and management
│   ├── utils/           # Helper functions and utilities
├── assets/              # Game assets
│   ├── audio/           # Sound effects and music
│   ├── fonts/           # Text fonts
│   ├── images/          # Textures and visual assets
├── tests/               # Test modules
```

## Core Component Structure

### Entity Components (components/*.rs)
- **TetrominoComponent**: Contains tetromino type, color, and orientation data
- **PositionComponent**: Grid position for game entities
- **BoardCellComponent**: Individual cells that make up the game board
- **ScoreComponent**: Tracking scores and level progression
- **PlayerComponent**: Marker component for player-controlled entities

### Game States (game/states.rs)
- **AppState**: Main application states (MainMenu, InGame, Paused, GameOver)
- **GameState**: More granular game states (Spawning, Falling, Locking, LineClear)
- State transition logic and definitions

### Global Resources (game/resources.rs)
- **BoardResource**: Game board state data
- **ScoreResource**: Current score, level, and lines cleared
- **GameConfigResource**: Game settings and configuration
- **InputStateResource**: Current state of controls

### System Groups (systems/*.rs)
- **InputSystems**: Handle keyboard input and translation to game actions
- **GameplaySystems**: Core tetris mechanics (movement, rotation, collision, line clearing)
- **RenderingSystems**: Visual representation of game entities
- **AudioSystems**: Sound effect and music triggering

### Plugins (plugins/*.rs)
- **GamePlugin**: Core gameplay mechanics and state
- **UIPlugin**: User interface elements and interactions
- **AudioPlugin**: Sound management and playback

## Initialization Flow

1. Application starts in main.rs
2. Bevy app is configured with plugins, resources, and systems
3. Initial state is set to main menu
4. Bevy's ECS takes over running systems based on current state
5. State transitions drive the game flow

## Bevy-specific Architecture

### Entity-Component-System Pattern
- Entities: Represent game objects (tetrominos, board cells)
- Components: Data attached to entities (position, color, rotation)
- Systems: Logic that operates on entities with specific components
- Resources: Global state shared across systems

### State Management
- State-driven system execution using Bevy's state machine
- Systems can be conditioned to run only in specific states
- State transitions trigger entry/exit systems

### Event System
- Events used for decoupled communication between systems
- Key events: BlockPlaced, LineCleared, GameOver, ScoreChanged

### Asset Management
- Bevy's asset system handles loading and management of resources
- Fonts, textures, and audio loaded through AssetServer
- Asset handles passed through components or resources