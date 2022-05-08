use bevy::prelude::*;

use crate::types::*;
use crate::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Running).with_system(input_monitor))
            .add_event::<PowerEvent>()
            .add_event::<GaugeEvent>()
            .add_event::<CoverActionEvent>();
    }
}

fn input_monitor(
    input: Res<Input<KeyCode>>,
    mut power_events: EventWriter<PowerEvent>,
    mut gauge_events: EventWriter<GaugeEvent>,
    mut cover_action_events: EventWriter<CoverActionEvent>,
) {
    for &(key, action_type) in [
        (KeyCode::Key1, CoverActionType::GainThreePower),
        (KeyCode::Key2, CoverActionType::SingleTerraform),
        (KeyCode::Key3, CoverActionType::TwoKnowledge),
        (KeyCode::Key4, CoverActionType::SevenCredits),
        (KeyCode::Key5, CoverActionType::TwoOre),
        (KeyCode::Key6, CoverActionType::DoubleTerraform),
        (KeyCode::Key7, CoverActionType::ThreeKnowledge),
        (KeyCode::Key8, CoverActionType::PointsForPlanetTypes),
        (KeyCode::Key9, CoverActionType::RescoreFederationToken),
        (KeyCode::Key0, CoverActionType::GainTechTile),
    ]
    .iter()
    {
        if input.just_pressed(key) {
            cover_action_events.send(CoverActionEvent::Cover(action_type));
        }
    }

    if input.just_pressed(KeyCode::Up) {
        power_events.send(PowerEvent::Charge(1));
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

    if input.just_pressed(KeyCode::Z) {
        cover_action_events.send(CoverActionEvent::Reset);
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
