use bevy::ecs::system::Resource;
use bevy::input::InputPlugin;
use bevy::prelude::*;

mod card;
mod deck;
mod pile;
mod standard_playing_card;
mod test;

use deck::Deck;
use pile::Pile;

pub struct CardsPlugin<T> {
    deck: Deck<T>,
}

impl<T> CardsPlugin<T> {
    pub fn new(deck: Deck<T>) -> Self {
        Self { deck }
    }
}

impl<T: 'static + Clone + Resource + Send + Sync> Plugin for CardsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.deck.clone())
            .insert_resource(Pile::<T>::default())
            .add_plugin(InputPlugin)
            .add_system(deck_view::<T>);
    }
}

fn deck_view<T: Clone + Resource>(
    input: Res<Input<KeyCode>>,
    mut deck: ResMut<Deck<T>>,
    mut pile: ResMut<Pile<T>>,
) {
    if input.just_pressed(KeyCode::Return) {
        let card = deck.deal_one().unwrap();
    }
}
