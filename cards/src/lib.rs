use bevy::ecs::system::Resource;
use bevy::prelude::*;

mod card;
pub mod deck;
mod pile;
pub mod standard_playing_card;
mod test;

pub use deck::Deck;
pub use pile::Pile;
pub use standard_playing_card::StandardPlayingCard;

pub struct CardsPlugin<T> {
    deck: Deck<T>,
}

impl<T> CardsPlugin<T> {
    pub fn new(deck: Deck<T>) -> Self {
        Self { deck }
    }
}

impl<T: 'static + Clone + Resource + Send + Sync + ToString> Plugin for CardsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.deck.clone())
            .insert_resource(Pile::<T>::default())
            .add_startup_system(init_pile)
            .add_system(deck_system::<T>)
            .add_system(pile_view_system::<T>);
    }
}

#[derive(Component)]
struct PileView;

fn init_pile(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/ClassicConsoleNeue.ttf");
    let text_style = TextStyle {
        font,
        font_size: 48.0,
        color: Color::WHITE,
    };

    commands.spawn(Camera2dBundle::new_with_far(-0.1));

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("INITIAL", text_style),
            ..Default::default()
        })
        .insert(PileView);
}

fn deck_system<T: Clone + Resource + ToString>(
    input: Res<Input<KeyCode>>,
    mut deck: ResMut<Deck<T>>,
    mut pile: ResMut<Pile<T>>,
) {
    if input.just_pressed(KeyCode::Return) {
        if let Ok(card) = deck.deal_one() {
            println!("Dealt: {}", card.to_string());
            pile.add(card);
        } else {
            deck.reset();
            deck.shuffle();
        }
    }
}

fn pile_view_system<T: Clone + Resource + ToString>(
    pile: Res<Pile<T>>,
    mut query: Query<&mut Text, With<PileView>>,
) {
    for mut text in query.iter_mut() {
        if let Some(card) = pile.peek() {
            text.sections[0].value = card.to_string();
        }
    }
}
