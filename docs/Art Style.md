# RustBlocks Art Style Guide

## Core Concept
RustBlocks combines nostalgic pixel art aesthetics with modern visual effects and animations. The game presents a neo-retro experience where classic Tetris gameplay meets contemporary design sensibilities, delivering an experience that feels both familiar and fresh.

## Color Palette

### Tetromino Colors (Colorblind-Friendly)
- **I-piece (Cyan)**: `#00CCCC` - Bright, distinctive blue
- **O-piece (Yellow)**: `#FFCC00` - Vibrant yellow
- **T-piece (Purple)**: `#9900CC` - Rich purple
- **S-piece (Green)**: `#00CC00` - Bright green
- **Z-piece (Red)**: `#FF3333` - Bright red adjusted for deuteranopia
- **J-piece (Blue)**: `#0000CC` - Deep blue
- **L-piece (Orange)**: `#FF9900` - Warm orange

### UI and Environment Colors
- **Primary Background**: `#111133` - Deep space blue
- **Secondary Background**: `#221144` - Slightly lighter for contrast
- **UI Accents - Primary**: `#00FFAA` - Neon cyan/green
- **UI Accents - Secondary**: `#FF00AA` - Neon magenta
- **UI Text - Primary**: `#FFFFFF` - White
- **UI Text - Secondary**: `#BBBBBB` - Light gray
- **Grid Lines**: `#333366` - Subtle blue
- **Border Highlights**: `#9966FF` - Soft purple highlight

## Typography

### Font Selection
- **Primary Display Font**: "VT323" - Perfect for headings, scores, and prominent text
  - Used for: Game title, scores, level indicators, menu headings
  - Style: Pixelated, chunky, strong retro aesthetic

- **Secondary Font**: "Share Tech Mono" - Clean monospace for better readability
  - Used for: Instructions, settings, smaller UI elements, credits
  - Style: Modern monospace with slight technical feel

### Text Guidelines
- Use uppercase for main headings
- Maintain consistent spacing between letters
- Add pixel-perfect drop shadows where needed for legibility
- Limit text animations to avoid distracting from gameplay

## Visual Elements

### Tetromino Design
- **Style**: Pixel blocks with subtle rounded corners
- **Effects**: Slight inner glow in the tetromino's color
- **Border**: 1px dark outline to define shape clearly
- **Special Effects**: Brief flash on rotation, subtle trail when hard dropping

### Game Board
- **Grid**: Thin, subtle lines (`#333366`)
- **Background**: Dark with barely visible scan lines effect
- **Border**: Highlighted pixel border with slight pulsing effect
- **Ghost Piece**: Semi-transparent outline showing landing position

### Backgrounds and Environments
- **Style**: AI-generated pixel art landscapes
- **Progression**: Evolve as player advances through levels
- **Animation**: Subtle parallax scrolling for depth
- **Themes**: Cosmic, cyberpunk, retrowave aesthetics

### Particles and Effects
- **Line Clear**: Pixelated explosion with modern glow
- **Level Up**: Screen flash with scanline warp effect
- **Game Over**: CRT-style static interference transitioning to score screen
- **Tetromino Lock**: Small pixel burst at lock point

## Animation Principles

### Tetromino Movement
- **Drop Speed**: Smooth acceleration based on level
- **Horizontal Movement**: Quick with subtle inertia (3-5 frames)
- **Rotation**: Snappy (2 frames) with minimal squash effect
- **Hard Drop**: Quick with trail effect
- **Lock Animation**: Brief flash and settle effect (2-3 frames)

### UI Animations
- **Menu Transitions**: Smooth slide with pixel scatter effect
- **Button Highlights**: Subtle pulse or glow
- **Score Updates**: Number roll-up with brief highlight
- **Notifications**: Slide in with slight bounce

### Special Effects
- **Line Clear Animation**: 
  - Single line: Simple flash and collapse
  - Multiple lines: Intensified effect with particle bursts
  - Tetris (4 lines): Screen shake and maximum particles

## Screen Layout

### Main Menu
- **Logo**: Large, centered at top third
- **Menu Options**: Centered below logo
- **Animated Background**: Full screen with parallax effect
- **Decorative Elements**: Falling tetrominos in background

### Game Screen
- **Board**: Centered, prominent
- **Next Piece**: Upper right
- **Hold Piece**: Upper left
- **Score/Level**: Top center
- **Controls**: Bottom or side, minimal visibility

### Pause/Game Over
- **Overlay**: Semi-transparent dark overlay with pixel texture
- **Text**: Centered, large VT323 font
- **Options**: Centered below text
- **Stats**: Display on Game Over screen

## Implementation Notes

- Use shader effects for glow and scan lines rather than pre-rendered assets
- Implement smooth animation transitions between states
- Ensure particle effects are optimized and don't impact gameplay
- Make visual effects toggleable for accessibility
- Maintain consistent pixel scale across all elements