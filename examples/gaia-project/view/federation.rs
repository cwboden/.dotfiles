use bevy::prelude::*;

use crate::logic::federation::FederationTokens;
use crate::types::*;

#[derive(Component)]
pub struct FederationTokenView;

pub fn federation_token_view(
    state: Res<FederationTokens>,
    mut query: Query<&mut Text, With<FederationTokenView>>,
) {
    query.iter_mut().for_each(|mut text| {
        text.sections[1].value = format!(
            "8VP + QIC: {}\n",
            state.get(FederationToken::EightPointsQic)
        );
        text.sections[2].value = format!(
            "8VP + 2P: {}\n",
            state.get(FederationToken::EightPointsTwoPower)
        );
        text.sections[3].value = format!(
            "7VP + 6C: {}\n",
            state.get(FederationToken::SevenPointsSixCredits)
        );
        text.sections[4].value = format!(
            "7VP + 2O: {}\n",
            state.get(FederationToken::SevenPointsTwoOre)
        );
        text.sections[5].value = format!(
            "6VP + 2K: {}\n",
            state.get(FederationToken::SixPointsTwoKnowledge)
        );
        text.sections[6].value = format!("12VP: {}\n", state.get(FederationToken::TwelvePoints));
    });
}
