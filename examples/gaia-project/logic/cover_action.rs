use crate::types::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Debug)]
struct CoverAction {
    action: Type,
    cost: Cost,
    is_used: bool,
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
            | Type::ThreeKnowledge => Resource::Power,
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
        for &action in [Type::SingleTerraform, Type::GainThreePower].iter() {
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
        for &action in [Type::TwoKnowledge, Type::SevenCredits, Type::TwoOre].iter() {
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
            CoverAction::new(Type::DoubleTerraform).get_cost(),
            Cost {
                resource: Resource::Power,
                amount: 5,
            }
        );
    }

    #[test]
    fn cover_action_three_knowledge_costs_seven_power() {
        assert_eq!(
            CoverAction::new(Type::ThreeKnowledge).get_cost(),
            Cost {
                resource: Resource::Power,
                amount: 7,
            }
        );
    }

    #[test]
    fn cover_action_points_for_planet_types_cost_two_qic() {
        assert_eq!(
            CoverAction::new(Type::PointsForPlanetTypes).get_cost(),
            Cost {
                resource: Resource::Qic,
                amount: 2,
            }
        );
    }

    #[test]
    fn cover_action_rescore_federation_token_costs_three_qic() {
        assert_eq!(
            CoverAction::new(Type::RescoreFederationToken).get_cost(),
            Cost {
                resource: Resource::Qic,
                amount: 3,
            }
        );
    }

    #[test]
    fn cover_action_gain_tech_tile_costs_four_qic() {
        assert_eq!(
            CoverAction::new(Type::GainTechTile).get_cost(),
            Cost {
                resource: Resource::Qic,
                amount: 4,
            }
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
}
