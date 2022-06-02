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
        .run();
}
