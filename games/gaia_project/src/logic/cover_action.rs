use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::types::*;

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum Type {
    // Power Actions
    GainThreePower,
    SingleTerraform,
    TwoKnowledge,
    SevenCredits,
    TwoOre,
    DoubleTerraform,
    ThreeKnowledge,

    // QIC Actions
    PointsForPlanetTypes,
    RescoreFederationToken,
    GainTechTile,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CoverAction {
    cost: Amount,
    pub is_used: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyCovered,
}

pub type Result<T> = std::result::Result<T, Error>;

impl CoverAction {
    pub fn new(action: Type) -> Self {
        let resource = match action {
            Type::GainThreePower
            | Type::SingleTerraform
            | Type::TwoKnowledge
            | Type::SevenCredits
            | Type::TwoOre
            | Type::DoubleTerraform
            | Type::ThreeKnowledge => Resource::PowerCharge,
            Type::GainTechTile | Type::RescoreFederationToken | Type::PointsForPlanetTypes => {
                Resource::Qic
            }
        };
        let amount = match action {
            Type::GainThreePower | Type::SingleTerraform => 3,
            Type::TwoKnowledge | Type::SevenCredits | Type::TwoOre => 4,
            Type::DoubleTerraform => 5,
            Type::ThreeKnowledge => 7,
            Type::PointsForPlanetTypes => 2,
            Type::RescoreFederationToken => 3,
            Type::GainTechTile => 4,
        };

        Self {
            cost: Amount::new_singular(resource, amount),
            is_used: false,
        }
    }

    pub fn get_cost(&self) -> Amount {
        self.cost.clone()
    }

    pub fn cover(&mut self) -> Result<()> {
        if self.is_used {
            Err(Error::AlreadyCovered)
        } else {
            self.is_used = true;
            Ok(())
        }
    }

    pub fn uncover(&mut self) {
        self.is_used = false;
    }
}

pub struct CoverActions {
    actions: Vec<CoverAction>,
}

impl CoverActions {
    pub fn new() -> Self {
        Self {
            actions: Type::iter().map(CoverAction::new).collect(),
        }
    }

    fn map_index(action: Type) -> usize {
        match action {
            Type::GainThreePower => 0,
            Type::SingleTerraform => 1,
            Type::TwoKnowledge => 2,
            Type::SevenCredits => 3,
            Type::TwoOre => 4,
            Type::DoubleTerraform => 5,
            Type::ThreeKnowledge => 6,
            Type::PointsForPlanetTypes => 7,
            Type::RescoreFederationToken => 8,
            Type::GainTechTile => 9,
        }
    }

    pub fn get(&self, action: Type) -> &CoverAction {
        let index = Self::map_index(action);
        &self.actions[index]
    }

    pub fn get_mut(&mut self, action: Type) -> &mut CoverAction {
        let index = Self::map_index(action);
        &mut self.actions[index]
    }

    pub fn reset(&mut self) {
        self.actions.iter_mut().for_each(|a| a.uncover());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cover_actions_cost_three_power() {
        for &action in [Type::SingleTerraform, Type::GainThreePower].iter() {
            assert_eq!(
                CoverAction::new(action).get_cost(),
                Amount::new_singular(Resource::PowerCharge, 3)
            );
        }
    }

    #[test]
    fn cover_actions_cost_four_power() {
        for &action in [Type::TwoKnowledge, Type::SevenCredits, Type::TwoOre].iter() {
            assert_eq!(
                CoverAction::new(action).get_cost(),
                Amount::new_singular(Resource::PowerCharge, 4)
            );
        }
    }

    #[test]
    fn cover_action_double_terraform_costs_five_power() {
        assert_eq!(
            CoverAction::new(Type::DoubleTerraform).get_cost(),
            Amount::new_singular(Resource::PowerCharge, 5)
        );
    }

    #[test]
    fn cover_action_three_knowledge_costs_seven_power() {
        assert_eq!(
            CoverAction::new(Type::ThreeKnowledge).get_cost(),
            Amount::new_singular(Resource::PowerCharge, 7)
        );
    }

    #[test]
    fn cover_action_points_for_planet_types_cost_two_qic() {
        assert_eq!(
            CoverAction::new(Type::PointsForPlanetTypes).get_cost(),
            Amount::new_singular(Resource::Qic, 2)
        );
    }

    #[test]
    fn cover_action_rescore_federation_token_costs_three_qic() {
        assert_eq!(
            CoverAction::new(Type::RescoreFederationToken).get_cost(),
            Amount::new_singular(Resource::Qic, 3)
        );
    }

    #[test]
    fn cover_action_gain_tech_tile_costs_four_qic() {
        assert_eq!(
            CoverAction::new(Type::GainTechTile).get_cost(),
            Amount::new_singular(Resource::Qic, 4)
        );
    }

    #[test]
    fn cover_action_cover() {
        let mut action = CoverAction::new(Type::TwoOre);
        action.cover().unwrap();
        assert_eq!(action.cover(), Err(Error::AlreadyCovered));
    }

    #[test]
    fn cover_action_uncover() {
        let mut action = CoverAction::new(Type::TwoOre);
        action.cover().unwrap();
        action.uncover();
        action.cover().unwrap();
    }

    #[test]
    fn cover_actions_all_actions_covered() {
        let actions = CoverActions::new();

        Type::iter().for_each(|t| assert_eq!(actions.get(t), &CoverAction::new(t)));
    }

    #[test]
    fn cover_actions_reset() {
        let mut actions = CoverActions::new();

        Type::iter().for_each(|t| actions.get_mut(t).cover().unwrap());

        actions.reset();

        // Should be able to cover each action again
        Type::iter().for_each(|t| actions.get_mut(t).cover().unwrap());
    }
}
