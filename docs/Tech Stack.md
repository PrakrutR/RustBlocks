# Tech Stack Reference

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

**Integration Notes:**
- Using stable Rust features only to ensure compatibility
- Focused on idiomatic Rust patterns optimized for mobile

## Game Framework: Macroquad

**Version:** 0.4.x

**Key Features Used:**
- Cross-platform window and graphics context handling
- Game loop with timing controls
- Basic input handling foundation
- Asset loading utilities

**Integration Approach:**
- Using Macroquad's main loop via `#[macroquad::main]`
- Leveraging the framework's window management
- Utilizing built-in timing functions for consistent game speed
- Handling basic keyboard/touch input events

**Usage Patterns:**
- Custom draw calls that integrate with Lyon/Tiny-skia
- State-based game progression
- Frame time management for consistent gameplay

## Graphics Rendering: Lyon + Tiny-skia

**Lyon Version:** 1.0.x
**Tiny-skia Version:** 0.8.x

**Key Features Used:**
- Lyon: Path building and tessellation for vector graphics
- Tiny-skia: High-quality rendering of vector graphics

**Integration Approach:**
- Lyon for creating tetromino shape geometry
- Tiny-skia for GPU-accelerated rendering
- Custom wrapper for managing the rendering pipeline

**Usage Patterns:**
- Pre-generated shape definitions for tetrominoes
- Runtime transformation for piece movement and rotation
- Custom effects pipeline for visual polish

## UI Framework: egui

**Version:** 0.24.x

**Key Features Used:**
- Immediate mode GUI for game menus
- Touch-friendly widgets
- Styling customization

**Integration Approach:**
- Wrapped within Macroquad's main loop
- Custom rendering pass for UI elements
- Mobile-optimized input handling

**Usage Patterns:**
- Minimal UI approach focused on mobile usability
- Consistent styling across all menus
- Touch-friendly button sizes and spacing

## Audio Engine: kira

**Version:** 0.8.x

**Key Features Used:**
- Multi-channel audio playback
- Sound effect management
- Dynamic audio mixing

**Integration Approach:**
- Initialization at game startup
- Encapsulated audio management system
- Event-based triggering of sound effects

**Usage Patterns:**
- Preloaded sound effects for game actions
- Background music with adaptive volume
- Audio feedback tied to game events

## Mobile Integration: cargo-apk

**Version:** 0.9.x

**Key Features Used:**
- Android packaging and deployment
- Permission management
- Resource bundling

**Integration Approach:**
- Custom build script for Android packaging
- Asset bundling through resource directories
- Android manifest configuration

**Usage Patterns:**
- Development builds with debugging enabled
- Release builds with optimizations
- Screen orientation locking for gameplay

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
- Mobile-specific power usage optimizations

## Resource Management

- Assets loaded at startup to prevent mid-game loading
- Memory usage monitoring for mobile constraints
- Efficient texture management for rendering pipeline
