use crate::types::PowerBowl;

pub struct PowerCycleTracker {
    bowls: [u8; 4],
}

impl PowerBowl {
    fn index(&self) -> usize {
        match self {
            PowerBowl::One => 0,
            PowerBowl::Two => 1,
            PowerBowl::Three => 2,
            PowerBowl::Gaia => 3,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    NotEnoughPower,
}

pub type Result<T> = std::result::Result<T, Error>;

impl PowerCycleTracker {
    pub fn new(bowl_1: u8, bowl_2: u8, bowl_3: u8, bowl_gaia: u8) -> Self {
        Self {
            bowls: [bowl_1, bowl_2, bowl_3, bowl_gaia],
        }
    }

    fn move_power(&mut self, amount: u8, bowl_from: PowerBowl, bowl_to: PowerBowl) -> u8 {
        let to_move = std::cmp::min(self.bowls[bowl_from.index()], amount);
        self.bowls[bowl_from.index()] -= to_move;
        self.bowls[bowl_to.index()] += to_move;

        amount - to_move
    }

    pub fn spend(&mut self, amount: u8) -> Result<()> {
        if self.bowls[PowerBowl::Three.index()] < amount {
            Err(Error::NotEnoughPower)
        } else {
            self.move_power(amount, PowerBowl::Three, PowerBowl::One);
            Ok(())
        }
    }

    pub fn charge(&mut self, amount: u8) {
        let remaining_amount = self.move_power(amount, PowerBowl::One, PowerBowl::Two);
        self.move_power(remaining_amount, PowerBowl::Two, PowerBowl::Three);
    }

    pub fn get(&self, bowl: PowerBowl) -> u8 {
        self.bowls[bowl.index()]
    }

    pub fn reserve(&mut self, amount: u8) -> Result<()> {
        let remaining_amount = self.move_power(amount, PowerBowl::One, PowerBowl::Gaia);
        let remaining_amount = self.move_power(remaining_amount, PowerBowl::Two, PowerBowl::Gaia);
        let remaining_amount = self.move_power(remaining_amount, PowerBowl::Three, PowerBowl::Gaia);

        if remaining_amount > 0 {
            Err(Error::NotEnoughPower)
        } else {
            Ok(())
        }
    }

    pub fn discard(&mut self, bowl: PowerBowl, amount: u8) -> Result<()> {
        if self.bowls[bowl.index()] < amount {
            Err(Error::NotEnoughPower)
        } else {
            self.bowls[bowl.index()] -= amount;
            Ok(())
        }
    }

    pub fn add(&mut self, amount: u8) {
        self.bowls[PowerBowl::One.index()] += amount;
    }

    pub fn force(&mut self, amount: u8) -> Result<()> {
        self.discard(PowerBowl::Two, amount)?;
        let remaining_amount = self.move_power(amount, PowerBowl::Two, PowerBowl::Three);
        if remaining_amount > 0 {
            Err(Error::NotEnoughPower)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_bowl_contents(
        tracker: &PowerCycleTracker,
        bowl_1: u8,
        bowl_2: u8,
        bowl_3: u8,
        bowl_gaia: u8,
    ) {
        assert_eq!(tracker.get(PowerBowl::One), bowl_1);
        assert_eq!(tracker.get(PowerBowl::Two), bowl_2);
        assert_eq!(tracker.get(PowerBowl::Three), bowl_3);
        assert_eq!(tracker.get(PowerBowl::Gaia), bowl_gaia);
    }

    #[test]
    fn constructor_populates_bowls() {
        let tracker = PowerCycleTracker::new(
            1, // bowl 1
            2, // bowl 2
            3, // bowl 3
            0, // bowl G
        );

        assert_bowl_contents(&tracker, 1, 2, 3, 0);
    }

    #[test]
    fn charge_one_power() {
        let mut tracker = PowerCycleTracker::new(
            1, // bowl 1
            0, // bowl 2
            0, // bowl 3
            0, // bowl G
        );
        assert_eq!(tracker.get(PowerBowl::One), 1);

        tracker.charge(1);
        assert_eq!(tracker.get(PowerBowl::Two), 1);

        tracker.charge(1);
        assert_eq!(tracker.get(PowerBowl::Three), 1);
    }

    #[test]
    fn charge_one_power_prioritized_bowl_one() {
        let mut tracker = PowerCycleTracker::new(
            1, // bowl 1
            1, // bowl 2
            0, // bowl 3
            0, // bowl G
        );

        tracker.charge(1);
        assert_bowl_contents(&tracker, 0, 2, 0, 0);
    }

    #[test]
    fn charge_two_power_moves_to_bowl_three() {
        let mut tracker = PowerCycleTracker::new(
            1, // bowl 1
            0, // bowl 2
            0, // bowl 3
            0, // bowl G
        );

        tracker.charge(2);
        assert_bowl_contents(&tracker, 0, 0, 1, 0);
    }

    #[test]
    fn charge_three_power_moves_to_bowl_three_ignoring_overflow() {
        let mut tracker = PowerCycleTracker::new(
            1, // bowl 1
            0, // bowl 2
            0, // bowl 3
            0, // bowl G
        );

        tracker.charge(3);
        assert_bowl_contents(&tracker, 0, 0, 1, 0);
    }

    #[test]
    fn reserve_one_power_prioritizes_lower_bowls() {
        let mut tracker = PowerCycleTracker::new(
            1, // bowl 1
            1, // bowl 2
            1, // bowl 3
            0, // bowl G
        );

        tracker.reserve(1).unwrap();
        assert_bowl_contents(&tracker, 0, 1, 1, 1);

        tracker.reserve(1).unwrap();
        assert_bowl_contents(&tracker, 0, 0, 1, 2);

        tracker.reserve(1).unwrap();
        assert_bowl_contents(&tracker, 0, 0, 0, 3);
    }

    #[test]
    fn reserve_three_power() {
        let mut tracker = PowerCycleTracker::new(
            1, // bowl 1
            1, // bowl 2
            1, // bowl 3
            0, // bowl G
        );

        tracker.reserve(3).unwrap();
        assert_bowl_contents(&tracker, 0, 0, 0, 3);
    }

    #[test]
    fn reserve_throws_error_if_not_enough_power() {
        let mut tracker = PowerCycleTracker::new(
            1, // bowl 1
            1, // bowl 2
            1, // bowl 3
            0, // bowl G
        );

        assert_eq!(tracker.reserve(4), Err(Error::NotEnoughPower));
    }

    #[test]
    fn spend_one_power() {
        let mut tracker = PowerCycleTracker::new(
            0, // bowl 1
            0, // bowl 2
            1, // bowl 3
            0, // bowl G
        );

        tracker.spend(1).unwrap();
        assert_bowl_contents(&tracker, 1, 0, 0, 0);
    }

    #[test]
    fn spend_two_power_leaves_one_remaining_in_bowl() {
        let mut tracker = PowerCycleTracker::new(
            0, // bowl 1
            0, // bowl 2
            3, // bowl 3
            0, // bowl G
        );

        tracker.spend(2).unwrap();
        assert_bowl_contents(&tracker, 2, 0, 1, 0);
    }

    #[test]
    fn spend_throws_error_if_not_enough_power() {
        let mut tracker = PowerCycleTracker::new(
            0, // bowl 1
            0, // bowl 2
            0, // bowl 3
            0, // bowl G
        );

        assert_eq!(tracker.spend(1), Err(Error::NotEnoughPower));
    }

    #[test]
    fn discard_power_from_each_bowl() {
        let mut tracker = PowerCycleTracker::new(
            1, // bowl 1
            1, // bowl 2
            1, // bowl 3
            1, // bowl G
        );

        tracker.discard(PowerBowl::One, 1).unwrap();
        assert_bowl_contents(&tracker, 0, 1, 1, 1);

        tracker.discard(PowerBowl::Two, 1).unwrap();
        assert_bowl_contents(&tracker, 0, 0, 1, 1);

        tracker.discard(PowerBowl::Three, 1).unwrap();
        assert_bowl_contents(&tracker, 0, 0, 0, 1);

        tracker.discard(PowerBowl::Gaia, 1).unwrap();
        assert_bowl_contents(&tracker, 0, 0, 0, 0);
    }

    #[test]
    fn discard_power_throws_error_if_not_enough_power() {
        let mut tracker = PowerCycleTracker::new(
            1, // bowl 1
            1, // bowl 2
            1, // bowl 3
            1, // bowl G
        );

        for &bowl in [
            PowerBowl::One,
            PowerBowl::Two,
            PowerBowl::Three,
            PowerBowl::Gaia,
        ]
        .iter()
        {
            assert_eq!(tracker.discard(bowl, 2), Err(Error::NotEnoughPower),);
        }
    }

    #[test]
    fn add_power_is_always_added_to_bowl_one() {
        let mut tracker = PowerCycleTracker::new(
            0, // bowl 1
            0, // bowl 2
            0, // bowl 3
            0, // bowl G
        );

        tracker.add(4);
        assert_bowl_contents(&tracker, 4, 0, 0, 0);
    }

    #[test]
    fn force_two_power_from_bowl_two_to_bowl_three() {
        let mut tracker = PowerCycleTracker::new(
            0, // bowl 1
            2, // bowl 2
            0, // bowl 3
            0, // bowl G
        );

        tracker.force(1).unwrap();
        assert_bowl_contents(&tracker, 0, 0, 1, 0);
    }

    #[test]
    fn force_throws_error_if_not_enough_power() {
        let mut tracker = PowerCycleTracker::new(
            0, // bowl 1
            1, // bowl 2
            0, // bowl 3
            0, // bowl G
        );

        assert_eq!(tracker.force(1), Err(Error::NotEnoughPower),);
    }
}
