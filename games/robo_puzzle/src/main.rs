#![allow(dead_code)]

use bevy::prelude::*;

mod command_tile;

use command_tile::CommandTilePlugin;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Robot Puzzle Game".to_string(),
            width: 1280f32,
            height: 720f32,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(CommandTilePlugin)
        .run();
}
