use bevy::prelude::*;

mod game;
mod components;
mod systems;
mod utils;

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
        .add_plugins(game::GamePlugin)
        .run();
}
