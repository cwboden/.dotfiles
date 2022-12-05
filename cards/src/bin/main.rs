use bevy::prelude::*;
use cards::{CardsPlugin, Deck, StandardPlayingCard};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1080.,
                height: 810.,
                ..default()
            },
            ..default()
        }))
        .add_plugin(CardsPlugin::new(Deck::new(&StandardPlayingCard::ALL)))
        .run();
}
