use bevy::prelude::*;

use crate::logic::research::ResearchTracks;
use crate::types::*;

pub struct ResearchViewState {
    pub tracks: ResearchTracks,
}

#[derive(Component)]
pub struct ResearchView;

pub fn research_view(
    mut view_state: ResMut<ResearchViewState>,
    mut events: EventReader<ResearchEvent>,
    mut query: Query<&mut Text, With<ResearchView>>,
) {
    events.iter().for_each(|&event| match event {
        ResearchEvent::Advance(research_type) => {
            view_state.tracks.get_mut(research_type).advance().unwrap();
        }
    });

    query.iter_mut().for_each(|mut text| {
        text.sections[1].value = format!(
            "Terraforming: {}\n",
            view_state.tracks.get(ResearchType::Terraforming).level
        );
        text.sections[2].value = format!(
            "Flight: {}\n",
            view_state.tracks.get(ResearchType::Flight).level
        );
        text.sections[3].value = format!(
            "ArtificialIntelligence: {}\n",
            view_state
                .tracks
                .get(ResearchType::ArtificialIntelligence)
                .level
        );
        text.sections[4].value = format!(
            "Gaiaforming: {}\n",
            view_state.tracks.get(ResearchType::Gaiaforming).level
        );
        text.sections[5].value = format!(
            "Economics: {}\n",
            view_state.tracks.get(ResearchType::Economics).level
        );
        text.sections[6].value = format!(
            "Science: {}\n",
            view_state.tracks.get(ResearchType::Science).level
        );
    });
}
