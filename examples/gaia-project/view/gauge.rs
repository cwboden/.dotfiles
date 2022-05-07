use bevy::prelude::*;

use crate::logic::gauge::Gauges;
use crate::types::*;

pub struct GaugeViewState {
    pub gauges: Gauges,
}

#[derive(Component)]
pub struct GaugeView;

pub fn gauge_view(
    mut view_state: ResMut<GaugeViewState>,
    mut events: EventReader<GaugeEvent>,
    mut query: Query<&mut Text, With<GaugeView>>,
) {
    for &event in events.iter() {
        match event {
            GaugeEvent::Gain(cost) => {
                view_state.gauges.get_mut(cost.resource).add(cost.amount);
            }
            GaugeEvent::Spend(cost) => {
                view_state.gauges.get_mut(cost.resource).sub(cost.amount);
            }
        }
    }

    for mut text in query.iter_mut() {
        text.sections[1].value = format!("Ore: {}\n", view_state.gauges.get(Resource::Ore).get());
        text.sections[2].value = format!(
            "Knowledge: {}\n",
            view_state.gauges.get(Resource::Knowledge).get()
        );
        text.sections[3].value = format!(
            "Credits: {}\n",
            view_state.gauges.get(Resource::Credit).get()
        );
        text.sections[4].value = format!("QIC: {}\n", view_state.gauges.get(Resource::Qic).get());
    }
}
