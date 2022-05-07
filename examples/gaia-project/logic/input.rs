use bevy::prelude::*;

use crate::types::*;
use crate::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Running).with_system(input_monitor))
            .add_event::<PowerEvent>()
            .add_event::<GaugeEvent>();
    }
}

fn input_monitor(
    input: Res<Input<KeyCode>>,
    mut power_events: EventWriter<PowerEvent>,
    mut gauge_events: EventWriter<GaugeEvent>,
) {
    for &(key, amount) in [
        (KeyCode::Key1, 1),
        (KeyCode::Key2, 2),
        (KeyCode::Key3, 3),
        (KeyCode::Key4, 4),
        (KeyCode::Key5, 5),
        (KeyCode::Key6, 6),
        (KeyCode::Key7, 7),
        (KeyCode::Key8, 8),
        (KeyCode::Key9, 9),
    ]
    .iter()
    {
        if input.just_pressed(key) {
            power_events.send(PowerEvent::Charge(amount));
        }
    }

    if input.just_pressed(KeyCode::R) {
        power_events.send(PowerEvent::Reserve(1));
    }

    if input.just_pressed(KeyCode::S) {
        power_events.send(PowerEvent::Spend(4));
    }

    if input.just_pressed(KeyCode::D) {
        power_events.send(PowerEvent::Discard(1));
    }

    if input.just_pressed(KeyCode::A) {
        power_events.send(PowerEvent::Add(1));
    }

    if input.just_pressed(KeyCode::F) {
        power_events.send(PowerEvent::Force(1));
    }

    for &(key, resource) in [
        (KeyCode::O, Resource::Ore),
        (KeyCode::K, Resource::Knowledge),
        (KeyCode::C, Resource::Credit),
        (KeyCode::Q, Resource::Qic),
    ]
    .iter()
    {
        if input.just_pressed(key) {
            gauge_events.send(GaugeEvent::Gain(Cost {
                resource,
                amount: 1,
            }));
        }
    }
}
