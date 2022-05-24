use std::collections::HashMap;

use bevy::prelude::*;

use crate::types::*;
use crate::GameState;

#[derive(Default)]
pub struct AssetLibrary {
    fonts: HashMap<String, Handle<Font>>,
    planet_atlas: Handle<TextureAtlas>,
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

    pub fn sprite_sheet_bundle_for_planet_type(
        &self,
        planet_type: PlanetType,
    ) -> SpriteSheetBundle {
        SpriteSheetBundle {
            texture_atlas: self.planet_atlas.clone(),
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.3)).with_translation(Vec3::new(
                -360. + 80. * (planet_type as usize) as f32,
                0.,
                0.,
            )),
            sprite: TextureAtlasSprite::new(planet_type as usize),
            ..Default::default()
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    [("game", "fonts/MartianMono-StdRg.ttf")]
        .iter()
        .for_each(|&(title, file_location)| {
            asset_library
                .fonts
                .insert(title.into(), asset_server.load(file_location));
        });
    println!("Loaded fonts: {:?}", asset_library.fonts);

    let planet_texture = asset_server.load("sprites/planets.png");
    asset_library.planet_atlas = texture_atlases.add(TextureAtlas::from_grid(
        planet_texture,
        Vec2::new(256., 256.),
        4, // columns
        4, // rows
    ));

    game_state.set(GameState::Running).unwrap();
}
