use std::collections::HashMap;

use bevy::prelude::*;

use crate::GameState;

#[derive(Default)]
pub struct AssetLibrary {
    fonts: HashMap<String, Handle<Font>>,
    sprites: HashMap<String, Handle<Sprite>>,
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

    pub fn sprite(&self, name: &str) -> Handle<Sprite> {
        match self.sprites.get(name) {
            Some(handle) => handle.clone(),
            None => panic!(
                "Could not find sprite: \'{}\' in sprite library: {:?}",
                name,
                self.sprites.keys().collect::<Vec<&String>>()
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
    [("game", "fonts/MartianMono-StdRg.ttf")]
        .iter()
        .for_each(|&(title, file_location)| {
            asset_library
                .fonts
                .insert(title.into(), asset_server.load(file_location));
        });
    println!("Loaded fonts: {:?}", asset_library.fonts);

    [
        ("planet-red", "sprites/planet-red.png"),
        ("planet-yellow", "sprites/planet-yellow.png"),
        ("planet-orange", "sprites/planet-orange.png"),
        ("planet-grey", "sprites/planet-grey.png"),
        ("planet-white", "sprites/planet-white.png"),
        ("planet-brown", "sprites/planet-brown.png"),
        ("planet-blue", "sprites/planet-blue.png"),
        ("planet-gaia", "sprites/planet-gaia.png"),
        ("planet-transdim", "sprites/planet-transdim.png"),
    ]
    .iter()
    .for_each(|&(title, file_location)| {
        asset_library
            .sprites
            .insert(title.into(), asset_server.load(file_location));
    });
    println!("Loaded sprites: {:?}", asset_library.sprites);

    game_state.set(GameState::Running).unwrap();
}
