use crate::GameState;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct AssetLibrary {
    fonts: HashMap<String, Handle<Font>>,
}

impl AssetLibrary {
    pub fn font(&self, name: &str) -> Handle<Font> {
        match self.fonts.get(name) {
            Some(handle) => handle.clone(),
            None => panic!(
                "Could not find font: \'{}\' in font library: {:?}",
                name,
                self.fonts.keys().collect::<Vec<&String>>()
            ),
        }
    }
}

pub struct AssetLibraryPlugin;

impl Plugin for AssetLibraryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLibrary>()
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(init_assets));
    }
}

fn init_assets(
    mut game_state: ResMut<State<GameState>>,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    let fonts = vec![("game", "fonts/MartianMono-StdRg.ttf")];

    for &(title, file_location) in fonts.iter() {
        asset_library
            .fonts
            .insert(title.into(), asset_server.load(file_location));
    }

    println!("Loaded fonts: {:?}", asset_library.fonts);
    game_state.set(GameState::Running).unwrap();
}
