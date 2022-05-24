use bevy::prelude::*;
use dotfiles::cards::{CardsPlugin, Deck, StandardPlayingCard};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1080.,
            height: 810.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(CardsPlugin::new(Deck::new(&StandardPlayingCard::ALL)))
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
