use rand::Rng;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttackFace {
    OutlinedSword,
    FilledSword,
    TwoSwordsAndSkull,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DefenceFace {
    Blank,
    OneShield,
    TwoShields,
    Multiplier2x,
}

pub struct Dice<T> {
    faces: Vec<T>,
}

impl<T> Dice<T> {
    pub fn roll(&self) -> &T {
        let index = rand::thread_rng().gen_range(0..self.faces.len());
        &self.faces[index]
    }
}

impl Default for Dice<AttackFace> {
    fn default() -> Self {
        Self {
            faces: vec![
                AttackFace::OutlinedSword,
                AttackFace::OutlinedSword,
                AttackFace::FilledSword,
                AttackFace::FilledSword,
                AttackFace::TwoSwordsAndSkull,
                AttackFace::TwoSwordsAndSkull,
            ],
        }
    }
}

impl Default for Dice<DefenceFace> {
    fn default() -> Self {
        Self {
            faces: vec![
                DefenceFace::Blank,
                DefenceFace::Blank,
                DefenceFace::OneShield,
                DefenceFace::OneShield,
                DefenceFace::TwoShields,
                DefenceFace::Multiplier2x,
            ],
        }
    }
}

pub struct DiceBuilder<T> {
    faces: Vec<T>,
}

impl<T: Clone> DiceBuilder<T> {
    pub fn new() -> Self {
        Self { faces: Vec::new() }
    }

    #[must_use]
    pub fn with_face(mut self, face: T) -> Self {
        self.faces.push(face);
        self
    }

    #[must_use]
    pub fn with_faces(mut self, faces: &[T]) -> Self {
        self.faces.extend_from_slice(faces);
        self
    }

    pub fn build(self) -> Dice<T> {
        Dice { faces: self.faces }
    }
}

impl<T: Clone> Default for DiceBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dice_builder_can_create_one_sided_dice() {
        let dice = DiceBuilder::new()
            .with_face(AttackFace::TwoSwordsAndSkull)
            .build();

        assert_eq!(dice.roll(), &AttackFace::TwoSwordsAndSkull);
    }

    #[test]
    fn dice_builder_can_create_6_sided_dice() {
        let faces = vec![
            DefenceFace::Blank,
            DefenceFace::Blank,
            DefenceFace::OneShield,
            DefenceFace::OneShield,
            DefenceFace::TwoShields,
            DefenceFace::Multiplier2x,
        ];

        let dice = DiceBuilder::new().with_faces(&faces).build();

        // XXX: Probably a better way to write this, lol
        for face in faces {
            loop {
                if dice.roll() == &face {
                    break;
                }
            }
        }
    }

    #[test]
    fn dice_has_default_for_attack_and_defence() {
        let _attack_dice = Dice::<AttackFace>::default();
        let _defence_dice = Dice::<DefenceFace>::default();
    }
}
