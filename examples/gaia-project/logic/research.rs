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
    level: u8,
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

    pub fn get_track(&self, research_type: Type) -> &ResearchTrack {
        let index = match research_type {
            Type::Terraforming => 0,
            Type::Flight => 1,
            Type::ArtificialIntelligence => 2,
            Type::Gaiaforming => 3,
            Type::Economics => 4,
            Type::Science => 5,
        };

        &self.tracks[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_track_advance() {
        let mut track = ResearchTrack::new(Type::Science);
        track.advance().unwrap();
        assert_eq!(track.level, 1);
    }

    #[test]
    fn research_track_advance_errors_at_max() {
        let mut track = ResearchTrack::new(Type::Science);

        for _ in 0..ResearchTrack::MAX {
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
        .for_each(|&t| assert_eq!(tracks.get_track(t).research_type, t));
    }
}
