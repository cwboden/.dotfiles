use bevy::prelude::*;

mod asset_library;
mod power;
mod types;
mod view;

use asset_library::AssetLibraryPlugin;
use view::ViewPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 640.,
            height: 480.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetLibraryPlugin)
        .add_plugin(ViewPlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
