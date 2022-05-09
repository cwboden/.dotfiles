use bevy::prelude::*;

use crate::logic::gauge::{self, Gauge};
use crate::logic::power::{self, PowerCycleTracker};
use crate::types::*;
use crate::view::cover_action::CoverActionViewState;

pub struct ResourcesState {
    pub ore: Gauge<Resource>,
    pub knowledge: Gauge<Resource>,
    pub credits: Gauge<Resource>,
    pub qic: Gauge<Resource>,
    pub power: PowerCycleTracker,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    NotEnoughResources,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<gauge::Error> for Error {
    fn from(other: gauge::Error) -> Self {
        match other {
            gauge::Error::NotEnoughResources => Self::NotEnoughResources,
        }
    }
}

impl From<power::Error> for Error {
    fn from(other: power::Error) -> Self {
        match other {
            power::Error::NotEnoughPower => Self::NotEnoughResources,
        }
    }
}

impl ResourcesState {
    pub fn new() -> Self {
        Self {
            ore: Gauge::new(Resource::Ore),
            knowledge: Gauge::new(Resource::Knowledge),
            credits: Gauge::new(Resource::Credit),
            qic: Gauge::new(Resource::Qic),
            power: PowerCycleTracker::new(2, 4, 0, 0),
        }
    }

    pub fn gain(&mut self, amount: Amount) {
        match amount.resource {
            Resource::Ore => {
                self.ore.add(amount.amount);
            }
            Resource::Knowledge => {
                self.knowledge.add(amount.amount);
            }
            Resource::Credit => {
                self.credits.add(amount.amount);
            }
            Resource::Qic => {
                self.qic.add(amount.amount);
            }
            Resource::PowerCharge => {
                self.power.charge(amount.amount);
            }
            Resource::PowerTokens => {
                self.power.add(amount.amount);
            }
        }
    }

    pub fn spend(&mut self, cost: Amount) -> Result<()> {
        match cost.resource {
            Resource::Ore => Ok(self.ore.try_sub(cost.amount)?),
            Resource::Knowledge => Ok(self.knowledge.try_sub(cost.amount)?),
            Resource::Credit => Ok(self.credits.try_sub(cost.amount)?),
            Resource::Qic => Ok(self.qic.try_sub(cost.amount)?),
            Resource::PowerCharge => Ok(self.power.spend(cost.amount)?),
            Resource::PowerTokens => {
                let mut amount = cost.amount;
                for &bowl in [
                    // XXX: User should be able to choose how power is discarded
                    PowerBowl::Gaia,
                    PowerBowl::One,
                    PowerBowl::Two,
                    PowerBowl::Three,
                ]
                .iter()
                {
                    let bowl_amount = self.power.get(bowl);
                    let discard_amount = std::cmp::min(bowl_amount, amount);
                    self.power.discard(bowl, discard_amount).unwrap();

                    amount -= discard_amount;
                }

                // We should have discarded all input power
                assert_eq!(amount, 0);
                Ok(())
            }
        }
    }
}

pub fn payment_system(
    mut payment_events: EventReader<PaymentEvent>,
    mut resources_state: ResMut<ResourcesState>,
    cover_action_state: Res<CoverActionViewState>,
    mut cover_action_events: EventWriter<CoverActionEvent>,
    mut research_events: EventWriter<ResearchEvent>,
) {
    payment_events.iter().for_each(|&event| match event {
        PaymentEvent::Gain(amount) => resources_state.gain(amount),
        PaymentEvent::CoverAction(cover_event_type) => {
            match cover_event_type {
                CoverActionEvent::Cover(action_type) => {
                    let cost = cover_action_state.actions.get(action_type).get_cost();
                    resources_state.spend(cost).unwrap();
                }
                CoverActionEvent::Reset => (),
            }

            cover_action_events.send(cover_event_type);
        }
        PaymentEvent::Research(research_type) => {
            resources_state
                .spend(Amount::new(Resource::Knowledge, 4))
                .unwrap();
            research_events.send(ResearchEvent::Advance(research_type));
        }
    });
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn resources_state_gain_and_spend() {
        let mut state = ResourcesState::new();
        Resource::iter().for_each(|r| {
            state.gain(Amount::new(r, 3));
            state.spend(Amount::new(r, 1)).unwrap();
        });
    }

    #[test]
    fn resources_state_spend_errors_when_not_enough_resources() {
        let mut state = ResourcesState::new();
        assert_eq!(
            state.spend(Amount::new(Resource::Ore, 1)),
            Err(Error::NotEnoughResources)
        );
    }
}
