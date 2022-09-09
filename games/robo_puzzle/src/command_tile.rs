use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandType {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Stop,
}

/// A single CommandTile that tells the robot how it should move
#[derive(Component)]
pub struct CommandTile {
    command_type: CommandType,
}

/// A source of CommandTiles that the player can drag-and-drop from to program
/// their robot.
#[derive(Component)]
pub struct CommandTileSource {
    command_type: CommandType,
}

fn spawn_sources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let commands_and_textures = vec![
        (
            CommandType::MoveUp,
            asset_server.load("texture/move-up-tile.png"),
        ),
        (
            CommandType::MoveDown,
            asset_server.load("texture/move-down-tile.png"),
        ),
        (
            CommandType::MoveLeft,
            asset_server.load("texture/move-left-tile.png"),
        ),
        (
            CommandType::MoveRight,
            asset_server.load("texture/move-right-tile.png"),
        ),
    ];

    let mut y_pos = 0f32;
    let tile_sprite_dimension = 64f32;
    let tile_padding = 16f32;
    for (command_type, texture) in commands_and_textures {
        commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform::from_translation(Vec3::new(256f32, y_pos, 1f32)),
                ..Default::default()
            })
            .insert(CommandTileSource { command_type });

        y_pos += tile_sprite_dimension + tile_padding;
    }
}

/// An element in the routine of CommandTiles which will be executed by the
/// robot.
#[derive(Component, Default)]
pub struct RoutineCommand {
    command_tile: Option<CommandTile>,
}

impl RoutineCommand {
    const NUM_TILES: u8 = 8;
}

fn spawn_routine(mut commands: Commands, asset_server: Res<AssetServer>) {
    let routine_sprite_dimension = 64f32;
    let routine_padding = 16f32;

    let mut x_pos = -256f32;
    for _ in 0..RoutineCommand::NUM_TILES {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("texture/empty-slot.png"),
                transform: Transform::from_translation(Vec3::new(x_pos, -128f32, 1f32)),
                ..Default::default()
            })
            .insert(RoutineCommand::default());

        x_pos += routine_sprite_dimension + routine_padding;
    }
}

/// Plugin which spawns available CommandTileSources for the player to interact
/// with, and the Routine, which players add CommandTiles to be executed to.
pub struct CommandTilePlugin;

impl Plugin for CommandTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_sources)
            .add_startup_system(spawn_routine);
    }
}
