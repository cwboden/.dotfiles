use crate::types::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Cost {
    resource: Resource,
    amount: u8,
}

#[derive(Debug)]
struct CoverAction {
    action: CoverActionType,
    cost: Cost,
    is_used: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyCovered,
}

pub type Result<T> = std::result::Result<T, Error>;

impl CoverAction {
    pub fn new(action: CoverActionType) -> Self {
        let resource = match action {
            CoverActionType::GainThreePower
            | CoverActionType::SingleTerraform
            | CoverActionType::TwoKnowledge
            | CoverActionType::SevenCredits
            | CoverActionType::TwoOre
            | CoverActionType::DoubleTerraform
            | CoverActionType::ThreeKnowledge => Resource::Power,
            CoverActionType::GainTechTile
            | CoverActionType::RescoreFederationToken
            | CoverActionType::PointsForPlanetTypes => Resource::Qic,
        };
        let amount = match action {
            CoverActionType::GainThreePower | CoverActionType::SingleTerraform => 3,
            CoverActionType::TwoKnowledge
            | CoverActionType::SevenCredits
            | CoverActionType::TwoOre => 4,
            CoverActionType::DoubleTerraform => 5,
            CoverActionType::ThreeKnowledge => 7,
            CoverActionType::PointsForPlanetTypes => 2,
            CoverActionType::RescoreFederationToken => 3,
            CoverActionType::GainTechTile => 4,
        };

        Self {
            action,
            cost: Cost { resource, amount },
            is_used: false,
        }
    }

    pub fn get_cost(&self) -> Cost {
        self.cost
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cover_actions_cost_three_power() {
        for &action in [
            CoverActionType::SingleTerraform,
            CoverActionType::GainThreePower,
        ]
        .iter()
        {
            assert_eq!(
                CoverAction::new(action).get_cost(),
                Cost {
                    resource: Resource::Power,
                    amount: 3,
                }
            );
        }
    }

    #[test]
    fn cover_actions_cost_four_power() {
        for &action in [
            CoverActionType::TwoKnowledge,
            CoverActionType::SevenCredits,
            CoverActionType::TwoOre,
        ]
        .iter()
        {
            assert_eq!(
                CoverAction::new(action).get_cost(),
                Cost {
                    resource: Resource::Power,
                    amount: 4,
                }
            );
        }
    }

    #[test]
    fn cover_action_double_terraform_costs_five_power() {
        assert_eq!(
            CoverAction::new(CoverActionType::DoubleTerraform).get_cost(),
            Cost {
                resource: Resource::Power,
                amount: 5,
            }
        );
    }

    #[test]
    fn cover_action_three_knowledge_costs_seven_power() {
        assert_eq!(
            CoverAction::new(CoverActionType::ThreeKnowledge).get_cost(),
            Cost {
                resource: Resource::Power,
                amount: 7,
            }
        );
    }

    #[test]
    fn cover_action_points_for_planet_types_cost_two_qic() {
        assert_eq!(
            CoverAction::new(CoverActionType::PointsForPlanetTypes).get_cost(),
            Cost {
                resource: Resource::Qic,
                amount: 2,
            }
        );
    }

    #[test]
    fn cover_action_rescore_federation_token_costs_three_qic() {
        assert_eq!(
            CoverAction::new(CoverActionType::RescoreFederationToken).get_cost(),
            Cost {
                resource: Resource::Qic,
                amount: 3,
            }
        );
    }

    #[test]
    fn cover_action_gain_tech_tile_costs_four_qic() {
        assert_eq!(
            CoverAction::new(CoverActionType::GainTechTile).get_cost(),
            Cost {
                resource: Resource::Qic,
                amount: 4,
            }
        );
    }

    #[test]
    fn cover_action_cover() {
        let mut action = CoverAction::new(CoverActionType::TwoOre);
        action.cover().unwrap();
        assert_eq!(action.cover(), Err(Error::AlreadyCovered));
    }

    #[test]
    fn cover_action_uncover() {
        let mut action = CoverAction::new(CoverActionType::TwoOre);
        action.cover().unwrap();
        action.uncover();
        action.cover().unwrap();
    }
}
