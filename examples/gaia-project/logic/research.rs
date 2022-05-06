use crate::types::*;

pub struct ResearchTrack {
    level: u8,
    research_type: ResearchType,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ResearchTrackAtMax,
}

pub type Result<T> = std::result::Result<T, Error>;

impl ResearchTrack {
    const MAX: u8 = 6;

    pub fn new(research_type: ResearchType) -> Self {
        Self {
            level: 0,
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

struct ResearchTracks {
    tracks: Vec<ResearchTrack>,
}

impl ResearchTracks {
    pub fn new() -> Self {
        let tracks = [
            ResearchType::Terraforming,
            ResearchType::Flight,
            ResearchType::ArtificialIntelligence,
            ResearchType::Gaiaforming,
            ResearchType::Economics,
            ResearchType::Science,
        ]
        .iter()
        .map(|&t| ResearchTrack::new(t))
        .collect();

        Self { tracks }
    }

    pub fn get_track(&self, research_type: ResearchType) -> &ResearchTrack {
        let index = match research_type {
            ResearchType::Terraforming => 0,
            ResearchType::Flight => 1,
            ResearchType::ArtificialIntelligence => 2,
            ResearchType::Gaiaforming => 3,
            ResearchType::Economics => 4,
            ResearchType::Science => 5,
        };

        &self.tracks[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_track_advance() {
        let mut track = ResearchTrack::new(ResearchType::Science);
        track.advance().unwrap();
        assert_eq!(track.level, 1);
    }

    #[test]
    fn research_track_advance_errors_at_max() {
        let mut track = ResearchTrack::new(ResearchType::Science);

        for _ in 0..ResearchTrack::MAX {
            track.advance().unwrap();
        }

        assert_eq!(track.advance(), Err(Error::ResearchTrackAtMax));
    }

    #[test]
    fn research_tracks_get() {
        let tracks = ResearchTracks::new();

        [
            ResearchType::Terraforming,
            ResearchType::Flight,
            ResearchType::ArtificialIntelligence,
            ResearchType::Gaiaforming,
            ResearchType::Economics,
            ResearchType::Science,
        ]
        .iter()
        .for_each(|&t| assert_eq!(tracks.get_track(t).research_type, t));
    }
}
