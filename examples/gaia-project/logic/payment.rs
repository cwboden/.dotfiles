use bevy::prelude::*;

use crate::types::*;
use crate::view::cover_action::CoverActionViewState;
use crate::view::gauge::GaugeViewState;
use crate::view::power::PowerViewState;

pub fn payment_system(
    mut payment_events: EventReader<PaymentEvent>,
    action_view_state: Res<CoverActionViewState>,
    power_view_state: Res<PowerViewState>,
    gauge_view_state: Res<GaugeViewState>,
    mut action_events: EventWriter<CoverActionEvent>,
    mut power_events: EventWriter<PowerEvent>,
    mut gauge_events: EventWriter<GaugeEvent>,
) {
    for &event in payment_events.iter() {
        match event {
            PaymentEvent::CoverAction(action) => {
                match action {
                    CoverActionEvent::Cover(action) => {
                        let cost = action_view_state.actions.get(action).get_cost();
                        match cost.resource {
                            Resource::Power => {
                                let current_amount = power_view_state.tracker.get(PowerBowl::Three);
                                if current_amount < cost.amount {
                                    print!(
                                        "Not enough Power to complete action:\n\tHave: {}\n\tNeed: {}\n",
                                        current_amount,
                                        cost.amount
                                    );

                                    // XXX: Should add better error handling than just returning.
                                    return;
                                }

                                power_events.send(PowerEvent::Spend(cost.amount));
                            }
                            Resource::Qic => {
                                let current_amount =
                                    gauge_view_state.gauges.get(Resource::Qic).get();
                                if current_amount < cost.amount {
                                    print!(
                                        "Not enough QIC to complete action:\n\tHave: {}\n\tNeed: {}\n",
                                        current_amount,
                                        cost.amount
                                    );

                                    // XXX: Should add better error handling than just returning.
                                    return;
                                }

                                gauge_events.send(GaugeEvent::Spend(cost));
                            }
                            _ => {
                                panic!(
                                    "PaymentEvent not supported for Resource: {:?}",
                                    cost.resource
                                )
                            }
                        }
                    }
                    CoverActionEvent::Reset => (),
                }

                action_events.send(action);
            }
        }
    }
}
