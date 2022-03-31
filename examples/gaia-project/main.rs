use bevy::prelude::*;
mod asset_library;
mod power;
mod types;

use asset_library::AssetLibraryPlugin;

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
        .run();
}
