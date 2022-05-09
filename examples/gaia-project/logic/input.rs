use bevy::prelude::*;

use crate::types::*;
use crate::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Running).with_system(input_monitor))
            .add_event::<PowerEvent>()
            .add_event::<CoverActionEvent>()
            .add_event::<PaymentEvent>();
    }
}

fn input_monitor(
    input: Res<Input<KeyCode>>,
    mut cover_action_events: EventWriter<CoverActionEvent>,
    mut payment_events: EventWriter<PaymentEvent>,
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
            payment_events.send(PaymentEvent::CoverAction(CoverActionEvent::Cover(
                action_type,
            )));
        }
    }

    if input.just_pressed(KeyCode::Up) {
        payment_events.send(PaymentEvent::Gain(Amount {
            resource: Resource::PowerCharge,
            amount: 1,
        }));
    }

    if input.just_pressed(KeyCode::A) {
        payment_events.send(PaymentEvent::Gain(Amount {
            resource: Resource::PowerTokens,
            amount: 1,
        }));
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
            payment_events.send(PaymentEvent::Gain(Amount {
                resource,
                amount: 1,
            }));
        }
    }
}
