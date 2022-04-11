use crate::logic::power::PowerCycleTracker;
use crate::types::*;
use bevy::prelude::*;

pub struct PowerViewPlugin;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PowerEvent {
    Add(u8),
    Charge(u8),
    Discard(u8),
    Force(u8),
    Reserve(u8),
    Spend(u8),
}

pub struct PowerViewState {
    pub tracker: PowerCycleTracker,
}

#[derive(Component)]
pub struct PowerView;

pub fn power_view(
    mut view_state: ResMut<PowerViewState>,
    mut events: EventReader<PowerEvent>,
    mut query: Query<&mut Text, With<PowerView>>,
) {
    for &event in events.iter() {
        match event {
            PowerEvent::Charge(amount) => view_state.tracker.charge(amount),
            PowerEvent::Reserve(amount) => view_state.tracker.reserve(amount).unwrap(),
            PowerEvent::Spend(amount) => view_state.tracker.spend(amount).unwrap(),
            PowerEvent::Discard(mut amount) => {
                for &bowl in [
                    PowerBowl::Gaia,
                    PowerBowl::One,
                    PowerBowl::Two,
                    PowerBowl::Three,
                ]
                .iter()
                {
                    let bowl_amount = view_state.tracker.get(bowl);
                    let discard_amount = std::cmp::min(bowl_amount, amount);
                    view_state.tracker.discard(bowl, discard_amount).unwrap();

                    amount -= discard_amount;
                }

                // We should have discarded all input power
                assert_eq!(amount, 0);
            }
            PowerEvent::Add(amount) => view_state.tracker.add(amount),
            PowerEvent::Force(amount) => view_state.tracker.force(amount).unwrap(),
        }
    }

    for mut text in query.iter_mut() {
        text.sections[1].value = format!("Bowl 1: {}\n", view_state.tracker.get(PowerBowl::One));
        text.sections[2].value = format!("Bowl 2: {}\n", view_state.tracker.get(PowerBowl::Two));
        text.sections[3].value = format!("Bowl 3: {}\n", view_state.tracker.get(PowerBowl::Three));
        text.sections[4].value = format!("Bowl G: {}\n", view_state.tracker.get(PowerBowl::Gaia));
    }
}
