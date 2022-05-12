use crate::logic::terraforming::TerraformingCost;
use crate::types::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Terraforming,
    Flight,
    ArtificialIntelligence,
    Gaiaforming,
    Economics,
    Science,
}

pub struct ResearchTrack {
    pub level: u8,
    research_type: Type,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ResearchTrackAtMax,
}

pub type Result<T> = std::result::Result<T, Error>;

impl ResearchTrack {
    const MAX: u8 = 6;

    pub fn new(research_type: Type) -> Self {
        Self {
            level: 1,
            research_type,
        }
    }

    pub fn advance(&mut self) -> Result<()> {
        if self.level == Self::MAX {
            Err(Error::ResearchTrackAtMax)
        } else {
            self.level += 1;
            Ok(())
        }
    }
}

pub struct ResearchTracks {
    tracks: Vec<ResearchTrack>,
}

impl ResearchTracks {
    pub fn new() -> Self {
        let tracks = [
            Type::Terraforming,
            Type::Flight,
            Type::ArtificialIntelligence,
            Type::Gaiaforming,
            Type::Economics,
            Type::Science,
        ]
        .iter()
        .map(|&t| ResearchTrack::new(t))
        .collect();

        Self { tracks }
    }

    fn map_index(research_type: Type) -> usize {
        match research_type {
            Type::Terraforming => 0,
            Type::Flight => 1,
            Type::ArtificialIntelligence => 2,
            Type::Gaiaforming => 3,
            Type::Economics => 4,
            Type::Science => 5,
        }
    }

    pub fn get(&self, research_type: Type) -> &ResearchTrack {
        let index = Self::map_index(research_type);
        &self.tracks[index]
    }

    pub fn get_mut(&mut self, research_type: Type) -> &mut ResearchTrack {
        let index = Self::map_index(research_type);
        &mut self.tracks[index]
    }
}

impl TerraformingCost for ResearchTracks {
    fn terraforming_cost(&self) -> Amount {
        let cost = match self.get(Type::Terraforming).level {
            1 => 3,
            2 => 3,
            3 => 2,
            4 => 2,
            5 => 1,
            6 => 1,
            _ => panic!("Invalid track value"),
        };

        Amount::new(Resource::Ore, cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_track_advance() {
        let mut track = ResearchTrack::new(Type::Science);
        track.advance().unwrap();
        assert_eq!(track.level, 2);
    }

    #[test]
    fn research_track_advance_errors_at_max() {
        let mut track = ResearchTrack::new(Type::Science);

        for _ in 1..ResearchTrack::MAX {
            track.advance().unwrap();
        }

        assert_eq!(track.advance(), Err(Error::ResearchTrackAtMax));
    }

    #[test]
    fn research_tracks_get() {
        let tracks = ResearchTracks::new();

        [
            Type::Terraforming,
            Type::Flight,
            Type::ArtificialIntelligence,
            Type::Gaiaforming,
            Type::Economics,
            Type::Science,
        ]
        .iter()
        .for_each(|&t| assert_eq!(tracks.get(t).research_type, t));
    }
}
