# Development Roadmap

## Phase 1: Setup and Foundation
**Timeframe: 1-2 weeks**

- Bevy environment setup and configuration
- Project structure initialization
- State management implementation
- Basic ECS component definitions
- Core systems skeleton
- Window configuration

## Phase 2: Core Game Mechanics
**Timeframe: 2-3 weeks**

- Board and tetromino data structures as components
- Movement and rotation systems
- Collision detection system
- Line clearing implementation
- Super Rotation System (SRS) implementation
- Scoring system and level progression
- Game over conditions

## Phase 3: Visual Implementation
**Timeframe: 2-3 weeks**

- Lyon integration with Bevy for vector graphics
- Tetromino and board rendering systems
- Animation systems
- UI implementation using Bevy UI
  - Main menu
  - Game HUD
  - Pause menu
  - Game over screen
- Visual feedback effects
- Shader-based effects for visual polish

## Phase 4: Desktop-specific Features
**Timeframe: 1-2 weeks**

- Window management (resizing, fullscreen)
- Keyboard control customization
- Input system refinement
- Multiple resolution support
- Performance optimization
- Settings persistence and configuration

## Phase 5: Audio and Enhanced Features
**Timeframe: 1-2 weeks**

- Bevy Kira Audio integration
- Sound effect implementation
- Background music systems
- Ghost piece visualization
- Next piece preview
- Hold piece functionality
- Special move detection and visual feedback

## Phase 6: Polish and Distribution
**Timeframe: 1-2 weeks**

- Final visual refinements
- Performance optimization
- Bug fixing
- Create installers or portable packages for distribution
- Settings persistence between sessions

**Total Project Duration: 8-14 weeks of active development**

## Future Expansion Possibilities
**After initial desktop release**

- Local multiplayer support using Bevy's ECS for multiple players
- Custom themes and skins through Bevy's asset system
- Level editor
- High score leaderboards
- Alternative game modes (Marathon, Sprint, Ultra)
- Cross-platform support (Linux, macOS)
- Particle system enhancements for visual effects