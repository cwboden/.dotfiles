#![allow(dead_code)]

use bevy::prelude::*;
use input::select::SelectPlugin;

mod asset_library;
mod logic;
mod types;
mod view;

use asset_library::AssetLibraryPlugin;
use logic::LogicPlugin;
use view::ViewPlugin;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    Loading,
    Running,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1080.,
            height: 810.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetLibraryPlugin)
        .add_plugin(ViewPlugin)
        .add_plugin(LogicPlugin)
        .add_plugin(SelectPlugin)
        .add_startup_system(init)
        .add_state(GameState::Loading)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
