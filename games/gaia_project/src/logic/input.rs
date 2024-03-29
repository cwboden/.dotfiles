use bevy::prelude::*;

use crate::types::*;
use crate::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Running).with_system(input_monitor))
            .add_event::<CoverActionEvent>()
            .add_event::<PaymentEvent>()
            .add_event::<ResearchEvent>()
            .add_event::<FederationTokenEvent>();
    }
}

fn input_monitor(
    input: Res<Input<KeyCode>>,
    mut cover_action_events: EventWriter<CoverActionEvent>,
    mut payment_events: EventWriter<PaymentEvent>,
    mut federation_events: EventWriter<FederationTokenEvent>,
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
        payment_events.send(PaymentEvent::Gain(Amount::new_singular(
            Resource::PowerCharge,
            1,
        )));
    }

    if input.just_pressed(KeyCode::A) {
        payment_events.send(PaymentEvent::Gain(Amount::new_singular(
            Resource::PowerTokens,
            1,
        )));
    }

    if input.just_pressed(KeyCode::Z) {
        cover_action_events.send(CoverActionEvent::Reset);
    }

    for &(key, resource) in [
        (KeyCode::O, Resource::Ore),
        (KeyCode::K, Resource::Knowledge),
        (KeyCode::C, Resource::Credits),
        (KeyCode::Q, Resource::Qic),
    ]
    .iter()
    {
        if input.just_pressed(key) {
            payment_events.send(PaymentEvent::Gain(Amount::new_singular(resource, 1)));
        }
    }

    for &(key, research_type) in [
        (KeyCode::B, ResearchType::Terraforming),
        (KeyCode::N, ResearchType::Flight),
        (KeyCode::M, ResearchType::ArtificialIntelligence),
        (KeyCode::Comma, ResearchType::Gaiaforming),
        (KeyCode::Period, ResearchType::Economics),
        (KeyCode::Slash, ResearchType::Science),
    ]
    .iter()
    {
        if input.just_pressed(key) {
            payment_events.send(PaymentEvent::Research(research_type));
        }
    }

    for &(key, token_type) in [
        (KeyCode::H, FederationToken::EightPointsQic),
        (KeyCode::J, FederationToken::EightPointsTwoPower),
        (KeyCode::K, FederationToken::SevenPointsSixCredits),
        (KeyCode::L, FederationToken::SevenPointsTwoOre),
        (KeyCode::Semicolon, FederationToken::SixPointsTwoKnowledge),
        (KeyCode::Apostrophe, FederationToken::TwelvePoints),
    ]
    .iter()
    {
        if input.just_pressed(key) {
            federation_events.send(FederationTokenEvent::Take(token_type));
        }
    }
}
