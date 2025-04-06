use bevy::prelude::*;
use once_cell::sync::Lazy;

// Define our tetromino colors based on the style guide
pub static COLORS: Lazy<[Color; 7]> = Lazy::new(|| [
    Color::hex("00CCCC").unwrap(),  // I - Cyan
    Color::hex("FFCC00").unwrap(),  // O - Yellow
    Color::hex("9900CC").unwrap(),  // T - Purple
    Color::hex("00CC00").unwrap(),  // S - Green
    Color::hex("FF3333").unwrap(),  // Z - Red
    Color::hex("0000CC").unwrap(),  // J - Blue
    Color::hex("FF9900").unwrap(),  // L - Orange
]);

// Tetromino types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoType {
    // Get the index for colors and shapes arrays
    pub fn index(&self) -> usize {
        match self {
            TetrominoType::I => 0,
            TetrominoType::O => 1,
            TetrominoType::T => 2,
            TetrominoType::S => 3,
            TetrominoType::Z => 4,
            TetrominoType::J => 5,
            TetrominoType::L => 6,
        }
    }
    
    // Get color for this tetromino type
    pub fn color(&self) -> Color {
        COLORS[self.index()]
    }
    
    // Get shape data for this tetromino type
    pub fn shape(&self) -> &'static [[bool; 4]; 4] {
        &SHAPES[self.index()]
    }
    
    // Get a random tetromino type
    pub fn random() -> Self {
        let index = rand::random::<usize>() % 7;
        match index {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::S,
            4 => TetrominoType::Z,
            5 => TetrominoType::J,
            _ => TetrominoType::L,
        }
    }
}

// Define tetromino shapes
pub const SHAPES: [[[bool; 4]; 4]; 7] = [
    // I
    [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // O
    [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    // T
    [
        [false, false, false, false],
        [false, true, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    // S
    [
        [false, false, false, false],
        [false, true, true, false],
        [true, true, false, false],
        [false, false, false, false],
    ],
    // Z
    [
        [false, false, false, false],
        [true, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    // J
    [
        [false, false, false, false],
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    // L
    [
        [false, false, false, false],
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
];

// Components for tetrominos
#[derive(Component)]
pub struct Tetromino {
    pub tetromino_type: TetrominoType,
    pub rotation: usize, // 0, 1, 2, 3 (0, 90, 180, 270 degrees)
}

#[derive(Component)]
pub struct FallingTetromino {
    pub speed: f32,
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct ActiveTetromino;

#[derive(Component)]
pub struct GhostTetromino;

#[derive(Component)]
pub struct NextTetromino;

#[derive(Component)]
pub struct HeldTetromino;

// Spawn a tetromino entity with its blocks as children
pub fn spawn_tetromino(
    commands: &mut Commands,
    tetromino_type: TetrominoType,
    position: Vec3,
    block_size: f32,
    falling: Option<FallingTetromino>,
) -> Entity {
    let color = tetromino_type.color();

    let tetromino_entity = commands.spawn((
        TransformBundle {
            local: Transform::from_translation(position),
            ..default()
        },
        VisibilityBundle::default(),
        Tetromino {
            tetromino_type,
            rotation: 0,
        },
    )).id();

    if let Some(falling_component) = falling {
        commands.entity(tetromino_entity).insert(falling_component);
    }

    let shape = tetromino_type.shape();
    for y in 0..4 {
        for x in 0..4 {
            if shape[y][x] {
                let block_x = (x as f32 - 1.5) * block_size;
                let block_y = (y as f32 - 1.5) * block_size;

                commands.entity(tetromino_entity).with_children(|parent| {
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::new(block_size - 1.0, block_size - 1.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(block_x, -block_y, 0.0),
                        ..default()
                    });
                });
            }
        }
    }

    tetromino_entity
}
