use bevy::prelude::*;

use crate::types::*;
use crate::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Running).with_system(input_monitor))
            .add_event::<PowerEvent>();
    }
}

fn input_monitor(input: Res<Input<KeyCode>>, mut events: EventWriter<PowerEvent>) {
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
            events.send(PowerEvent::Charge(amount));
        }
    }

    if input.just_pressed(KeyCode::R) {
        events.send(PowerEvent::Reserve(1));
    }

    if input.just_pressed(KeyCode::S) {
        events.send(PowerEvent::Spend(4));
    }

    if input.just_pressed(KeyCode::D) {
        events.send(PowerEvent::Discard(1));
    }

    if input.just_pressed(KeyCode::A) {
        events.send(PowerEvent::Add(1));
    }

    if input.just_pressed(KeyCode::F) {
        events.send(PowerEvent::Force(1));
    }
}
