use bevy::prelude::*;

use crate::types::*;

pub struct FederationTokens {
    stacks: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    NoTokensRemaining,
}

pub type Result<T> = std::result::Result<T, Error>;

impl FederationTokens {
    pub fn new() -> Self {
        Self { stacks: vec![3; 6] }
    }

    pub fn take(&mut self, token_type: FederationToken) -> Result<Amount> {
        let index = match token_type {
            FederationToken::EightPointsQic => 0,
            FederationToken::EightPointsTwoPower => 1,
            FederationToken::SevenPointsSixCredits => 2,
            FederationToken::SevenPointsTwoOre => 3,
            FederationToken::SixPointsTwoKnowledge => 4,
            FederationToken::TwelvePoints => 5,
            FederationToken::OneOreOneKnowledgeTwoCredits => {
                panic!("No FederationToken stack for {token_type:?}")
            }
        };

        if self.stacks[index] == 0 {
            Err(Error::NoTokensRemaining)
        } else {
            self.stacks[index] -= 1;
            Ok(token_type.into())
        }
    }
}

pub fn federation_token_system(
    mut federation_events: EventReader<FederationTokenEvent>,
    mut state: ResMut<FederationTokens>,
    mut payment_events: EventWriter<PaymentEvent>,
) {
    federation_events.iter().for_each(|&event| match event {
        FederationTokenEvent::Take(token_type) => {
            let amount = state.take(token_type).unwrap();
            payment_events.send(PaymentEvent::Gain(amount));
        }
    });
}
