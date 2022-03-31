use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct AssetLibrary {
    fonts: HashMap<String, Handle<Font>>,
}

impl AssetLibrary {
    pub fn font(&self, name: &str) -> Handle<Font> {
        self.fonts.get(name).unwrap().clone()
    }
}

pub struct AssetLibraryPlugin;

impl Plugin for AssetLibraryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLibrary>()
            .add_startup_system(init_assets);
    }
}

fn init_assets(mut asset_library: ResMut<AssetLibrary>, asset_server: Res<AssetServer>) {
    let fonts = vec![("game", "fonts\\MartianMono-StdRg.ttf")];

    for &(title, file_location) in fonts.iter() {
        asset_library.fonts.insert(title.into(), asset_server.load(file_location));
    }
}
