use bevy::prelude::*;

use crate::logic::gauge::{self, Gauge};
use crate::logic::power::{self, PowerCycleTracker};
use crate::logic::terraforming::TerraformingCost;
use crate::types::*;
use crate::view::cover_action::CoverActionViewState;
use crate::view::research::ResearchViewState;

pub struct ResourcesState {
    pub ore: Gauge,
    pub knowledge: Gauge,
    pub credits: Gauge,
    pub qic: Gauge,
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
        self.ore.add(amount.get(Resource::Ore));
        self.knowledge.add(amount.get(Resource::Knowledge));
        self.credits.add(amount.get(Resource::Credit));
        self.qic.add(amount.get(Resource::Qic));
        self.power.charge(amount.get(Resource::PowerCharge));
        self.power.add(amount.get(Resource::PowerTokens));
    }

    pub fn spend(&mut self, amount: Amount) -> Result<()> {
        self.ore.try_sub(amount.get(Resource::Ore))?;
        self.knowledge.try_sub(amount.get(Resource::Knowledge))?;
        self.credits.try_sub(amount.get(Resource::Credit))?;
        self.qic.try_sub(amount.get(Resource::Qic))?;
        self.power.spend(amount.get(Resource::PowerCharge))?;

        let mut num_discard = amount.get(Resource::PowerTokens);
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
            let discard_amount = std::cmp::min(bowl_amount, num_discard);
            self.power.discard(bowl, discard_amount).unwrap();

            num_discard -= discard_amount;
        }
        // We should have discarded all input power
        assert_eq!(num_discard, 0);

        Ok(())
    }
}

pub fn payment_system(
    mut payment_events: EventReader<PaymentEvent>,
    mut resources_state: ResMut<ResourcesState>,
    cover_action_state: Res<CoverActionViewState>,
    research_state: Res<ResearchViewState>,
    mut cover_action_events: EventWriter<CoverActionEvent>,
    mut research_events: EventWriter<ResearchEvent>,
) {
    payment_events.iter().for_each(|event| match event {
        PaymentEvent::Gain(amount) => resources_state.gain(amount.clone()),
        PaymentEvent::CoverAction(cover_event_type) => {
            match cover_event_type {
                CoverActionEvent::Cover(action_type) => {
                    let cost = cover_action_state.actions.get(*action_type).get_cost();
                    resources_state.spend(cost).unwrap();
                }
                CoverActionEvent::Reset => (),
            }

            cover_action_events.send(*cover_event_type);
        }
        PaymentEvent::Research(research_type) => {
            resources_state
                .spend(Amount::new_singular(Resource::Knowledge, 4))
                .unwrap();
            research_events.send(ResearchEvent::Advance(*research_type));
        }
        PaymentEvent::Terraform((from, to)) => {
            let steps = from.terraforms_from(*to);
            let mut cost = research_state.tracks.terraforming_cost();
            cost.multiply(steps);

            resources_state.spend(cost).unwrap();
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
            state.gain(Amount::new_singular(r, 3));
            state.spend(Amount::new_singular(r, 1)).unwrap();
        });
    }

    #[test]
    fn resources_state_spend_errors_when_not_enough_resources() {
        let mut state = ResourcesState::new();
        assert_eq!(
            state.spend(Amount::new_singular(Resource::Ore, 1)),
            Err(Error::NotEnoughResources)
        );
    }
}
