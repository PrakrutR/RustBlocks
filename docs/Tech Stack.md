# Tech Stack Reference (Desktop Focus)

## Core Language: Rust

**Version:** 1.74+ (Stable)

**Key Features Used:**
- Memory safety without garbage collection
- Pattern matching for game state management
- Zero-cost abstractions for performance-critical code
- Type system for representing game entities

**Usage Patterns:**
- Ownership model for managing game state
- Error handling with Result for robust operation
- Traits for component behavior standardization
- Enums for state representation (e.g., tetromino types)

## Game Framework: Macroquad

**Version:** 0.4.x

**Key Features Used:**
- Window and graphics context handling
- Game loop with timing controls
- Input handling for keyboard and mouse
- Asset loading utilities
- Desktop window management

**Integration Approach:**
- Using Macroquad's main loop via `#[macroquad::main]`
- Leveraging the framework's window management
- Utilizing built-in timing functions for consistent game speed
- Handling keyboard input events for game control

**Desktop Features:**
- Custom window sizing and configuration
- Support for multiple screen resolutions
- Fullscreen and windowed mode options
- Keyboard control schemes with customizable bindings

## Graphics Rendering: Lyon + Tiny-skia

**Lyon Version:** 1.0.x
**Tiny-skia Version:** 0.8.x

**Key Features Used:**
- Lyon: Path building and tessellation for vector graphics
- Tiny-skia: High-quality rendering of vector graphics

**Integration Approach:**
- Lyon for creating tetromino shape geometry
- Tiny-skia for high-quality rendering
- Custom wrapper for managing the rendering pipeline

**Desktop Enhancements:**
- Higher-resolution assets and rendering
- More complex visual effects leveraging desktop GPU capabilities
- Screen scaling options for different monitor sizes
- Enhanced particle effects and animations

## UI Framework: egui

**Version:** 0.24.x

**Key Features Used:**
- Immediate mode GUI for game menus
- Mouse-friendly widgets
- Styling customization
- Window and dialog management

**Integration Approach:**
- Wrapped within Macroquad's main loop
- Custom rendering pass for UI elements
- Desktop-focused input handling

**Desktop UI Features:**
- Mouse-driven menu navigation
- Keyboard shortcuts
- Context menus and tooltips
- Settings panels with more configuration options

## Audio Engine: kira

**Version:** 0.8.x

**Key Features Used:**
- Multi-channel audio playback
- Sound effect management
- Dynamic audio mixing
- Music streaming

**Integration Approach:**
- Initialization at game startup
- Encapsulated audio management system
- Event-based triggering of sound effects

**Desktop Audio Enhancements:**
- Higher quality audio files
- More complex mixing and effects processing
- Volume control and audio settings
- Support for external audio devices

## Development Tools

**rust-analyzer:**
- IDE integration for code completion and error checking
- Project-wide symbol navigation

**cargo-watch:**
- Automatic compilation on file changes
- Integration with development workflow

**cargo-insta:**
- Snapshot testing for game state validation
- Regression testing for critical game mechanics

## Performance Considerations

- Using release mode with optimizations (`--release`)
- Minimizing allocations during gameplay
- Implementing fixed timestep logic for consistent gameplay
- Texture atlas usage for efficient rendering
- Adaptive performance based on system capabilities

## Resource Management

- Assets loaded at startup to prevent mid-game loading
- Memory usage monitoring
- Efficient texture management for rendering pipeline
- Dynamic resource loading based on available system memory